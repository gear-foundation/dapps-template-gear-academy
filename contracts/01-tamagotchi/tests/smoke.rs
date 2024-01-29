use gstd::debug;
use gtest::{Log, Program, System};
use tamagotchi_io::TmgAction;


#[test]
fn smoke_test() {
    let sys = System::new();
    sys.init_logger();
    let program: Program<'_> = Program::current(&sys);
  
    let res = program.send(2, String::from("Ivan"));
    
    assert!(!res.main_failed());

    let res_tmgAction_name = program.send(2, TmgAction::Name);

    let expected_log = Log::builder()
    .dest(2)
    .payload(String::from("Ivan"));
     assert!(res_tmgAction_name.contains(&expected_log));

    let res_tmgAction_age = program.send(2, TmgAction::Age);
    assert!(!res_tmgAction_age.main_failed());
    
}
