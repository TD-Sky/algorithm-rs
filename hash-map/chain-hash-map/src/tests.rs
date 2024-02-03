use crate::ChainHashMap;
use std::mem;

fn sample() -> ChainHashMap<String, u32> {
    let mut map: ChainHashMap<String, u32> = ChainHashMap::new();

    map.insert("C".to_owned(), 50);
    map.insert("Rust".to_owned(), 12);
    map.insert("Javascript".to_owned(), 27);

    map
}

#[test]
fn insert() {
    let mut map = sample();

    assert_eq!(map.insert("Java".to_owned(), 27), None);
    assert_eq!(map.insert("Javascript".to_owned(), 114514), Some(27));
}

#[test]
fn remove() {
    let mut map = sample();

    assert_eq!(map.remove("C"), Some(50));
    assert_eq!(map.remove("C++"), None);
}

#[test]
fn get() {
    let map = sample();

    assert_eq!(map.get("Rust"), Some(&12));
    assert_eq!(map.get("Javascript"), Some(&27));
    assert_eq!(map.get("C"), Some(&50));
}

#[test]
fn update() {
    let mut map = sample();
    let age = map.get_mut("Rust").unwrap();

    assert_eq!(mem::replace(age, 1919810), 12);
    assert_eq!(map.get("Rust"), Some(&1919810));
}
