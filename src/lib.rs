// dox - used as documentation for duplicate wasm functions (Uncertain if this
// will cause problems but seen this in Reqwest)

//! # main-loop-async
//!
//! This crate is an extraction of the generic parts of the functionality from
//! `reqwest-cross` and aims to make it ergonomic  to spawn async (or blocking
//! sync tasks on native) and do not want to block in the calling task/thread,
//! for example in main loop in a UI task/thread or game loop. This is achieved
//! by using callbacks (Note: Unable to spawn tasks on WASM that directly return
//! a value). This crate provides a few options to choose from and the
//! one that fits best for you depends on what you need. A good way to get an
//! idea what level of abstraction would work for you is by looking at the
//! [examples][#examples]. I would say if you're writing a larger application
//! then [DataState] can abstract away a lot of the boiler plate. If automated
//! retires are desired see [`DataStateRetry`] which exposes similar methods but
//! with retry built in.
//!
//! NOTE: At least 1 [feature flag](#feature-flags) for
//! native MUST be set to choose which runtime to use. Currently only Tokio is
//! supported but if you want to use another runtime please open an issue on
//! github and we'd be happy to add it. To communicate between the callback and
//! the caller you can use various approaches such as:
//!
//! - The helper type in this crate [DataState] see [examples
//!   folder][examples_folder]
//! - channels  (used in [examples](#examples))
//! - `Arc<Mutex<_>>`
//! - promises and so on.
//!
//!
//! # Feature Flags
#![doc = document_features::document_features!()]
//!
//! Exactly 1 of the "native-*" flags MUST be enabled to select which runtime to
//! use for native. If one of the other options needs to be used instead of
//! tokio then defaults must be disabled.
//!
//! # Sync vs Async
//!
//! This crate supports async on both native and WASM. It also provides support
//! for using threads sync tasks if using native. The sync version was added to
//! support applications that are native only but still need to call out from
//! the main loop with possibly sync tasks.
//!
//! # How to run tokio on "secondary" thread
//!
//! If you want to use the main thread for your UI and need to run
//! [tokio][tokio-url] on a "secondary" thread I found this
//! [example](https://github.com/parasyte/egui-tokio-example) helpful. I found it in this
//! [discussion](https://github.com/emilk/egui/discussions/521), which had other suggested
//! examples as well.
//!
//! [examples_folder]: https://github.com/c-git/main-loop-async/tree/main/examples

mod data_state;
mod data_state_retry;
mod platform;
mod traits;
#[cfg(feature = "yield_now")]
mod yield_;

pub use data_state::{Awaiting, CanMakeProgress, DataState, DataStateError, ErrorBounds};
pub use data_state_retry::DataStateRetry;
#[cfg(not(target_arch = "wasm32"))]
pub use platform::spawn_thread_with_return;
pub use platform::{spawn, spawn_with_return};
pub use traits::{Spawnable, SpawnableNoReturn, SpawnableWithReturn};
#[cfg(feature = "yield_now")]
pub use yield_::yield_now;

// Exported to ensure version used matches
pub use futures::channel::oneshot;
