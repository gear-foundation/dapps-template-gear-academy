use gtest::{Program, System};
use tamagotchi_io::TmgAction; 

#[test]
fn smoke_test() {
    let sys = System::new();
    sys.init_logger();
    let _program = Program::current(&sys);

    // TODO: 8️⃣ Test the program initialization and message handling
    let res = _program.send(2, String::from("Nombre Tamagotchi"));
    assert!(!res.main_failed());

    let res = _program.send(
        2,
        TmgAction::Name,
    );
    assert!(!res.main_failed());

    let res = _program.send(
        2,
        TmgAction::Age,
    );
    assert!(!res.main_failed());

}
