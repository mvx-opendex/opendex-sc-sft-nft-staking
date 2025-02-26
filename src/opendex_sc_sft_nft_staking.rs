#![no_std]

use multiversx_sc::contract_base::ManagedSerializer;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

static ISSUE_NFT_COLLECTION_FEE: u64 = 50000000000000000u64;

static REWARD_RATE_PRECISION: u64 = 10_000_000_000u64;

static SAFETY_CONSTANT: u64 = 1_000_000_000_000_000_000u64;

#[type_abi]
#[derive(TopEncode, TopDecode)]
pub struct StakeInfo<M: ManagedTypeApi> {
    token_nonce: u64,                       // SFT nonce staked
    amount: BigUint<M>,                     // Amount of SFTs staked
    stake_timestamp: u64,                   // When staked
    last_claim_timestamp: u64,              // Last reward claim
    user_reward_per_token_paid: BigUint<M>, // Reward per token when last user action occurred
    rewards: BigUint<M>,                    // Earned rewards when last user action occurred
}

/// A staking contract for SFTs that issues NFTs as staking positions.
/// Users stake SFTs, receive NFTs, and earn rewards over time.
#[multiversx_sc::contract]
pub trait OpendexSftNftStaking: multiversx_sc_modules::only_admin::OnlyAdminModule {
    #[init]
    fn init(
        &self,
        staking_sft_collection_id: TokenIdentifier,
        reward_token: EgldOrEsdtTokenIdentifier,
        fee_receiver: ManagedAddress,
        performance_fee: u32,
        funder: ManagedAddress,
    ) {
        let caller = self.blockchain().get_caller();
        self.add_admin(caller);

        self.staking_sft_collection_id()
            .set(&staking_sft_collection_id);
        self.reward_token().set(&reward_token);
        self.fee_receiver().set(&fee_receiver);
        self.performance_fee_percent().set(performance_fee);
        self.total_staked().set(&BigUint::zero());
        self.reward_start_time().set(0);
        self.reward_end_time().set(0);
        self.funder().set(&funder);
        self.last_update_time().set(0);
        self.reward_per_token_stored().set(BigUint::zero());
    }

    // STORAGE

