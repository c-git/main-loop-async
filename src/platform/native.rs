//! Stores the code specific to native compilations
// but the comments are for both because these are the ones that show on docs.rs

use tracing::error;

#[cfg(feature = "native-tokio")]
use crate::Spawnable;
use crate::SpawnableWithReturn;

#[cfg(not(feature = "native-tokio"))]
compile_error!(
    "Must chose a native runtime by enabling a feature flag. Right now only tokio is supported. If you have a different runtime that you want please create an issue on github."
);

/// Spawns a async job to run `f` (the future passed) and returns a
/// [`futures::channel::oneshot::Receiver`] which can be used to get the return
/// value from `f`.
///
/// See the examples [folder](https://github.com/c-git/reqwest-cross/tree/main/examples)
/// for more complete examples.
///
/// # Example
/// ```rust
/// # use main_loop_async::spawn_with_return;
///
/// # #[cfg(all(not(target_arch = "wasm32"),feature = "native-tokio"))]
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
///  let task = async || { "hi".to_owned() };
///  let rx = spawn_with_return(task);
///
///  let result = rx.await?; //Only an example, in a real use case use try_recv instead
///  assert_eq!(result, "hi");
/// # Ok(())
/// # }
///
/// # #[cfg(target_arch = "wasm32")]
/// # fn main(){}
/// ```
pub fn spawn_with_return<F: SpawnableWithReturn<O>, O: Spawnable>(
    f: F,
) -> futures::channel::oneshot::Receiver<<O as Future>::Output> {
    let (tx, rx) = futures::channel::oneshot::channel();
    spawn(async move {
        let result = f().await;
        let result = tx.send(result);
        if let Err(err_msg) = result {
            error!("failed to send result from `spawn_with_return`: {err_msg:?}");
        }
    });
    rx
}

/// Spawns a future on the underlying runtime in a cross platform way (NB: the
/// Send bound is removed in WASM)
#[cfg(feature = "native-tokio")]
pub fn spawn<F: Spawnable>(future: F) {
    tokio::spawn(future);
}
