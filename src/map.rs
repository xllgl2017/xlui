use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};

pub struct MapNode<T> {
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

    pub fn insert(&mut self, key: impl ToString, value: T) {
        match self.keys.get(&key.to_string()) {
            None => {
                self.keys.insert(key.to_string(), self.values.len());
                self.values.push(MapNode { key: key.to_string(), value });
            }
            Some(index) => self.values[*index] = MapNode { key: key.to_string(), value }
        }
    }
    pub fn iter(&self) -> MapIter<T> {
        MapIter { inner: self.values.iter() }
    }

    pub fn iter_mut(&mut self) -> MapIterMut<T> {
        MapIterMut { inner: self.values.iter_mut() }
    }

    pub fn entry_mut(&mut self) -> EntryMapIterMut<T> {
        EntryMapIterMut { inner: self.values.iter_mut() }
    }


    pub fn get(&self, key: impl ToString) -> Option<&T> {
        let k = key.to_string();
        let index = self.keys.get(&k)?;
        Some(&self.values.get(*index)?.value)
    }

    pub fn get_mut(&mut self, key: impl ToString) -> Option<&mut T> {
        let k = key.to_string();
        let index = self.keys.get(&k)?;
        Some(&mut self.values.get_mut(*index)?.value)
    }

    pub fn remove(&mut self, key: &String) -> Option<T> {
        let index = self.keys.remove(key)?;
        let value = self.values.remove(index).value;
        self.keys.clear();
        self.values.iter().enumerate().for_each(|(i, v)| {
            self.keys.insert(v.key.clone(), i);
        });
        Some(value)
    }

    pub fn remove_map_by_index(&mut self, index: usize) -> (String, T) {
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

    pub fn first(&self) -> Option<&T> {
        Some(&self.values.first()?.value)
    }

    pub fn last(&self) -> Option<&T> {
        Some(&self.values.last()?.value)
    }

    pub fn last_mut(&mut self) -> Option<&mut T> {
        Some(&mut self.values.last_mut()?.value)
    }

    pub fn has_key(&mut self, key: &String) -> bool {
        self.keys.contains_key(key)
    }

    pub fn position(&self, key: &String) -> Option<&usize> {
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

impl<T> Index<&String> for Map<T> {
    type Output = T;
    fn index(&self, index: &String) -> &Self::Output {
        let index = self.keys[index];
        &self.values[index].value
    }
}

impl<T> IndexMut<&String> for Map<T> {
    fn index_mut(&mut self, index: &String) -> &mut Self::Output {
        let index = self.keys[index];
        &mut self.values[index].value
    }
}

impl<T> Default for Map<T> {
    fn default() -> Self {
        Map::new()
    }
}

pub struct MapIter<'a, T> {
    inner: Iter<'a, MapNode<T>>,
}

impl<'a, T> Iterator for MapIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.inner.next()?;
        Some(&item.value)
    }
}

pub struct MapIterMut<'a, T> {
    inner: IterMut<'a, MapNode<T>>,
}

impl<'a, T> Iterator for MapIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.inner.next()?;
        Some(&mut item.value)
    }
}

pub struct EntryMapIterMut<'a, T> {
    inner: IterMut<'a, MapNode<T>>,
}

impl<'a, T> Iterator for EntryMapIterMut<'a, T> {
    type Item = (&'a String, &'a mut T);
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.inner.next()?;
        Some((&item.key, &mut item.value))
    }
}

