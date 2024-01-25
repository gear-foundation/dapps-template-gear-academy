use gtest::{Log, Program, System};
use tamagotchi_nft_io::{TmgEvent, TmgAction};
// TODO: 0️⃣ Copy tests from the previous lesson and push changes to the master branch

#[test]
fn owning_test() {
    let sys = System::new();
    sys.init_logger();
    let _program = Program::current(&sys);

    // TODO: 6️⃣ Test new functionality
    let res = _program.send(2, String::from("Init Tamagotchi"));
    
    assert!(!res.main_failed());
}
