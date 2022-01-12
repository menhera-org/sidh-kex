mod utils;
extern crate sidh;
extern crate rand;

use wasm_bindgen::prelude::*;

use rand::thread_rng;
use sidh::sidh::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, sidh-kex!");
}

#[wasm_bindgen]
pub fn generate_alice_keypair() {
    let mut rng = thread_rng();
    let (alice_public, alice_secret) = generate_alice_keypair(&mut rng);
    let alice_public_bytes = alice_public.to_bytes();
    let alice_secret_bytes = alice_secret.to_bytes();
    return (alice_public_bytes, alice_secret_bytes);
}
