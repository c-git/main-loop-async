use wasm_bindgen_test::wasm_bindgen_test;
use wasm_bindgen_test::wasm_bindgen_test_configure;

wasm_bindgen_test_configure!(run_in_browser);
fn main() {
    #[wasm_bindgen_test]
    async fn test_for_deadlock() -> Result<(), Box<dyn std::error::Error>> {
        let rx = main_loop_async::spawn_with_return(async || 200);

        let task_result = rx.await?; //If we can't block the calling task use try_recv instead
        assert_eq!(task_result, 200);
        Ok(())
    }
}
