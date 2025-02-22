use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn fund_and_start_go() {
    world().run("scenarios/fund_and_start.scen.json");
}

#[test]
fn fund_and_start_errors_go() {
    world().run("scenarios/fund_and_start_errors.scen.json");
}

#[test]
fn init_go() {
    world().run("scenarios/init.scen.json");
}

#[test]
fn issue_nft_collection_go() {
    world().run("scenarios/issue_nft_collection.scen.json");
}

#[test]
fn opendex_sc_sft_nft_staking_go() {
    world().run("scenarios/opendex_sc_sft_nft_staking.scen.json");
}

#[test]
fn set_fee_receiver_go() {
    world().run("scenarios/set_fee_receiver.scen.json");
}

#[test]
fn set_performance_fee_go() {
    world().run("scenarios/set_performance_fee.scen.json");
}
