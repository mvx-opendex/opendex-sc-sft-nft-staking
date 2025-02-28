use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn deploy_staking_errors_go() {
    world().run("scenarios/deploy_staking_errors.scen.json");
}

#[test]
fn harvest_fees_go() {
    world().run("scenarios/harvest_fees.scen.json");
}

#[test]
fn opendex_sc_sft_nft_staking_controller_go() {
    world().run("scenarios/opendex_sc_sft_nft_staking_controller.scen.json");
}

#[test]
fn remove_contract_errors_go() {
    world().run("scenarios/remove_contract_errors.scen.json");
}

#[test]
fn set_default_fee_receiver_go() {
    world().run("scenarios/set_default_fee_receiver.scen.json");
}

#[test]
fn set_default_performance_fee_go() {
    world().run("scenarios/set_default_performance_fee.scen.json");
}

#[test]
fn set_deployment_fee_go() {
    world().run("scenarios/set_deployment_fee.scen.json");
}

#[test]
fn set_sc_template_address_go() {
    world().run("scenarios/set_sc_template_address.scen.json");
}

#[test]
fn upgrade_contract_errors_go() {
    world().run("scenarios/upgrade_contract_errors.scen.json");
}