    #[view(getStakingSftCollectionId)]
    #[storage_mapper("staking_sft_collection_id")]
    fn staking_sft_collection_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getRewardToken)]
    #[storage_mapper("reward_token")]
    fn reward_token(&self) -> SingleValueMapper<EgldOrEsdtTokenIdentifier>;

    #[view(getStakedNftCollectionId)]
    #[storage_mapper("staked_nft_collection_id")]
    fn staked_nft_collection_id(&self) -> NonFungibleTokenMapper<Self::Api>;

    /// Return amount of reward per second (multiplied by 10^10 for precision)
    #[view(getRewardPerSecond)]
    #[storage_mapper("reward_per_second")]
    fn reward_per_second(&self) -> SingleValueMapper<BigUint>;

    #[view(getTotalStaked)]
    #[storage_mapper("total_staked")]
    fn total_staked(&self) -> SingleValueMapper<BigUint>;

    #[view(getFeeReceiver)]
    #[storage_mapper("fee_receiver")]
    fn fee_receiver(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getPerformanceFeePercent)]
    #[storage_mapper("performance_fee_percent")]
    fn performance_fee_percent(&self) -> SingleValueMapper<u32>;

    #[view(getRewardPeriodEnd)]
    #[storage_mapper("reward_end_time")]
    fn reward_end_time(&self) -> SingleValueMapper<u64>;

    #[view(getRewardStartTime)]
    #[storage_mapper("reward_start_time")]
    fn reward_start_time(&self) -> SingleValueMapper<u64>;

    #[view(getFunder)]
    #[storage_mapper("funder")]
    fn funder(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getLastUpdateTime)]
    #[storage_mapper("last_update_time")]
    fn last_update_time(&self) -> SingleValueMapper<u64>;

    #[view(getRewardPerTokenStored)]
    #[storage_mapper("reward_per_token_stored")]
    fn reward_per_token_stored(&self) -> SingleValueMapper<BigUint>;

    // User endpoints

    /// Stake SFT/NFTs
    /// Payment: 1 single payment accepted.
    #[endpoint]
    #[payable]
    fn stake(&self) {
        self.staked_nft_collection_id().require_issued_or_set();

        self.update_reward_per_token();

        let payment = self.call_value().single_esdt();
        let caller = self.blockchain().get_caller();
        let current_time = self.blockchain().get_block_timestamp();

        require!(
            payment.token_identifier == self.staking_sft_collection_id().get(),
            "Invalid SFT token"
        );
        require!(
            payment.amount > BigUint::zero(),
            "Must stake at least 1 SFT"
        );

        let nft_amount = BigUint::from(1u32);
        let stake_info = StakeInfo {
            token_nonce: payment.token_nonce,
            amount: payment.amount.clone(),
            stake_timestamp: current_time,
            last_claim_timestamp: current_time,
            user_reward_per_token_paid: self.reward_per_token_stored().get(),
            rewards: BigUint::zero(),
        };

        let staked_nft = self
            .staked_nft_collection_id()
            .nft_create(nft_amount.clone(), &stake_info);

        // Send NFT to user
        self.send()
            .direct_non_zero_esdt_payment(&caller, &staked_nft);

        // Update total staked
        self.total_staked()
            .update(|total| *total += &payment.amount);
    }

    #[endpoint]
    #[payable("*")]
    fn unstake(&self) {
        self.update_reward_per_token();

        let payment = self.call_value().single_esdt();
        let caller = self.blockchain().get_caller();
        let position_nonce = payment.token_nonce;

        // Verify the NFT
        self.require_valid_staked_nft_payment(payment.clone());

        // Get stake info from NFT attributes
        let stake_info = self.decode_nft_attributes(position_nonce);

        // Claim rewards
        self.claim_rewards_internal(position_nonce, &caller, &stake_info);

        // Clean up
        self.total_staked()
            .update(|total| *total -= &stake_info.amount);

        // Burn the NFT
        self.staked_nft_collection_id()
            .nft_burn(position_nonce, &BigUint::from(1u32));

        // Return SFTs
        self.send().direct_esdt(
            &caller,
            &self.staking_sft_collection_id().get(),
            stake_info.token_nonce,
            &stake_info.amount,
        );
    }

    /// Claim rewards.
    /// Payment: 1 single "staked NFT"
    #[endpoint(claimRewards)]
    #[payable]
    fn claim_rewards(&self) {
        self.update_reward_per_token();

        let payment = self.call_value().single_esdt();
        let caller = self.blockchain().get_caller();
        let position_nonce = payment.token_nonce;

        let staked_nft_collection_id = self.staked_nft_collection_id().get_token_id();

        // Verify the NFT is sent by the claimant
        self.require_valid_staked_nft_payment(payment.clone());

        // Get stake info from NFT attributes
        let stake_info = self.decode_nft_attributes(position_nonce);

        // Claim rewards
        self.claim_rewards_internal(position_nonce, &caller, &stake_info);

        // Send the NFT back to the caller
        self.send().direct_esdt(
            &caller,
            &staked_nft_collection_id,
            position_nonce,
            &BigUint::from(1u32),
        );
    }

    #[view(getPendingRewards)]
    fn get_pending_rewards_view(
        &self,
        staked_amount: BigUint,
        user_reward_per_token_paid: BigUint,
        user_rewards: BigUint,
    ) -> MultiValue3<BigUint, BigUint, BigUint> {
        self.get_pending_rewards(&staked_amount, &user_reward_per_token_paid, &user_rewards)
    }

    // Funder actions

    #[endpoint(issueStakedNftCollection)]
    #[payable("EGLD")]
    fn issue_staked_nft_collection(&self, token_name: ManagedBuffer, token_ticker: ManagedBuffer) {
        self.require_caller_is_funder();

        let payment = self.call_value().egld().clone_value();

        require!(
            payment == ISSUE_NFT_COLLECTION_FEE,
            "Invalid payment amount"
        );

        let caller = self.blockchain().get_caller();

        self.staked_nft_collection_id().issue_and_set_all_roles(
            EsdtTokenType::NonFungible,
            payment.clone(),
            token_name,
            token_ticker,
            0, // royalties (0 decimals)
            Some(self.callbacks().issue_callback(&caller)),
        );
    }

    #[callback]
    fn issue_callback(
        &self,
        caller: &ManagedAddress,
        #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(token_id) => {
                self.staked_nft_collection_id().set_token_id(token_id);
            }
            ManagedAsyncCallResult::Err(_) => {
                self.staked_nft_collection_id().clear();

                let payment = self.call_value().egld().clone_value();
                if payment > BigUint::zero() {
                    self.send().direct_egld(&caller, &payment);
                }
            }
        }
    }

    #[endpoint(fundRewardsAndSetDuration)]
    #[payable]
    fn fund_rewards_and_set_duration(&self, duration_in_seconds: u64) {
        self.require_caller_is_funder();

        self.update_reward_per_token();

        require!(self.reward_end_time().get() == 0, "Already started");

        let (token_id, amount) = self.call_value().egld_or_single_fungible_esdt();
        let current_time = self.blockchain().get_block_timestamp();

        require!(
            token_id == self.reward_token().get(),
            "Invalid reward token"
        );
        require!(amount > BigUint::zero(), "Must provide reward amount");
        require!(duration_in_seconds > 0, "Duration must be greater than 0");

        let reward_per_second = amount * REWARD_RATE_PRECISION / BigUint::from(duration_in_seconds);
        self.reward_per_second().set(&reward_per_second);
        self.reward_start_time().set(current_time);
        self.reward_end_time()
            .set(current_time + duration_in_seconds);
        self.last_update_time().set(current_time);
    }

    // Admin endpoints

    #[only_admin]
    #[endpoint(setPerformanceFeePercent)]
    fn set_performance_fee_percent(&self, fee_percent: u32) {
        require!(fee_percent <= 100, "Fee percent must be between 0 and 100");
        self.performance_fee_percent().set(fee_percent);
    }

    #[only_admin]
    #[endpoint(setFeeReceiver)]
    fn set_fee_receiver(&self, new_receiver: ManagedAddress) {
        self.fee_receiver().set(&new_receiver);
    }

    #[only_admin]
    #[endpoint(setFunder)]
    fn set_funder(&self, new_account: ManagedAddress) {
        self.funder().set(&new_account);
    }

    // Functions

    fn require_caller_is_funder(&self) {
        require!(
            self.blockchain().get_caller() == self.funder().get(),
            "Not funder"
        );
    }

    fn decode_nft_attributes(&self, nonce: u64) -> StakeInfo<Self::Api> {
        let serializer = ManagedSerializer::new();

        let nft_attributes = self
            .blockchain()
            .get_esdt_token_data(
                &self.blockchain().get_sc_address(),
                &self.staked_nft_collection_id().get_token_id(),
                nonce,
            )
            .attributes;

        serializer.top_decode_from_managed_buffer_custom_message(
            &nft_attributes,
            "Invalid NFT attributes",
        )
    }

    fn claim_rewards_internal(
        &self,
        position_nonce: u64,
        caller: &ManagedAddress,
        stake_info: &StakeInfo<Self::Api>,
    ) {
        let (rewards, user_amount, fee_amount) = self
            .get_pending_rewards(
                &stake_info.amount,
                &stake_info.user_reward_per_token_paid,
                &stake_info.rewards,
            )
            .into_tuple();

        let reward_token_id = self.reward_token().get();

        if rewards > 0 {
            // Update NFT attributes with new last_claim_timestamp
            let updated_stake_info = StakeInfo {
                token_nonce: stake_info.token_nonce,
                amount: stake_info.amount.clone(),
                stake_timestamp: stake_info.stake_timestamp,
                last_claim_timestamp: self.last_update_time().get(),
                user_reward_per_token_paid: self.reward_per_token_stored().get(),
                rewards: &stake_info.rewards + &rewards,
            };

            self.staked_nft_collection_id()
                .nft_update_attributes(position_nonce, &updated_stake_info);

            self.send().direct_non_zero(
                &self.fee_receiver().get(),
                &reward_token_id,
                0u64,
                &fee_amount,
            );

            self.send()
                .direct_non_zero(caller, &reward_token_id, 0u64, &user_amount);
        }
    }

    fn get_pending_rewards(
        &self,
        staked_amount: &BigUint,
        user_reward_per_token_paid: &BigUint,
        user_rewards: &BigUint,
    ) -> MultiValue3<BigUint, BigUint, BigUint> {
        let amount = staked_amount.clone()
            * (self.current_reward_per_token() - user_reward_per_token_paid)
            / SAFETY_CONSTANT
            + user_rewards;

        let fee = &amount * self.performance_fee_percent().get() / 100u32;

        MultiValue3::from((amount.clone(), amount - &fee, fee))
    }

    fn update_reward_per_token(&self) {
        self.reward_per_token_stored()
            .set(self.current_reward_per_token());
        self.last_update_time()
            .set(self.last_time_reward_applicable());
    }

    fn require_valid_staked_nft_payment(&self, payment: EsdtTokenPayment) {
        require!(
            payment.token_identifier == self.staked_nft_collection_id().get_token_id()
                && payment.amount == BigUint::from(1u32),
            "Invalid staking position NFT"
        );
    }

    #[view(getCurrentRewardPerToken)]
    fn current_reward_per_token(&self) -> BigUint {
        let reward_per_token_stored = self.reward_per_token_stored().get();

        if self.total_staked().get() == 0 {
            reward_per_token_stored
        } else {
            let time_diff = self.last_time_reward_applicable() - self.last_update_time().get();

            let reward_per_token_increase =
                (BigUint::from(time_diff) * self.reward_per_second().get() * SAFETY_CONSTANT)
                    / (self.total_staked().get() * REWARD_RATE_PRECISION);

            reward_per_token_stored + reward_per_token_increase
        }
    }

    #[inline]
    fn last_time_reward_applicable(&self) -> u64 {
        self.reward_end_time()
            .get()
            .min(self.blockchain().get_block_timestamp())
    }
}
