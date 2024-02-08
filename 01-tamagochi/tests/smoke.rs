use gtest::{Log, Program, System};
use io::{TmAction, TmEvent};

#[test]
fn smoke_test() {
    let system = System::new();
    system.init_logger();
    
    let program = Program::current(&system);

    let tamagochi_name = "Tamagochi_smoke";

    let mut res = program.send(2, String::from(tamagochi_name));

    assert!(!res.main_failed());

    res = program.send(2, TmAction::Name);
    let expected_log = Log::builder().dest(2).payload(TmEvent::Name(String::from(tamagochi_name)));

    assert!(res.contains(&expected_log));
}