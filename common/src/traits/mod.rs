pub trait Paginator {
    fn page_size(&self) -> usize;
    fn page(&self) -> usize;
}