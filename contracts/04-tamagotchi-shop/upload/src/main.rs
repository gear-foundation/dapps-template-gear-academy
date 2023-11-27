use gclient::{
    errors::{Gear, ModuleError},
    Error, GearApi, Result,
};
use gear_core::ids::CodeId;
use gsdk::gp::Encode;
use sharded_fungible_token_io::InitFToken;
use std::fs;

const FT_STORAGE_WASM: &str = "sharded_fungible_token_storage";
const FT_LOGIC_WASM: &str = "sharded_fungible_token_logic";
const FT_MAIN_WASM: &str = "sharded_fungible_token";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let api = GearApi::init_with(
        gclient::WSAddress::new("wss://testnet.vara.rs", Some(443)),
        "//Alice",
    )
    .await?;
    // To run local node use:
    // let api = GearApi::dev_from_path("gear").await?;
    println!("Connected to the node\n");

    let mut listener = api.subscribe().await?;

    assert!(listener.blocks_running().await?);
    println!("Network is live\n");

    println!("Uploading 'FT Storage' code");
    let ft_storage_code_id = upload_code(&api, FT_STORAGE_WASM).await?;

    println!("Uploading 'FT Logic' code");
    let ft_logic_code_id = upload_code(&api, FT_LOGIC_WASM).await?;

    println!("Uploading 'FT Main' code");
    let ft_main_code_id = upload_code(&api, FT_MAIN_WASM).await?;

    let payload = InitFToken {
        storage_code_hash: ft_storage_code_id.into_bytes().into(),
        ft_logic_code_hash: ft_logic_code_id.into_bytes().into(),
    };

    let user_id_bytes: [u8; 32] = api.account_id().clone().into();
    let user_id = user_id_bytes.into();

    let gas_info = api
        .calculate_create_gas(Some(user_id), ft_main_code_id, vec![], 0, false)
        .await?;
    println!("Calculated gas: {}", gas_info.min_limit);

    let (_, ft_main_prog_id, _) = api
        .create_program(
            ft_main_code_id,
            b"salt",
            payload.encode(),
            2 * gas_info.min_limit,
            0,
        )
        .await?;
    println!("'FT Main' program created, ID: {ft_main_prog_id}\n");

    Ok(())
}

async fn upload_code(api: &GearApi, file_name: &'static str) -> Result<CodeId> {
    let code = fs::read(format!(
        "target/wasm32-unknown-unknown/debug/{file_name}.opt.wasm"
    ))?;
    let res = api.upload_code(&code).await;
    let ft_storage_code_id = CodeId::generate(&code);
    let code_id = match res {
        Err(Error::Module(ModuleError::Gear(Gear::CodeAlreadyExists))) => {
            println!("    Code already exists, skipping upload");
            ft_storage_code_id
        }
        Ok((code_id, _)) => {
            assert_eq!(code_id, ft_storage_code_id);
            println!("    Code uploaded");
            code_id
        }
        Err(e) => {
            return Err(e);
        }
    };

    println!("    Code ID: {code_id}\n");

    Ok(code_id)
}
