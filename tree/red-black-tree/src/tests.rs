use super::RBTreeMap;

fn sample() -> RBTreeMap<u32, &'static str> {
    let mut map = RBTreeMap::new();

    map.insert(0, "Mary");
    map.insert(2, "John");
    map.insert(4, "Peter");
    map.insert(6, "Randal");

    map
}

#[test]
fn insert_elt() {
    let mut map = sample();

    assert_eq!(map.insert(3, "Fox"), None);
    assert_eq!(map.insert(0, "Milly"), Some("Mary"));
}

#[test]
fn remove() {
    let mut map = sample();

    assert_eq!(map.remove(&2), Some("John"));
    assert_eq!(map.remove(&0), Some("Mary"));
    assert_eq!(map.remove(&4), Some("Peter"));
    assert_eq!(map.remove(&6), Some("Randal"));
    assert_eq!(map.remove(&7), None);
}

#[test]
fn get() {
    let map = sample();

    assert_eq!(map.get(&2), Some(&"John"));
    assert_eq!(map.get(&8), None);
}

#[test]
fn drop_tree() {
    let map = sample();

    drop(map);
}
