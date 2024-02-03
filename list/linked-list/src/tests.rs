use crate::LinkedList;

#[test]
fn test_insert() {
    let mut list = LinkedList::from_iter([11, 45, 14]);
    list.insert(list.len() / 2, 13);
    list.insert(0, 0);
    list.insert(list.len(), 42);
    assert_eq!(list, LinkedList::from_iter([0, 11, 45, 13, 14, 42]));
}

#[test]
fn test_remove() {
    let mut list = LinkedList::from_iter([19, 19, 81, 0]);

    list.remove(list.len() / 2);
    assert_eq!(list, LinkedList::from_iter([19, 19, 0]));

    list.remove(0);
    assert_eq!(list, LinkedList::from_iter([19, 0]));

    list.remove(list.len() - 1);
    assert_eq!(list, LinkedList::from_iter([19]));
}

#[test]
fn test_single() {
    let mut list = LinkedList::from_iter([114514]);

    list.insert(0, 0);
    assert_eq!(list, LinkedList::from_iter([0, 114514]));
    list.pop_front();

    list.insert(1, 0);
    assert_eq!(list, LinkedList::from_iter([114514, 0]));
    list.pop_back();

    list.remove(0);
    assert!(list.is_empty());
}
