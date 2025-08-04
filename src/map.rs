use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};

struct MapNode<T> {
    key: String,
    value: T,
}


pub struct Map<T> {
    keys: HashMap<String, usize>,
    values: Vec<MapNode<T>>,
}

impl<T> Map<T> {
    pub fn new() -> Map<T> {
        Map {
            keys: HashMap::new(),
            values: vec![],
        }
    }
    pub fn iter(&self) -> MapIter<T> {
        MapIter { inner: self.values.iter() }
    }

    pub fn iter_mut(&mut self) -> MapIterMut<T> {
        MapIterMut { inner: self.values.iter_mut() }
    }

    pub fn insert(&mut self, key: impl ToString, value: T) {
        self.keys.insert(key.to_string(), self.values.len());
        self.values.push(MapNode { key: key.to_string(), value });
    }

    pub fn get(&mut self, key: impl ToString) -> Option<&T> {
        let k = key.to_string();
        let index = self.keys.get(&k)?;
        Some(&self.values.get(*index)?.value)
    }

    pub fn get_mut(&mut self, key: impl ToString) -> Option<&mut T> {
        let k = key.to_string();
        let index = self.keys.get(&k)?;
        Some(&mut self.values.get_mut(*index)?.value)
    }

    pub fn remove(&mut self, key: impl ToString) -> Option<T> {
        let k = key.to_string();
        let index = self.keys.remove(&k)?;
        Some(self.values.remove(index).value)
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}

impl<T> Index<usize> for Map<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index].value
    }
}

impl<T> IndexMut<usize> for Map<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index].value
    }
}

impl<T> Index<String> for Map<T> {
    type Output = T;

    fn index(&self, index: String) -> &Self::Output {
        let index = self.keys[&index];
        &self.values[index].value
    }
}

impl<T> IndexMut<String> for Map<T> {
    fn index_mut(&mut self, index: String) -> &mut Self::Output {
        let index = self.keys[&index];
        &mut self.values[index].value
    }
}

pub struct MapIter<'a, T> {
    inner: Iter<'a, MapNode<T>>,
}

impl<'a, T> Iterator for MapIter<'a, T> {
    type Item = (&'a String, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.inner.next()?;
        Some((&item.key, &item.value))
    }
}

pub struct MapIterMut<'a, T> {
    inner: IterMut<'a, MapNode<T>>,
}

impl<'a, T> Iterator for MapIterMut<'a, T> {
    type Item = (&'a String, &'a mut T);
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.inner.next()?;
        Some((&item.key, &mut item.value))
    }
}

