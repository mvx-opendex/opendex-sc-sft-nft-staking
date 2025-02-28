use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn opendex_sc_sft_nft_staking_controller_go() {
    world().run("scenarios/opendex_sc_sft_nft_staking_controller.scen.json");
}
