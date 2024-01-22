use gtest::{Program, System};

#[test]
fn smoke_test() {
    let sys = System::new();
    sys.init_logger();
    let program: Program<'_> = Program::current(&sys);

    let res = program.send(2, String::from("Ivan"));
    let res2 = program.send(2, 10);
    assert!(!res.main_failed());

}
