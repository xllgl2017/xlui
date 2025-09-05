use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};

pub struct MapNode<K, V> {
    key: K,
    value: V,
}


pub struct Map<K, V> {
    keys: HashMap<K, usize>,
    values: Vec<MapNode<K, V>>,
}

impl<K: Clone + Eq + Hash, V> Map<K, V> {
    pub fn new() -> Map<K, V> {
        Map {
            keys: HashMap::new(),
            values: vec![],
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        match self.keys.get(&key) {
            None => {
                self.keys.insert(key.clone(), self.values.len());
                self.values.push(MapNode { key, value });
            }
            Some(index) => self.values[*index] = MapNode { key, value }
        }
    }
    pub fn iter(&self) -> MapIter<K, V> {
        MapIter { inner: self.values.iter() }
    }

    pub fn iter_mut(&mut self) -> MapIterMut<K, V> {
        MapIterMut { inner: self.values.iter_mut() }
    }

    pub fn entry_mut(&mut self) -> EntryMapIterMut<K, V> {
        EntryMapIterMut { inner: self.values.iter_mut() }
    }


    pub fn get(&self, key: &K) -> Option<&V> {
        let index = self.keys.get(key)?;
        Some(&self.values.get(*index)?.value)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let index = self.keys.get(key)?;
        Some(&mut self.values.get_mut(*index)?.value)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let index = self.keys.remove(key)?;
        let value = self.values.remove(index).value;
        self.keys.clear();
        self.values.iter().enumerate().for_each(|(i, v)| {
            self.keys.insert(v.key.clone(), i);
        });
        Some(value)
    }

    pub fn remove_map_by_index(&mut self, index: usize) -> (K, V) {
        let res = self.values.remove(index);
        self.keys.remove(&res.key);
        self.keys.clear();
        self.values.iter().enumerate().for_each(|(i, v)| {
            self.keys.insert(v.key.clone(), i);
        });
        (res.key, res.value)
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn first(&self) -> Option<&V> {
        Some(&self.values.first()?.value)
    }

    pub fn last(&self) -> Option<&V> {
        Some(&self.values.last()?.value)
    }

    pub fn last_mut(&mut self) -> Option<&mut V> {
        Some(&mut self.values.last_mut()?.value)
    }

    pub fn has_key(&mut self, key: &K) -> bool {
        self.keys.contains_key(key)
    }

    pub fn position(&self, key: &K) -> Option<&usize> {
        self.keys.get(key)
    }

    pub fn clear(&mut self) {
        self.keys.clear();
        self.values.clear();
    }

    pub fn reverse(&mut self) {
        self.keys.clear();
        self.values.reverse();
        self.values.iter().enumerate().for_each(|(i, x)| { self.keys.insert(x.key.clone(), i); });
    }
}

impl<K, V> Index<usize> for Map<K, V> {
    type Output = V;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index].value
    }
}

impl<K, V> IndexMut<usize> for Map<K, V> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index].value
    }
}

impl<K: Eq + Hash, V> Index<&K> for Map<K, V> {
    type Output = V;
    fn index(&self, index: &K) -> &Self::Output {
        let index = self.keys[index];
        &self.values[index].value
    }
}

impl<K: Eq + Hash, V> IndexMut<&K> for Map<K, V> {
    fn index_mut(&mut self, index: &K) -> &mut Self::Output {
        let index = self.keys[index];
        &mut self.values[index].value
    }
}

impl<K: Clone + Eq + Hash, V> Default for Map<K, V> {
    fn default() -> Self {
        Map::new()
    }
}

pub struct MapIter<'a, K, V> {
    inner: Iter<'a, MapNode<K, V>>,
}

impl<'a, K, V> Iterator for MapIter<'a, K, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.inner.next()?;
        Some(&item.value)
    }
}

pub struct MapIterMut<'a, K, V> {
    inner: IterMut<'a, MapNode<K, V>>,
}

impl<'a, K, V> Iterator for MapIterMut<'a, K, V> {
    type Item = &'a mut V;
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.inner.next()?;
        Some(&mut item.value)
    }
}

pub struct EntryMapIterMut<'a, K, V> {
    inner: IterMut<'a, MapNode<K, V>>,
}

impl<'a, K, V> Iterator for EntryMapIterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.inner.next()?;
        Some((&item.key, &mut item.value))
    }
}

