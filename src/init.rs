pub trait Init {
    fn init(&self) -> &Self;
    fn start(&self) -> impl std::future::Future<Output = ()> + Send;
}
