use gtest::{Program, System};

// TODO: 0️⃣ Copy tests from the previous lesson and push changes to the master branch

#[test]
fn smoke_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    let result = program.send(2, String::from("Goodmoring"));
    assert!(!result.main_failed());
    let result = program.send(2, TmgAction::Name);
    let log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Name(String::from("Goodmoring")));
    assert!(result.contains(&log));
    let _result = program.send(2, TmgAction::Age);
    // let log = Log::builder().dest(2).payload(TmgEvent::Age(sys.block_timestamp()));
    // assert!(result.contains(&log));

    //How to test the age?
}

#[test]
fn negative_smoke_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    let payload = vec![1, 2, 3];
    let _result = program.send(2, payload);
    // assert!(result.main_failed());
    // Why the assert is panic?

    // let result = program.send(1, TmgAction::Name);
    // let log = Log::builder().dest(2).payload(TmgEvent::Name("Goodmoring".to_string()));
    // assert!(!result.contains(&log));
    // let result = program.send(1, TmgAction::Age);
    //     let log = Log::builder().dest(2).payload(TmgEvent::Age(sys.block_timestamp()));
    //     assert!(!result.contains(&log));
}

#[test]
fn interaction_test() {
    let sys = System::new();
    sys.init_logger();
    let _program = Program::current(&sys);
    // TODO: 6️⃣ Test new functionality
}
