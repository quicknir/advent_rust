use std::{marker::PhantomData, num::NonZeroUsize};

#[derive(Debug, Clone, Copy)]
struct Link(NonZeroUsize);

impl Link {
    fn new(u: usize) -> Self {
        Link(unsafe { NonZeroUsize::new_unchecked(u + 1) })
    }

    fn get(self) -> usize {
        self.0.get() - 1
    }
}

#[derive(Debug)]
enum Entry<T> {
    Data(T),
    FreeNode(Option<Link>),
}

pub struct SlotKey<T> {
    index: usize,
    phantom: PhantomData<T>,
}

pub struct SlotMap<T> {
    entries: Vec<Entry<T>>,
    free_list: Option<Link>,
}

impl<T> SlotMap<T> {
    pub fn new() -> Self {
        SlotMap {
            entries: Default::default(),
            free_list: Default::default(),
        }
    }

    pub fn insert(&mut self, t: T) -> SlotKey<T> {
        if let Some(link) = self.free_list {
            let Entry::FreeNode(next_free) = self.entries[link.get()] else {
                unreachable!()
            };
            self.free_list = next_free;
            self.entries[link.get()] = Entry::Data(t);
            SlotKey {
                index: link.get(),
                phantom: PhantomData,
            }
        } else {
            self.entries.push(Entry::Data(t));
            SlotKey {
                index: self.entries.len() - 1,
                phantom: PhantomData,
            }
        }
    }

    pub fn take(&mut self, k: SlotKey<T>) -> Option<T> {
        if let Entry::FreeNode(_) = &self.entries[k.index] {
            return None;
        }
        let e = std::mem::replace(&mut self.entries[k.index], Entry::FreeNode(self.free_list));
        self.free_list = Some(Link::new(k.index));
        if let Entry::Data(t) = e {
            Some(t)
        } else {
            unreachable!()
        }
    }
    pub fn remove(&mut self, k: SlotKey<T>) -> bool {
        if let Entry::FreeNode(_) = &self.entries[k.index] {
            return false;
        }
        self.entries[k.index] = Entry::FreeNode(self.free_list);
        self.free_list = Some(Link::new(k.index));
        true
    }
    pub fn get(&self, k: SlotKey<T>) -> Option<&T> {
        match &self.entries[k.index] {
            Entry::Data(t) => Some(t),
            Entry::FreeNode(_) => None,
        }
    }
    pub fn get_mut(&mut self, k: SlotKey<T>) -> Option<&mut T> {
        match &mut self.entries[k.index] {
            Entry::Data(t) => Some(t),
            Entry::FreeNode(_) => None,
        }
    }
    pub fn get_many_mut<const N: usize>(&mut self, _ks: [SlotKey<T>; N]) -> Option<[&mut T; N]> {
        todo!()
    }
}
