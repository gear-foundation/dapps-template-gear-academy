use gstd::ext::debug;
use gtest::{Log ,Program, System};
use tamagotchi_interaction_io::{TmgEvent, TmgAction};


// TODO: 0️⃣ Copy tests from the previous lesson and push changes to the master branch

#[test]
fn interaction_test() {
    //init
    let sys = System::new();
    sys.init_logger();
    let _program = Program::current(&sys);

    let res = _program.send(2, String::from("Init Tamagotchi"));
    
        assert!(!res.main_failed());

    // TODO: 6️⃣ Test new functionality
    // FED 
    let fed_res = _program.send(2, TmgAction::Feed);

    let log = Log::builder()
    .dest(2)
    .payload(TmgEvent::Fed(2000));
        assert!(fed_res.contains(&log));
    
    // ENTERTAINED
    let entertained_res = _program.send(2, TmgAction::Entertain);

    let log = Log::builder()
    .dest(2)
    .payload(TmgEvent::Entertained(2000));
        assert!(entertained_res.contains(&log));

    // SLEPT
    let slept_res = _program.send(2, TmgAction::Sleep);

    let log = Log::builder()
    .dest(2)
    .payload(TmgEvent::Slept(2000));
        assert!(slept_res.contains(&log));

}
