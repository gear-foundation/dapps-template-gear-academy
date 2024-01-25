use gtest::{Log, Program, System};
use tamagotchi_io::TmgAction;

#[test]
fn smoke_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);

    let name = "Pikachu";

    let res = program.send(2, String::from(name));
    assert!(!res.main_failed());

    let res = program.send(2, TmgAction::Name);

    let expected_log = Log::builder().dest(2).payload(String::from(name));
    assert!(res.contains(&expected_log));
}
