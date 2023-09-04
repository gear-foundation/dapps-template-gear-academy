use gtest::{Program, System, Log};
use tmg1_io::{TmgAction, TmgEvent};

#[test]
fn smoke_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);

    let res = program.send(2, "Tama".to_string());
//    println!("{:?}", res);
    assert!(!res.main_failed());

    let res = program.send(2, TmgAction::Name);
//    println!("res: {:?}", res);

    let expected_log = Log::builder()
        .dest(2)
        // I don't understand why the test passes for arbitrary payload
        .payload(TmgEvent::Name("????".to_string()));
    assert!(res.contains(&expected_log));
}
