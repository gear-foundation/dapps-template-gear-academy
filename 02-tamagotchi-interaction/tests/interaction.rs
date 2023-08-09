use gtest::{Program, System};

// TODO: 0️⃣ Copy tests from the previous lesson

#[test]
fn interaction_test() {
    let sys = System::new();
    sys.init_logger();
    let _program = Program::current(&sys);

    // TODO: 6️⃣ Test the functionality
}
