multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[type_abi]
#[derive(ManagedVecItem, NestedDecode, NestedEncode, TopEncode, TopDecode)]
pub struct Contract<M: ManagedTypeApi> {
    pub sc_address: ManagedAddress<M>,
    owner: ManagedAddress<M>,
}

#[type_abi]
#[derive(ManagedVecItem, NestedDecode, NestedEncode, TopEncode, TopDecode)]
pub struct ContractAndStakingStatus<M: ManagedTypeApi> {
    contract: Contract<M>,
    status: crate::proxy_staking::Status<M>,
}

#[multiversx_sc::module]
pub trait FactoryModule:
    crate::config::ConfigModule + crate::fees::FeesModule + multiversx_sc_modules::pause::PauseModule
{
    #[payable("EGLD")]
    #[endpoint(deployStaking)]
    fn deploy_staking(
        &self,
        staking_sft_collection_id: TokenIdentifier,
        min_nonce_id: u64,
        max_nonce_id: u64,
        reward_token_id: EgldOrEsdtTokenIdentifier,
    ) -> ManagedAddress {
        let caller = self.blockchain().get_caller();

        if caller != self.blockchain().get_owner_address() {
            self.require_not_paused();
        }

        require!(
            staking_sft_collection_id.is_valid_esdt_identifier(),
            "Invalid staking collection ID"
        );

        require!(reward_token_id.is_valid(), "Invalid reward token ID");

        self.require_valid_egld_payment();

        require!(
            !self.sc_template_address().is_empty(),
            "SC template is not set"
        );

        let (new_sc_address, ()) = self
            .tx()
            .typed(crate::proxy_staking::OpendexSftNftStakingProxy)
            .init(
                &staking_sft_collection_id,
                min_nonce_id,
                max_nonce_id,
                &reward_token_id,
                self.default_fee_receiver().get(),
                self.default_performance_fee_percent().get(),
                &caller,
            )
            .deploy_from_source(
                &self.sc_template_address().get(),
                CodeMetadata::UPGRADEABLE | CodeMetadata::READABLE | CodeMetadata::PAYABLE_BY_SC,
            );

        let contract = Contract::<Self::Api> {
            sc_address: new_sc_address.clone(),
            owner: caller.clone(),
        };

        self.all_contracts().push(&contract);

        self.contract(&new_sc_address).set(&contract);

        new_sc_address
    }

    #[endpoint(upgradeStaking)]
    fn upgrade_staking(&self, sc_address: &ManagedAddress) {
        self.check_user_rights_on_contract(sc_address);

        self.tx()
            .to(sc_address.clone())
            .raw_upgrade()
            .from_source(self.sc_template_address().get())
            .code_metadata(
                CodeMetadata::UPGRADEABLE | CodeMetadata::READABLE | CodeMetadata::PAYABLE_BY_SC,
            )
            .upgrade_async_call_and_exit();
    }

    /// Remove a staking contract.
    ///
    /// Staking MUST be finished and have no more stakers left.
    #[only_owner]
    #[endpoint(removeContract)]
    fn remove_contract(&self, sc_address: ManagedAddress) {
        let status = self
            .tx()
            .to(sc_address.clone())
            .typed(crate::proxy_staking::OpendexSftNftStakingProxy)
            .get_status()
            .returns(ReturnsResult)
            .sync_call();

        require!(
            status.reward_end_time < self.blockchain().get_block_timestamp(),
            "Staking is not ended"
        );

        require!(status.total_staked == 0, "Still stakers left");

        let index_ = self
            .all_contracts()
            .iter()
            .enumerate()
            .find(|(_i, contract)| contract.sc_address == sc_address)
            .map(|(i, _contract)| i);

        require!(index_.is_some(), "Contract not found");

        self.all_contracts().swap_remove(index_.unwrap() + 1);
    }

    fn check_user_rights_on_contract(&self, sc_address: &ManagedAddress) -> Contract<Self::Api> {
        require!(!self.contract(sc_address).is_empty(), "Contract not found");

        let contract = self.contract(sc_address).get();

        let caller = self.blockchain().get_caller();

        if caller != self.blockchain().get_owner_address() {
            self.require_not_paused();

            require!(contract.owner == caller, "Not owner");
        }

        contract
    }

    #[view(getContractsAndStatuses)]
    fn get_contracts_and_statuses(
        &self,
        skip: usize,
        limit: usize,
    ) -> MultiValueEncoded<ContractAndStakingStatus<Self::Api>> {
        self.get_all_contracts(skip, limit)
            .into_iter()
            .map(|contract| {
                let status = self
                    .tx()
                    .to(contract.sc_address.clone())
                    .typed(crate::proxy_staking::OpendexSftNftStakingProxy)
                    .get_status()
                    .returns(ReturnsResult)
                    .sync_call_readonly();

                ContractAndStakingStatus::<Self::Api> { contract, status }
            })
            .collect()
    }

    #[view(getAllContracts)]
    fn get_all_contracts(
        &self,
        skip: usize,
        limit: usize,
    ) -> MultiValueEncoded<Contract<Self::Api>> {
        self.all_contracts().iter().skip(skip).take(limit).collect()
    }

    #[storage_mapper("all_contracts")]
    fn all_contracts(&self) -> VecMapper<Contract<Self::Api>>;

    #[view(getContract)]
    #[storage_mapper("contract")]
    fn contract(&self, sc_address: &ManagedAddress) -> SingleValueMapper<Contract<Self::Api>>;
}
