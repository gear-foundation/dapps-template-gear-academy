use gtest::{Log, Program, System};
use tamagotchi_io::{TmgAction, TmgEvent};

#[test]
fn smoke_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);

    // TODO: 8️⃣ Test the program initialization and message handling
    let tamagotchi_name = String::from("Keno");
    let initialize_contract = program.send(2, tamagotchi_name.clone());

    assert!(!initialize_contract.main_failed(), "init function failed");

    let get_name_action = TmgAction::Name;
    let get_name_result = program.send(3, get_name_action);

    assert!(!get_name_result.main_failed(), "Name request failed");

    let expected_tamagatchi = Log::builder()
        .dest(3)
        .payload(TmgEvent::Name(String::from("Keno")));

    assert!(get_name_result.contains(&expected_tamagatchi));


}
