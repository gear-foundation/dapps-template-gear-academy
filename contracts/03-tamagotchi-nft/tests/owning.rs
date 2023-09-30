use gtest::{Program, System, Log};
use tamagotchi_nft_io::*;

// TODO: 0️⃣ Copy tests from the previous lesson and push changes to the master branch

const USER_ID: u64 = 100;
const USER_ID2: u64 = 101;
const USER_ID3: u64 = 102;
const USER_NAME: &str = "David";
const TAMAGOTCHI_ID: u64 = 1;

fn init_tamagotchi(sys: &System) {
    sys.init_logger();
    let program = Program::current(&sys);
    let res = program.send(USER_ID, String::from(USER_NAME));
    assert!(!res.main_failed());
}

#[test]
fn tamagotchi_test_init() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    let res = program.send(USER_ID, String::from("DavidHM"));

    let  expected_init_log = Log::builder()
        .dest(USER_ID)
        .payload("successful initialization!");

    assert!(res.contains(&expected_init_log));
    assert!(!res.main_failed());
}

#[test]
fn correct_name() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    let res = program.send(USER_ID, String::from("david"));
    let  expected_name_log = Log::builder()
        .dest(USER_ID)
        .payload(TmgEvent::Name(String::from("david")));
    let res = program.send(
        USER_ID,
        TmgAction::Name
    );
    assert!(!res.main_failed());
    assert!(res.contains(&expected_name_log));
}

#[test]
fn tamagotchi_feed() {
    let sys = System::new();
    init_tamagotchi(&sys);
    let program = sys.get_program(TAMAGOTCHI_ID);
    
    let  expected_name_log = Log::builder()
        .dest(USER_ID)
        .payload(TmgEvent::Fed);
    let res = program.send(
        USER_ID,
        TmgAction::Feed
    );
    assert!(!res.main_failed());
    assert!(res.contains(&expected_name_log));
}

#[test]
fn tamagotchi_entertanied() {
    let sys = System::new();
    init_tamagotchi(&sys);
    let program = sys.get_program(TAMAGOTCHI_ID);
    let  expected_name_log = Log::builder()
        .dest(USER_ID)
        .payload(TmgEvent::Entertained);
    let res = program.send(
        USER_ID,
        TmgAction::Play
    );
    assert!(!res.main_failed());
    assert!(res.contains(&expected_name_log));
}

#[test]
fn tamagotchi_slept() {
    let sys = System::new();
    init_tamagotchi(&sys);
    let program = sys.get_program(TAMAGOTCHI_ID);
    let  expected_name_log = Log::builder()
        .dest(USER_ID)
        .payload(TmgEvent::Slept);
    let res = program.send(
        USER_ID,
        TmgAction::Sleep
    );
    assert!(!res.main_failed());
    assert!(res.contains(&expected_name_log));
}

// TODO: 6️⃣ Test new functionality

#[test]
fn tamagotchi_transfer() {
    let sys = System::new();
    init_tamagotchi(&sys);
    let program = sys.get_program(TAMAGOTCHI_ID);
    let  expected_name_log = Log::builder()
        .dest(USER_ID)
        .payload(TmgEvent::Transferred(USER_ID2.into()));
    let res = program.send(
        USER_ID,
        TmgAction::Transfer(USER_ID2.into())
    );
    assert!(!res.main_failed());
    assert!(res.contains(&expected_name_log));
}

#[test]
fn tamagotchi_transfer_fail() {
    let sys = System::new();
    init_tamagotchi(&sys);
    let program = sys.get_program(TAMAGOTCHI_ID);
    let  expected_name_log = Log::builder()
        .dest(USER_ID3)
        .payload(TmgEvent::Transferred(USER_ID2.into()));
    let res = program.send(
        USER_ID2,
        TmgAction::Transfer(USER_ID2.into())
    );
    assert!(!res.main_failed());
    assert!(!res.contains(&expected_name_log));
}

#[test]
fn tamagotchi_approve() {
    let sys = System::new();
    init_tamagotchi(&sys);
    let program = sys.get_program(TAMAGOTCHI_ID);
    let  expected_name_log = Log::builder()
        .dest(USER_ID)
        .payload(TmgEvent::Approved(USER_ID2.into()));
    let res = program.send(
        USER_ID,
        TmgAction::Approve(USER_ID2.into())
    );
    assert!(!res.main_failed());
    assert!(res.contains(&expected_name_log));
}

#[test]
fn tamagotchi_approve_fail() {
    let sys = System::new();
    init_tamagotchi(&sys);
    let program = sys.get_program(TAMAGOTCHI_ID);
    let  expected_name_log = Log::builder()
        .dest(USER_ID2)
        .payload(TmgEvent::Approved(USER_ID3.into()));
    let res = program.send(
        USER_ID2,
        TmgAction::Approve(USER_ID3.into())
    );
    assert!(!res.main_failed());
    assert!(!res.contains(&expected_name_log));
}

#[test]
fn tamagotchi_revoke() {
    let sys = System::new();
    init_tamagotchi(&sys);
    let program = sys.get_program(TAMAGOTCHI_ID);
    let mut expected_name_log = Log::builder()
        .dest(USER_ID)
        .payload(TmgEvent::Approved(USER_ID2.into()));
    let mut res = program.send(
        USER_ID,
        TmgAction::Approve(USER_ID2.into())
    );
    assert!(!res.main_failed());
    assert!(res.contains(&expected_name_log));
    
    expected_name_log = Log::builder()
        .dest(USER_ID)
        .payload(TmgEvent::ApprovalRevoked);
    res = program.send(
        USER_ID,
        TmgAction::RevokeApproval
    );
    assert!(!res.main_failed());
    assert!(res.contains(&expected_name_log));
}

#[test]
fn tamagotchi_revoke_fail() {
    let sys = System::new();
    init_tamagotchi(&sys);
    let program = sys.get_program(TAMAGOTCHI_ID);
    let mut expected_name_log = Log::builder()
        .dest(USER_ID)
        .payload(TmgEvent::Approved(USER_ID2.into()));
    let mut res = program.send(
        USER_ID,
        TmgAction::Approve(USER_ID2.into())
    );
    assert!(!res.main_failed());
    assert!(res.contains(&expected_name_log));
    
    expected_name_log = Log::builder()
        .dest(USER_ID3)
        .payload(TmgEvent::ApprovalRevoked);
    res = program.send(
        USER_ID3,
        TmgAction::RevokeApproval
    );
    assert!(!res.main_failed());
    assert!(!res.contains(&expected_name_log));
}
