mod binary;

// TODO: Write a common test for all searchers, and write docs.
pub trait Searcher {
    fn search<T: PartialEq + PartialOrd>(x: &[T], item: T) -> Option<usize>;
}

/// A symbol table is a data structure composed of key-value pairs. The user
/// should be able to insert or remove key-value pairs, and also be able to
/// search for a value having a given key.
///
/// Some properties:
/// * Only one value is associated with each key.
/// * The insertion of a key-value whose key is already present in the table
///   replaces the older key-value pair with the new one.
/// * Keys should be comparable, that is, the space of keys should admit a
///   partial order.
pub trait SymbolTable<K, V> {
    fn new() -> Self;

    /// Put key-value pair into the table.
    fn put(&mut self, key: K, val: V);

    /// Get the value paired with the given key. If the key-value pair exists,
    /// returns `Some(value)`, otherwise `None`.
    fn get(&self, key: K) -> Option<V>;

    /// Eagerly deletion of the given key (and its associated value) from the
    /// table.
    fn del(&mut self, key: K);

    /// Whether the table contains a key-value pair with the matching key.
    fn contains(&self, key: K) -> bool;

    /// Whether the table is empty.
    fn is_empty(&self) -> bool;

    /// Number of key-value pairs in the table.
    fn size(&self) -> usize;

    /// Returns an iterator through all keys of the table.
    fn keys<I: Iterator>(&self) -> I;
}

/// The `OrderedSymbolTable` builds out of `SymbolTable`, in that it presents
/// additional methods for symbol tables whose keys live in a partially ordered
/// space. This also allows for faster search time.
pub trait OrderedSymbolTable<K: PartialOrd, V> {
    /// If the symbol table is non-empty, returns the minimal key in the table
    /// as `Some(min_key)`, otherwise returns `None`.
    fn min(&self) -> Option<K>;

    /// If the symbol table is non-empty, returns the maximal key in the table
    /// as `Some(max_key)`, otherwise returns `None`.
    fn max(&self) -> Option<K>;

    /// If the symbol table is non-empty, returns the largest key less than or
    /// equal to `key` as `Some(floor_key)`, otherwise returns `None`.
    fn floor(&self, key: K) -> Option<K>;

    /// If the symbol table is non-empty, returns the smallest key greater than
    /// or equal to `key` as `Some(ceil_key)`, otherwise returns `None`.
    fn ceiling(&self, key: K) -> Option<K>;

    /// Returns the total number of keys less than `key`.  If the symbol table
    /// is empty or does not contain any key less than `key`, the method
    /// will return `0`.
    fn rank(&self, key: K) -> usize;

    /// Returns a key whose rank is equal to `rank` as `Some(key)`, if no key
    /// has such rank the method returns `None`.
    ///
    /// Note that if `k1` and `k2` are keys of the table whose rank equals
    /// `rank`, the method will return the minimum key between `k1` and
    /// `k2` (which is well defined since `k1 != k2`) due to the ordering
    /// property of the table.
    fn select(&self, rank: usize) -> Option<K>;

    /// Deletes the minimal key of the table.
    fn del_min(&mut self);

    /// Deletes the maximal key of the table.
    fn del_max(&mut self);

    /// Returns an iterator through all keys that are greater or equal to `low`
    /// and less than `high`.
    fn keys_within<I: Iterator>(&self, low: K, high: K) -> I;
}

fn frequency_counter<ST>(_: ST)
where
    ST: for<'a> SymbolTable<&'a str, i32> + for<'a> OrderedSymbolTable<&'a str, i32>,
{
    let text = concat!(
        "it was the best of times it was the worst of times ",
        "it was the age of wisdom it was the age of foolishness ",
        "it was the epoch of belief it was the epoch of incredulity ",
        "it was the season of light it was the season of darkness ",
        "it was the spring of hope it was the winter of despair"
    )
    .split_whitespace();

    let mut table = ST::new();
    for w in text {
        if !table.contains(w) {
            table.put(w, 1);
        } else {
            table.put(w, table.get(w).unwrap_or(0) + 1);
        }
    }
    // TODO: Check result of `table` with respect to `HashMap`.
    todo!();
}
