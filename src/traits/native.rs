use std::fmt::Debug;
/// An async function that is able to be spawned (Not directly able to be
/// spawned on WASM because of the return type)
pub trait Spawnable: 'static + Send + std::future::Future<Output: Send + 'static + Debug> {}
impl<T: 'static + Send + std::future::Future<Output: Send + 'static + Debug>> Spawnable for T {}

/// An async func that accepts a generic argument
/// and returns a generic value
pub trait SpawnableWithReturn<Out: Spawnable>: 'static + Send + FnOnce() -> Out {}
impl<Out: Spawnable, T: 'static + Send + FnOnce() -> Out> SpawnableWithReturn<Out> for T {}

/// An async function that is able to be spawned, lowest common denominator as
/// we cannot have a return type on WASM
pub trait SpawnableNoReturn: 'static + Send + std::future::Future<Output = ()> {}
impl<T: 'static + Send + std::future::Future<Output = ()> + ?Sized> SpawnableNoReturn for T {}
