multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ConfigModule {
    #[only_owner]
    #[endpoint(setDefaultFeeReceiver)]
    fn set_default_fee_receiver(&self, receiver: &ManagedAddress) {
        self.default_fee_receiver().set(receiver);
    }

    #[only_owner]
    #[endpoint(setDefaultPerformanceFeePercent)]
    fn set_default_performance_fee_percent(&self, percent: u32) {
        self.default_performance_fee_percent().set(percent);
    }

    #[only_owner]
    #[endpoint(setScTemplateAddress)]
    fn set_sc_template_address(&self, address: &ManagedAddress) {
        self.sc_template_address().set(address);
    }

    #[view(getDefaultFeeReceiver)]
    #[storage_mapper("default_fee_receiver")]
    fn default_fee_receiver(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getDefaultPerformanceFeePercent)]
    #[storage_mapper("default_performance_fee_percent")]
    fn default_performance_fee_percent(&self) -> SingleValueMapper<u32>;

    #[view(getScTemplateAddress)]
    #[storage_mapper("sc_template_address")]
    fn sc_template_address(&self) -> SingleValueMapper<ManagedAddress>;
}
