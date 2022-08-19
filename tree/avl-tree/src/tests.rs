use super::AVLTreeMap;

fn sample() -> AVLTreeMap<u32, &'static str> {
    let mut map = AVLTreeMap::new();

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
    let mut map = sample();

    drop(&mut map);
}

#[test]
fn preorder() {
    let map = sample();
    let preorder: Vec<_> = map.preorder().collect();

    assert_eq!(
        preorder,
        vec![
            (&2, &"John"),
            (&0, &"Mary"),
            (&4, &"Peter"),
            (&6, &"Randal")
        ]
    );
}

#[test]
fn inorder() {
    let map = sample();
    let inorder: Vec<_> = map.inorder().collect();

    assert_eq!(
        inorder,
        vec![
            (&0, &"Mary"),
            (&2, &"John"),
            (&4, &"Peter"),
            (&6, &"Randal")
        ]
    );
}
