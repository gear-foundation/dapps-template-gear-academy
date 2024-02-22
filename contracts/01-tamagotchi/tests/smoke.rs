use gtest::{Program, System, Log};
use tamagotchi_io::{Tamagotchi, TmgAction, TmgEvent};

#[test]
fn smoke_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);

    // TODO: 8️⃣ Test the program initialization and message handling

    let name = "Coco".to_string();

    let age = sys.block_timestamp();

    let tmgochi = Tamagotchi {
        name: name.clone(),
        date_of_birth: age,
    };

    let res = program.send(2, tmgochi);
    assert!(!res.main_failed());


    let res = program.send(2,  TmgAction::Name);
    let log = Log::builder()
        .payload(TmgEvent::Name(name));
     assert!(res.contains(&log));


    let res = program.send(2,  TmgAction::Age);
    let log = Log::builder()
        .payload(TmgEvent::Age(age - sys.block_timestamp()));
     assert!(res.contains(&log));

}
