use gtest::{Program, System, Log};
use tamagotchi_io::*;

#[test]
fn smoke_test() {
    let sys = System::new();
    sys.init_logger();
    let _program = Program::current(&sys);

    _program.send(2, String::from("Heriel"));

    let _res = _program.send(2, TmgAction::Name);
    let expected_log = Log::builder()
    .dest(2)
    .payload(TmgEvent::Name("Heriel".to_string()));
    assert!(_res.contains(&expected_log));

}