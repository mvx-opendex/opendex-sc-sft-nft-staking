#![no_std]

use multiversx_sc::contract_base::ManagedSerializer;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

static ISSUE_NFT_COLLECTION_FEE: u64 = 50000000000000000u64;
static RPS_PRECISION: u64 = 10_000_000_000u64;

#[type_abi]
#[derive(TopEncode, TopDecode)]
pub struct StakeInfo<M: ManagedTypeApi> {
    token_nonce: u64,          // SFT nonce staked
    amount: BigUint<M>,        // Amount of SFTs staked
    stake_timestamp: u64,      // When staked
    last_claim_timestamp: u64, // Last reward claim
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
        self.reward_start_time().set(0u64);
        self.reward_period_end().set(0u64);
        self.funder().set(&funder);
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
    #[storage_mapper("reward_period_end")]
    fn reward_period_end(&self) -> SingleValueMapper<u64>;

    #[view(getRewardStartTime)]
    #[storage_mapper("reward_start_time")]
    fn reward_start_time(&self) -> SingleValueMapper<u64>;

    #[view(getFunder)]
    #[storage_mapper("funder")]
    fn funder(&self) -> SingleValueMapper<ManagedAddress>;

    // User endpoints

    #[endpoint]
    #[payable]
    fn stake(&self) {
        self.staked_nft_collection_id().require_issued_or_set();

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
        let payment = self.call_value().single_esdt();
        let caller = self.blockchain().get_caller();
        let position_nonce = payment.token_nonce;

        let staked_nft_collection_mapper = self.staked_nft_collection_id();
        let staked_nft_collection_id = staked_nft_collection_mapper.get_token_id();

        // Verify the NFT is sent back
        require!(
            payment.token_identifier == staked_nft_collection_id
                && payment.amount == BigUint::from(1u32),
            "Must send valid staking position NFT"
        );

        // Get stake info from NFT attributes
        let stake_info = self.decode_nft_attributes(position_nonce);

        // Claim rewards
        self.claim_rewards_internal(position_nonce, &caller, &stake_info);

        // Return SFTs
        self.send().direct_esdt(
            &caller,
            &self.staking_sft_collection_id().get(),
            stake_info.token_nonce,
            &stake_info.amount,
        );

        // Clean up
        self.total_staked()
            .update(|total| *total -= &stake_info.amount);

        // Burn the NFT
        staked_nft_collection_mapper.nft_burn(position_nonce, &BigUint::from(1u32));
    }

    #[endpoint(claimRewards)]
    #[payable("*")]
    fn claim_rewards(&self) {
        let payment = self.call_value().single_esdt();
        let caller = self.blockchain().get_caller();
        let position_nonce = payment.token_nonce;

        let staked_nft_collection_id = self.staked_nft_collection_id().get_token_id();

        // Verify the NFT is sent by the claimant
        require!(
            payment.token_identifier == staked_nft_collection_id
                && payment.amount == BigUint::from(1u32),
            "Must send valid staking position NFT"
        );

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

        let (token_id, amount) = self.call_value().egld_or_single_fungible_esdt();
        let current_time = self.blockchain().get_block_timestamp();

        require!(
            token_id == self.reward_token().get(),
            "Invalid reward token"
        );
        require!(amount > BigUint::zero(), "Must provide reward amount");
        require!(duration_in_seconds > 0, "Duration must be greater than 0");

        let reward_per_second = amount * RPS_PRECISION / BigUint::from(duration_in_seconds);
        self.reward_per_second().set(&reward_per_second);
        self.reward_start_time().set(current_time);
        self.reward_period_end()
            .set(current_time + duration_in_seconds);
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
        let current_time = self.blockchain().get_block_timestamp();
        let reward_start = self.reward_start_time().get();
        let reward_end = self.reward_period_end().get();
        let total_staked = self.total_staked().get();

        require!(total_staked > BigUint::zero(), "No tokens staked in pool");

        let claim_end_time = if current_time > reward_end && reward_end > 0 {
            reward_end
        } else {
            current_time
        };

        let claim_start_time = if reward_start > stake_info.last_claim_timestamp {
            reward_start
        } else {
            stake_info.last_claim_timestamp
        };

        let time_diff = if claim_end_time > claim_start_time {
            claim_end_time - claim_start_time
        } else {
            0
        };

        let total_possible_rewards = &self.reward_per_second().get() * time_diff / RPS_PRECISION;
        let user_share = &stake_info.amount * &total_possible_rewards / &total_staked;

        if user_share > BigUint::zero() {
            let fee_percent = BigUint::from(self.performance_fee_percent().get());
            let fee_amount = &user_share * &fee_percent / &BigUint::from(100u32);
            let user_amount = &user_share - &fee_amount;

            self.send().direct_non_zero(
                &self.fee_receiver().get(),
                &self.reward_token().get(),
                0u64,
                &fee_amount,
            );

            self.send()
                .direct_non_zero(caller, &self.reward_token().get(), 0u64, &user_amount);

            // Update NFT attributes with new last_claim_timestamp
            let updated_stake_info = StakeInfo {
                token_nonce: stake_info.token_nonce,
                amount: stake_info.amount.clone(),
                stake_timestamp: stake_info.stake_timestamp,
                last_claim_timestamp: claim_end_time,
            };

            self.staked_nft_collection_id()
                .nft_update_attributes(position_nonce, &updated_stake_info);
        }
    }
}
