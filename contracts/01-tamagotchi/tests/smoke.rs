use gstd::debug;
use gtest::{Log, Program, System};
use tamagotchi_io:: TmgAction;







#[test]
fn smoke_test() {
    let sys = System::new();
    sys.init_logger();
    let program: Program<'_> = Program::current(&sys);
  
    let res = program.send(2, String::from("Init Tamagotchi"));
    
    assert!(!res.main_failed());

    let res_tmgAction_name = program.send(2, TmgAction::Name);

    let expected_log = Log::builder()
    .dest(2)
    .payload(String::from("Ivan"));
     assert!(res_tmgAction_name.contains(&expected_log));

//     let res_tmgAction_age = program.send(2, TmgAction::Age);
// debug!("tamagotchi age is: {:?}", res_tmgAction_age);
    
}
