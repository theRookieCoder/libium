use std::collections::HashSet;

pub trait IterExt<T> {
    fn collect_vec(self) -> Vec<T>;

    fn collect_hashset(self) -> HashSet<T>
    where
        T: Eq + std::hash::Hash;

    /// Delimits elements of `self` with a comma and returns a single string
    fn display(self, sep: impl AsRef<str>) -> String
    where
        T: ToString;
}

impl<T, I: Iterator<Item = T>> IterExt<T> for I {
    fn collect_vec(self) -> Vec<T> {
        self.collect::<Vec<T>>()
    }

    fn collect_hashset(self) -> HashSet<T>
    where
        T: Eq + std::hash::Hash,
    {
        self.collect::<HashSet<T>>()
    }

    fn display(self, sep: impl AsRef<str>) -> String
    where
        T: ToString,
    {
        self.map(|s| ToString::to_string(&s))
            .collect_vec()
            .join(sep.as_ref())
    }
}

pub trait IterExtPositions<T> {
    /// Returns the indices of elements where `predicate` returns true
    fn positions(self, predicate: impl Fn(T) -> bool) -> impl Iterator<Item = usize>;
}

impl<T, I: Iterator<Item = (usize, T)>> IterExtPositions<T> for I {
    fn positions(self, predicate: impl Fn(T) -> bool) -> impl Iterator<Item = usize> {
        self.filter_map(move |(i, e)| if predicate(e) { Some(i) } else { None })
    }
}
