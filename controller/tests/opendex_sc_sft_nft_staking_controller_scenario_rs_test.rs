use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    // blockchain.set_current_dir_from_workspace("relative path to your workspace, if applicable");
    blockchain.register_contract("mxsc:output/opendex-sc-sft-nft-staking-controller.mxsc.json", opendex_sc_sft_nft_staking_controller::ContractBuilder);
    blockchain
}

#[test]
fn deploy_staking_errors_rs() {
    world().run("scenarios/deploy_staking_errors.scen.json");
}

#[test]
fn harvest_fees_rs() {
    world().run("scenarios/harvest_fees.scen.json");
}

#[test]
fn opendex_sc_sft_nft_staking_controller_rs() {
    world().run("scenarios/opendex_sc_sft_nft_staking_controller.scen.json");
}

#[test]
fn remove_contract_errors_rs() {
    world().run("scenarios/remove_contract_errors.scen.json");
}

#[test]
fn set_default_fee_receiver_rs() {
    world().run("scenarios/set_default_fee_receiver.scen.json");
}

#[test]
fn set_default_performance_fee_rs() {
    world().run("scenarios/set_default_performance_fee.scen.json");
}

#[test]
fn set_deployment_fee_rs() {
    world().run("scenarios/set_deployment_fee.scen.json");
}

#[test]
fn set_sc_template_address_rs() {
    world().run("scenarios/set_sc_template_address.scen.json");
}

#[test]
fn upgrade_contract_errors_rs() {
    world().run("scenarios/upgrade_contract_errors.scen.json");
}
