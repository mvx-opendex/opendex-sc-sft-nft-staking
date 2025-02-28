use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    // blockchain.set_current_dir_from_workspace("relative path to your workspace, if applicable");
    blockchain.register_contract("mxsc:output/opendex-sc-sft-nft-staking.mxsc.json", opendex_sc_sft_nft_staking::ContractBuilder);
    blockchain
}

#[test]
fn claim_rewards_1_alone_full_rs() {
    world().run("scenarios/claim_rewards_1_alone_full.scen.json");
}

#[test]
fn claim_rewards_1_alone_half_rs() {
    world().run("scenarios/claim_rewards_1_alone_half.scen.json");
}

#[test]
fn claim_rewards_1_no_reward_rs() {
    world().run("scenarios/claim_rewards_1_no_reward.scen.json");
}

#[test]
fn claim_rewards_2_10_000_rs() {
    world().run("scenarios/claim_rewards_2_10_000.scen.json");
}

#[test]
fn claim_rewards_2_7_500_rs() {
    world().run("scenarios/claim_rewards_2_7_500.scen.json");
}

#[test]
fn claim_rewards_errors_rs() {
    world().run("scenarios/claim_rewards_errors.scen.json");
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
fn restart_rewards_1_alone_rs() {
    world().run("scenarios/restart_rewards_1_alone.scen.json");
}

#[test]
fn restart_rewards_2_rs() {
    world().run("scenarios/restart_rewards_2.scen.json");
}

#[test]
fn set_fee_receiver_rs() {
    world().run("scenarios/set_fee_receiver.scen.json");
}

#[test]
fn set_funder_rs() {
    world().run("scenarios/set_funder.scen.json");
}

#[test]
fn set_funder_not_admin_rs() {
    world().run("scenarios/set_funder_not_admin.scen.json");
}

#[test]
fn set_performance_fee_rs() {
    world().run("scenarios/set_performance_fee.scen.json");
}

#[test]
fn stake_1_rs() {
    world().run("scenarios/stake_1.scen.json");
}

#[test]
fn stake_2_rs() {
    world().run("scenarios/stake_2.scen.json");
}

#[test]
fn stake_errors_rs() {
    world().run("scenarios/stake_errors.scen.json");
}

#[test]
fn unstake_1_alone_full_rs() {
    world().run("scenarios/unstake_1_alone_full.scen.json");
}

#[test]
fn unstake_1_alone_half_rs() {
    world().run("scenarios/unstake_1_alone_half.scen.json");
}

#[test]
fn unstake_1_no_reward_rs() {
    world().run("scenarios/unstake_1_no_reward.scen.json");
}

#[test]
fn unstake_errors_rs() {
    world().run("scenarios/unstake_errors.scen.json");
}
