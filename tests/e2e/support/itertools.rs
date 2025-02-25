use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
#[cfg(doc)]
use itertools::Itertools;
use std::hash::Hash;

pub trait IteratorExtension: Iterator {
    /// Groups the iterator identically to [`Itertools::into_group_map`]
    /// but produces an [`OrderedHashMap`] instead.
    ///
    /// # Invariant
    /// This function relies on the fact that [`Iterator`]
    /// is always consumed in order.
    fn into_ordered_group_map<K, V>(self) -> OrderedHashMap<K, Vec<V>>
    where
        Self: Iterator<Item = (K, V)> + Sized,
        K: Hash + Eq,
    {
        let mut groups = OrderedHashMap::<K, Vec<V>>::default();

        for (key, value) in self {
            groups.entry(key).or_default().push(value);
        }

        groups
    }
}

impl<T: Iterator> IteratorExtension for T {}
