use gtest::{Program, System};

#[test]
fn smoke_test() {
    let sys = System::new();
    sys.init_logger();
    let _program = Program::current(&sys);

    // TODO: 8️⃣ Test the program initialization and message handling
}
