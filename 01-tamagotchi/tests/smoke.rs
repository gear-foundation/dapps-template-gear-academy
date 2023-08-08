use gtest::{Program, System};

#[test]
fn smoke_test() {
    let system = System::new();
    let _program = Program::current(&system);

    // TODO: 8️⃣ Test the program initialization and message handling
}
