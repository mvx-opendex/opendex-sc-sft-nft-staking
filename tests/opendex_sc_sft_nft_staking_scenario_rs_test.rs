use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    // blockchain.set_current_dir_from_workspace("relative path to your workspace, if applicable");
    blockchain.register_contract("mxsc:output/opendex-sc-sft-nft-staking.mxsc.json", opendex_sc_sft_nft_staking::ContractBuilder);
    blockchain
}

#[test]
fn opendex_sc_sft_nft_staking_rs() {
    world().run("scenarios/opendex_sc_sft_nft_staking.scen.json");
}
