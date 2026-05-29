//! Stores the wrapper functions that can be called from either native or wasm
//! code

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod wasm;

// Using * imports to bring them up to this level
#[cfg(not(target_arch = "wasm32"))]
pub use native::*;
#[cfg(target_arch = "wasm32")]
pub use wasm::*;

use crate::{Spawnable, SpawnableWithReturn};

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
pub fn spawn_with_return<F: SpawnableWithReturn<Out>, Out: Spawnable>(
    f: F,
) -> futures::channel::oneshot::Receiver<<Out as Future>::Output> {
    let (tx, rx) = futures::channel::oneshot::channel();
    spawn(async move {
        let result = f().await;
        let result = tx.send(result);
        if let Err(err_msg) = result {
            tracing::error!("failed to send result from `spawn_with_return`: {err_msg:?}");
        }
    });
    rx
}
