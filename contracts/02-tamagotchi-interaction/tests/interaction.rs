use gtest::{Log, Program, System};
use tamagotchi_interaction_io::{TmgAction, TmgEvent};

// TODO: 0️⃣ Copy tests from the previous lesson and push changes to the master branch
#[test]
fn smoke_test() {
    let sys = System::new();
    sys.init_logger();
    let _program = Program::current(&sys);

    // TODO: 8️⃣ Test the program initialization and message handling
    let res = _program.send(2, String::from("Tamagotchi Name"));
    assert!(!res.main_failed());

    let res = _program.send(2, TmgAction::Name);
    let expected_log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Name(String::from("Tamagotchi Name")));
    assert!(res.contains(&expected_log));

    let res = _program.send(2, TmgAction::Age);
    assert!(!res.log().is_empty());
}

#[test]
fn interaction_test() {
    let sys = System::new();
    sys.init_logger();
    let _program = Program::current(&sys);

    // TODO: 6️⃣ Test new functionality
    let res = _program.send(2, TmgAction::Feed);
    let expected_log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Fed);
    assert!(res.contains(&expected_log));

    let res = _program.send(2, TmgAction::Entertain);
    let expected_log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Entertained);
    assert!(res.contains(&expected_log));

    let res = _program.send(2, TmgAction::Sleep);
    let expected_log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Slept);
    assert!(res.contains(&expected_log));
}
