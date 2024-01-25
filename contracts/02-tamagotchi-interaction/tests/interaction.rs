use gtest::{Log ,Program, System};
use tamagotchi_interaction_io::{TmgEvent, TmgAction};


// TODO: 0️⃣ Copy tests from the previous lesson and push changes to the master branch

#[test]
fn interaction_test() {
    let sys = System::new();
    sys.init_logger();
    let _program = Program::current(&sys);

    let res = _program.send(2, String::from("Init Tamagotchi"));
    
    assert!(!res.main_failed());

    // TODO: 6️⃣ Test new functionality
    let fed_res = _program.send(2, TmgAction::Feed);

    let log = Log::builder()
    .dest(2)
    .payload(TmgEvent::Fed);
assert!(res.contains(&log));


}
