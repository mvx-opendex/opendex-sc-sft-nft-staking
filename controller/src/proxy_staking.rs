// Code generated by the multiversx-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use multiversx_sc::proxy_imports::*;

pub struct OpendexSftNftStakingProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for OpendexSftNftStakingProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = OpendexSftNftStakingProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        OpendexSftNftStakingProxyMethods { wrapped_tx: tx }
    }
}

pub struct OpendexSftNftStakingProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> OpendexSftNftStakingProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<EgldOrEsdtTokenIdentifier<Env::Api>>,
        Arg2: ProxyArg<ManagedAddress<Env::Api>>,
        Arg3: ProxyArg<u32>,
        Arg4: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        staking_sft_collection_id: Arg0,
        reward_token: Arg1,
        fee_receiver: Arg2,
        performance_fee: Arg3,
        funder: Arg4,
    ) -> TxTypedDeploy<Env, From, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_deploy()
            .argument(&staking_sft_collection_id)
            .argument(&reward_token)
            .argument(&fee_receiver)
            .argument(&performance_fee)
            .argument(&funder)
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> OpendexSftNftStakingProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn upgrade(
        self,
    ) -> TxTypedUpgrade<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_upgrade()
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> OpendexSftNftStakingProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn staking_sft_collection_id(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, TokenIdentifier<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getStakingSftCollectionId")
            .original_result()
    }

    pub fn reward_token(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, EgldOrEsdtTokenIdentifier<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getRewardToken")
            .original_result()
    }

    pub fn staked_nft_collection_id(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, TokenIdentifier<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getStakedNftCollectionId")
            .original_result()
    }

    /// Return amount of reward per second (multiplied by 10^10 for precision) 
    pub fn reward_per_second(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getRewardPerSecond")
            .original_result()
    }

    pub fn total_staked(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getTotalStaked")
            .original_result()
    }

    pub fn fee_receiver(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getFeeReceiver")
            .original_result()
    }

    pub fn performance_fee_percent(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u32> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getPerformanceFeePercent")
            .original_result()
    }

    pub fn reward_end_time(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u64> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getRewardPeriodEnd")
            .original_result()
    }

    pub fn reward_start_time(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u64> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getRewardStartTime")
            .original_result()
    }

    pub fn funder(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getFunder")
            .original_result()
    }

    pub fn last_update_time(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u64> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getLastUpdateTime")
            .original_result()
    }

    pub fn reward_per_token_stored(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getRewardPerTokenStored")
            .original_result()
    }

    /// Stake SFT/NFTs. 
    /// Payment: 1 single payment accepted. 
    pub fn stake(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("stake")
            .original_result()
    }

    /// Untake SFT/NFTs by sending a staked NFT receipt. 
    /// Payment: 1 single payment accepted. 
    pub fn unstake(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("unstake")
            .original_result()
    }

    /// Claim rewards. 
    /// Payment: 1 single "staked NFT". 
    pub fn claim_rewards(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("claimRewards")
            .original_result()
    }

    pub fn get_pending_rewards_view<
        Arg0: ProxyArg<BigUint<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
        Arg2: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        staked_amount: Arg0,
        user_reward_per_token_paid: Arg1,
        user_rewards: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValue3<BigUint<Env::Api>, BigUint<Env::Api>, BigUint<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getPendingRewards")
            .argument(&staked_amount)
            .argument(&user_reward_per_token_paid)
            .argument(&user_rewards)
            .original_result()
    }

    pub fn issue_staked_nft_collection<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
    >(
        self,
        token_name: Arg0,
        token_ticker: Arg1,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("issueStakedNftCollection")
            .argument(&token_name)
            .argument(&token_ticker)
            .original_result()
    }

    pub fn fund_rewards_and_set_duration<
        Arg0: ProxyArg<u64>,
    >(
        self,
        duration_in_seconds: Arg0,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("fundRewardsAndSetDuration")
            .argument(&duration_in_seconds)
            .original_result()
    }

    pub fn set_performance_fee_percent_endpoint<
        Arg0: ProxyArg<u32>,
    >(
        self,
        fee_percent: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setPerformanceFeePercent")
            .argument(&fee_percent)
            .original_result()
    }

    pub fn set_fee_receiver<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        new_receiver: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setFeeReceiver")
            .argument(&new_receiver)
            .original_result()
    }

    pub fn set_funder<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        new_account: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setFunder")
            .argument(&new_account)
            .original_result()
    }

    pub fn current_reward_per_token(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getCurrentRewardPerToken")
            .original_result()
    }

    pub fn get_status(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, Status<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getStatus")
            .original_result()
    }

    pub fn is_admin<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, bool> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("isAdmin")
            .argument(&address)
            .original_result()
    }

    pub fn add_admin<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("addAdmin")
            .argument(&address)
            .original_result()
    }

    pub fn remove_admin<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("removeAdmin")
            .argument(&address)
            .original_result()
    }

    pub fn admins(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getAdmins")
            .original_result()
    }
}

#[type_abi]
#[derive(ManagedVecItem, NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct Status<Api>
where
    Api: ManagedTypeApi,
{
    pub staking_sft_collection_id: TokenIdentifier<Api>,
    pub total_staked: BigUint<Api>,
    pub reward_token: EgldOrEsdtTokenIdentifier<Api>,
    pub staked_nft_collection_id: Option<TokenIdentifier<Api>>,
    pub reward_start_time: u64,
    pub reward_end_time: u64,
    pub reward_per_second: BigUint<Api>,
    pub fee_receiver: ManagedAddress<Api>,
    pub performance_fee: u32,
    pub funder: ManagedAddress<Api>,
}
