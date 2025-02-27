use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn claim_rewards_1_alone_full_go() {
    world().run("scenarios/claim_rewards_1_alone_full.scen.json");
}

#[test]
fn claim_rewards_1_alone_half_go() {
    world().run("scenarios/claim_rewards_1_alone_half.scen.json");
}

#[test]
fn claim_rewards_2_10_000_go() {
    world().run("scenarios/claim_rewards_2_10_000.scen.json");
}

#[test]
fn claim_rewards_2_7_500_go() {
    world().run("scenarios/claim_rewards_2_7_500.scen.json");
}

#[test]
fn claim_rewards_errors_go() {
    world().run("scenarios/claim_rewards_errors.scen.json");
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
fn restart_rewards_1_alone_go() {
    world().run("scenarios/restart_rewards_1_alone.scen.json");
}

#[test]
fn restart_rewards_2_go() {
    world().run("scenarios/restart_rewards_2.scen.json");
}

#[test]
fn set_fee_receiver_go() {
    world().run("scenarios/set_fee_receiver.scen.json");
}

#[test]
fn set_performance_fee_go() {
    world().run("scenarios/set_performance_fee.scen.json");
}

#[test]
fn stake_1_go() {
    world().run("scenarios/stake_1.scen.json");
}

#[test]
fn stake_2_go() {
    world().run("scenarios/stake_2.scen.json");
}

#[test]
fn stake_errors_go() {
    world().run("scenarios/stake_errors.scen.json");
}

#[test]
fn unstake_1_alone_full_go() {
    world().run("scenarios/unstake_1_alone_full.scen.json");
}

#[test]
fn unstake_1_alone_half_go() {
    world().run("scenarios/unstake_1_alone_half.scen.json");
}

#[test]
fn unstake_errors_go() {
    world().run("scenarios/unstake_errors.scen.json");
}
