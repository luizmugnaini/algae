mod binary;

// TODO: Write a common test for all searchers, and write docs.
pub trait Searcher {
    fn search<T: PartialEq + PartialOrd>(x: &[T], item: T) -> Option<usize>;
}
