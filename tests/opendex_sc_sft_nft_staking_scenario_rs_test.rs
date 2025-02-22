use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    // blockchain.set_current_dir_from_workspace("relative path to your workspace, if applicable");
    blockchain.register_contract("mxsc:output/opendex-sc-sft-nft-staking.mxsc.json", opendex_sc_sft_nft_staking::ContractBuilder);
    blockchain
}

#[test]
fn fund_and_start_rs() {
    world().run("scenarios/fund_and_start.scen.json");
}

#[test]
fn fund_and_start_errors_rs() {
    world().run("scenarios/fund_and_start_errors.scen.json");
}

#[test]
fn init_rs() {
    world().run("scenarios/init.scen.json");
}

#[test]
fn issue_nft_collection_rs() {
    world().run("scenarios/issue_nft_collection.scen.json");
}

#[test]
fn opendex_sc_sft_nft_staking_rs() {
    world().run("scenarios/opendex_sc_sft_nft_staking.scen.json");
}

#[test]
fn set_fee_receiver_rs() {
    world().run("scenarios/set_fee_receiver.scen.json");
}

#[test]
fn set_performance_fee_rs() {
    world().run("scenarios/set_performance_fee.scen.json");
}
