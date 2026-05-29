// Native and WASM require different main functions but after that it should be
// the same. This example shows how to do a simple fetch.

use main_loop_async::spawn_with_return;

#[cfg(all(not(target_arch = "wasm32"), feature = "native-tokio"))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    common_code().await
}

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
#[cfg(target_arch = "wasm32")]
fn main() {
    #[wasm_bindgen_test::wasm_bindgen_test]
    async fn do_fetch() -> Result<(), Box<dyn std::error::Error>> {
        common_code().await
    }
}

#[expect(
    clippy::unused_async,
    reason = "for demonstration purposes of the example"
)]
async fn double(value: i32) -> i32 {
    value * 2
}

async fn common_code() -> Result<(), Box<dyn std::error::Error>> {
    let rx = spawn_with_return(|| double(5));

    // Note the next call block this execution path (task / thread) see loop
    // examples for alternatives
    let task_result = rx.await?;
    assert_eq!(
        task_result, 10,
        "5 sent in and it should be doubled to be 10"
    );
    Ok(())
}
