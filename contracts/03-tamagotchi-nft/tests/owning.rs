use gtest::{Program, System};

// TODO: 0️⃣ Copy tests from the previous lesson and push changes to the master branch
#[test]
fn tamagotchi_test_init() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    let res = program.send(2, String::from("DavidHM"));

    let  expected_init_log = Log::builder()
        .dest(2)
        .payload("successful initialization!");

    assert!(res.contains(&expected_init_log));
    assert!(!res.main_failed());
}

#[test]
fn correct_name() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    let res = program.send(2, String::from("david"));
    let  expected_name_log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Name(String::from("david")));
    let res = program.send(
        2,
        TmgAction::Name
    );
    assert!(!res.main_failed());
    assert!(res.contains(&expected_name_log));
}

#[test]
fn tamagotchi_feed() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    let res = program.send(2, String::from("david"));
    
    let  expected_name_log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Fed);
    let res = program.send(
        2,
        TmgAction::Feed
    );
    assert!(!res.main_failed());
    assert!(res.contains(&expected_name_log));
}

#[test]
fn tamagotchi_entertanied() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    let res = program.send(2, String::from("david"));
    let  expected_name_log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Entertained);
    let res = program.send(
        2,
        TmgAction::Play
    );
    assert!(!res.main_failed());
    assert!(res.contains(&expected_name_log));
}

#[test]
fn tamagotchi_slept() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    let res = program.send(2, String::from("david"));
    let  expected_name_log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Slept);
    let res = program.send(
        2,
        TmgAction::Sleep
    );
    assert!(!res.main_failed());
    assert!(res.contains(&expected_name_log));
}



#[test]
fn owning_test() {
    let sys = System::new();
    sys.init_logger();
    let _program = Program::current(&sys);

    // TODO: 6️⃣ Test new functionality
}
