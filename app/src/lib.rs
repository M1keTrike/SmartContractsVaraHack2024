#![no_std]

pub mod services;
pub mod states;

use gear_wasm_builder::WasmBuilder; 

fn main() {
    WasmBuilder::with_meta(states::state::AuctionMetadata::repr())
        .exclude_features(["binary-vendor"])
        .build();
}
