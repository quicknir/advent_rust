use std::hash::Hash;
use std::collections::HashMap;
use std::collections::HashSet;
    
pub trait HashIterator: Sized + Iterator {
    type H: FromIterator<Self::Item>;
}

impl<T1: Eq+Hash, T2,  T: Iterator<Item=(T1, T2)>> HashIterator for T {
    type H = HashMap<T1, T2>;
}

pub trait ResultHashIterator: Sized + Iterator {
    type H: FromIterator<Self::Item>;
}

impl<T1: Eq+Hash, T2,  E, T: Iterator<Item=Result<(T1, T2), E>>> ResultHashIterator for T {
    type H = Result<HashMap<T1, T2>, E>;
}

pub trait IteratorExts : Iterator {
    fn to_vec(self) -> Vec<Self::Item> where Self: Sized {
        self.collect::<Vec<_>>()
    }
    fn try_vec<T, E>(self) -> Result<Vec<T>, E> where Self: Sized, Result<Vec<T>, E>: FromIterator<Self::Item> {
        self.collect::<Result<Vec<T>, E>>()
    }
    fn to_hashset(self) -> HashSet<Self::Item> where Self: Sized, Self::Item: Eq+Hash {
        self.collect::<HashSet<_>>()
    }
    fn try_hashset<T: Eq+Hash, E>(self) -> Result<HashSet<T>, E> where Self: Sized, Result<HashSet<T>, E>: FromIterator<Self::Item> {
        self.collect::<Result<HashSet<_>, _>>()
    }
    fn to_hashmap(self) -> Self::H where Self: HashIterator {
        self.collect::<Self::H>()
    }
    fn try_hashmap(self) -> Self::H where Self: ResultHashIterator {
        self.collect::<Self::H>()
    }

    fn chunked_iterator(self) -> impl Iterator<Item=Vec<Self::Item>> where Self::Item : AsRef<str>, Self: Sized {
        let mut v = vec![];
        self.map(|x| Some(x)).chain(std::iter::once(None)).filter_map(move |x| {
            let Some(x) = x else { return Some(std::mem::take(&mut v)) };
            if x.as_ref() == "" {
                Some(std::mem::take(&mut v))
            }
            else {
                v.push(x);
                None
            }
        })
    }

    fn only(mut self) -> Option<Self::Item> where Self: Sized {
        let o = self.next()?;
        match self.next() {
            None => Some(o),
            Some(_) => None,
        }
    }

}

impl<T: Iterator> IteratorExts for T {}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_chunked() {
        let v = vec!["hello", "there", "", "and", "more"];
        let v2 = v.into_iter().chunked_iterator().to_vec();
        assert_eq!(v2[0], vec!["hello", "there"]);
        assert_eq!(v2[1], vec!["and", "more"]);
    }
}
