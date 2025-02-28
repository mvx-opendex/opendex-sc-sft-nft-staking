multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait FeesModule {
    #[only_owner]
    #[endpoint(harvestFees)]
    fn harvest_fees(&self) {
        let caller = self.blockchain().get_caller();

        let balance = self
            .blockchain()
            .get_sc_balance(&EgldOrEsdtTokenIdentifier::egld(), 0u64);

        self.send().direct_non_zero_egld(&caller, &balance);
    }

    #[only_owner]
    #[endpoint(setDeploymentFee)]
    fn set_deployment_fee(&self, fee: &BigUint) {
        self.deployment_fee().set(fee);
    }

    fn require_valid_egld_payment(&self) {
        let egld_payment = self.call_value().egld().clone_value();

        let expected_payment = self.deployment_fee().get();

        require!(egld_payment == expected_payment, "Invalid payment amount");
    }

    #[view(getDeploymentFee)]
    #[storage_mapper("deployment_fee")]
    fn deployment_fee(&self) -> SingleValueMapper<BigUint>;
}
