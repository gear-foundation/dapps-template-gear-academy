use gear_wasm_builder::WasmBuilder;
use io::ProgramMetadata;
use gmeta::Metadata;

fn main(){
    WasmBuilder::with_meta(ProgramMetadata::repr()).exclude_features(vec!["binary-vendor"]).build();
}