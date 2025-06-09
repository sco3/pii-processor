pub trait Init {
    fn init(&self) -> &Self;
    fn start(&self);
}
