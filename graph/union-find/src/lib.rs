use std::collections::BTreeMap;

pub struct UnionFind {
    child2parent: BTreeMap<u32, u32>,
    id2weight: BTreeMap<u32, usize>,
}

impl FromIterator<u32> for UnionFind {
    fn from_iter<T: IntoIterator<Item = u32>>(iter: T) -> Self {
        let mut id2weight = BTreeMap::new();

        for id in iter {
            id2weight.insert(id, 1);
        }

        Self {
            child2parent: BTreeMap::new(),
            id2weight,
        }
    }
}

impl<const N: usize> From<[u32; N]> for UnionFind {
    fn from(src: [u32; N]) -> Self {
        UnionFind::from_iter(src)
    }
}

impl UnionFind {
    pub fn find(&self, mut id: u32) -> u32 {
        while let Some(&parent) = self.child2parent.get(&id) {
            id = parent;
        }

        id
    }

    pub fn connected(&self, id1: u32, id2: u32) -> bool {
        self.find(id1) == self.find(id2)
    }

    pub fn union(&mut self, id1: u32, id2: u32) {
        let root1 = self.find(id1);
        let root2 = self.find(id2);

        if root1 == root2 {
            return;
        }

        let weight1 = self.id2weight.get(&root1).cloned().unwrap();
        let weight2 = self.id2weight.get(&root2).cloned().unwrap();

        if weight1 >= weight2 {
            self.child2parent.insert(root2, root1);
            self.id2weight.entry(root1).and_modify(|w1| *w1 += weight2);
        } else {
            self.child2parent.insert(root1, root2);
            self.id2weight.entry(root2).and_modify(|w2| *w2 += weight1);
        }
    }
}
