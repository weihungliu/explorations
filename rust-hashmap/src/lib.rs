use std::hash::{Hash, Hasher, SipHasher};

pub struct MapItem<T> {
    key: String,
    value: T
}

pub struct StringMap<T> {
    items: Vec<Option<MapItem<T>>>,
}

impl<T> StringMap<T> {
    pub fn new(size: usize) -> StringMap<T> {
        let mut items = Vec::with_capacity(size);

        for __ in 0..size {
            items.push(None);
        }

        StringMap {
            items: items,
        }
    }

    pub fn set(&mut self, key: &str, value: T) -> bool {
        let mut hasher = SipHasher::default();
        key.hash(&mut hasher);
        let hash = (hasher.finish() as usize) % self.items.len();

        let change = if let Some(ref item) = self.items[hash] {
            item.key == key
        } else {
            true
        };

        if change {
            self.items[hash] = Some(MapItem {
                key: key.to_owned(),
                value: value,
            });
            return true;
        } else {
            return false;
        }
    }

    pub fn get(&mut self, key: &str) -> Option<&T> {
        let mut hasher = SipHasher::default();
        key.hash(&mut hasher);
        let hash = (hasher.finish() as usize) % self.items.len();

        if let Some(ref item) = self.items[hash] {
            if item.key == key {
                return Some(&item.value);
            }
        }

        None
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut T> {
        let mut hasher = SipHasher::default();
        key.hash(&mut hasher);
        let hash = (hasher.finish() as usize) % self.items.len();

        if let Some(ref mut item) = self.items[hash] {
            if item.key == key {
                return Some(&mut item.value);
            }
        }

        None
    }

    pub fn delete(&mut self, key: &str) {
        let mut hasher = SipHasher::default();
        key.hash(&mut hasher);
        let hash = (hasher.finish() as usize) % self.items.len();

        self.items[hash] = None;
    }

    pub fn load(&self) -> f64 {
        let load = self.items.iter()
            .filter(|item| item.is_some())
            .count();

        (load as f64) / (self.items.len() as f64)
    }
}

#[test]
fn test_load() {
    let mut h: StringMap<u32> = StringMap::new(5);

    assert!(h.set("hello", 5));
    assert_eq!(h.load(), 0.2);

    h.set("world", 6);
    assert_eq!(h.load(), 0.4);

    h.set("hello", 7);
    assert_eq!(h.load(), 0.4);

    h.delete("hello");
    assert_eq!(h.load(), 0.2);
}

#[test]
fn test_overload() {
    let mut h: StringMap<u32> = StringMap::new(1);

    assert!(h.set("hello", 1));

    for i in 0..10 {
        assert!(!h.set(&i.to_string(), i));
        assert!(1.0 == h.load());
    }
}

#[test]
fn test_get_set() {
    let mut h: StringMap<u32> = StringMap::new(100);

    assert!(h.get("Not Found").is_none());

    if h.set("hello", 5) {
        assert_eq!(h.get("hello").unwrap(), &5);
    }
    if h.set("world", 100) {
        assert_eq!(h.get("world").unwrap(), &100);
    }
    if h.set("hello", 0) {
        assert_eq!(h.get("hello").unwrap(), &0);
    }
}

#[test]
fn test_everything() {
    let mut h: StringMap<u32> = StringMap::new(100);
    let mut count = 0;

    for i in 0..100 {
        let key = i.to_string();
        if h.set(&key, i) {
            assert_eq!(h.get(&key).unwrap(), &i);
            count += 1;
        }

        assert_eq!(count as f64 / 100.0, h.load());
    }

    for i in 0..100 {
        h.delete(&i.to_string());
        assert!(h.get(&i.to_string()).is_none());
    }

    assert_eq!(0.0, h.load());
}
