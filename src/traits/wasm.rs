/// dox
pub trait Spawnable: 'static + std::future::Future<Output: 'static> {}
impl<T: 'static + std::future::Future<Output: 'static>> Spawnable for T {}

/// dox
pub trait SpawnableWithReturn<Out: Spawnable>: 'static + FnOnce() -> Out {}
impl<Out: Spawnable, T: 'static + FnOnce() -> Out> SpawnableWithReturn<Out> for T {}

/// dox
pub trait SpawnableNoReturn: 'static + std::future::Future<Output = ()> {}
impl<T: 'static + std::future::Future<Output = ()> + ?Sized> SpawnableNoReturn for T {}
