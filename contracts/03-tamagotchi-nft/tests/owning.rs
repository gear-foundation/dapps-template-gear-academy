
use gtest::{Log, Program, System};
use tamagotchi_nft_io::{TmgEvent, TmgAction};
// TODO: 0️⃣ Copy tests from the previous lesson and push changes to the master branch

const USER1: u64 = 100;
const USER2: u64 = 101;

#[test]
fn owning_test() {
    let sys = System::new();
    sys.init_logger();
    let _program = Program::current(&sys);

    
    let res = _program.send(2, String::from("Init Tamagotchi"));
    
    assert!(!res.main_failed());

       // FED 
       let fed_res = _program.send(2, TmgAction::Feed);

       let log = Log::builder()
       .dest(2)
       .payload(TmgEvent::Fed(2000));
           assert!(fed_res.contains(&log));
       
    //    // ENTERTAINED
    //    let entertained_res = _program.send(2, TmgAction::Entertain);
   
    //    let log = Log::builder()
    //    .dest(2)
    //    .payload(TmgEvent::Entertained(2000));
    //        assert!(entertained_res.contains(&log));
   
    //    // SLEPT
    //    let slept_res = _program.send(2, TmgAction::Sleep);
   
    //    let log = Log::builder()
    //    .dest(2)
    //    .payload(TmgEvent::Slept(2000));
    //        assert!(slept_res.contains(&log));

    // TODO: 6️⃣ Test new functionality

    //Transfer test
    let transfer_res = _program.send(2, TmgAction::Transfer(USER2.into()));

    let log = Log::builder()
       .dest(2)
       .payload(TmgEvent::Transfer(USER2.into()));
           assert!(transfer_res.contains(&log));

    //Approve test
    let approve_res = _program.send(USER2, TmgAction::Approve(USER1.into()));       
    
    let log = Log::builder()
    .dest(101)
    .payload(TmgEvent::Approve(USER1.into()));
        assert!(approve_res.contains(&log));

    //Revoke Approval test
    let revoke_res = _program.send(USER2, TmgAction::RevokeApproval);
    assert!(!res.main_failed());
}
