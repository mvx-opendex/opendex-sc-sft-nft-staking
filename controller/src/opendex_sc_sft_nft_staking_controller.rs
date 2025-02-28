#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod config;
mod factory;
mod fees;
mod proxy_staking;

/// SFT/NFT staking contract controller used to deploy & update staking contracts.
#[multiversx_sc::contract]
pub trait OpendexScSftNftStakingController:
    config::ConfigModule
    + factory::FactoryModule
    + fees::FeesModule
    + multiversx_sc_modules::pause::PauseModule
{
    #[init]
    fn init(&self, template_sc_address: ManagedAddress) {
        self.sc_template_address().set(template_sc_address);
    }

    #[upgrade]
    fn upgrade(&self) {}
}
