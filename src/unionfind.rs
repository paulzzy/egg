use crate::Id;
use std::fmt::Debug;

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde-1", derive(serde::Serialize, serde::Deserialize))]
pub struct UnionFind {
    parents: Vec<Id>,
}

impl UnionFind {
    pub fn make_set(&mut self) -> Id {
        let id = Id::from(self.parents.len());
        self.parents.push(id);
        id
    }

    pub fn size(&self) -> usize {
        self.parents.len()
    }

    fn parent(&self, query: Id) -> Id {
        self.parents[usize::from(query)]
    }

    fn parent_mut(&mut self, query: Id) -> &mut Id {
        &mut self.parents[usize::from(query)]
    }

    pub fn find(&self, mut current: Id) -> Id {
        while current != self.parent(current) {
            current = self.parent(current)
        }
        current
    }

    pub fn find_mut(&mut self, mut current: Id) -> Id {
        while current != self.parent(current) {
            let grandparent = self.parent(self.parent(current));
            *self.parent_mut(current) = grandparent;
            current = grandparent;
        }
        current
    }

    /// Given two leader ids, unions the two eclasses making root1 the leader.
    pub fn union(&mut self, root1: Id, root2: Id) -> Id {
        *self.parent_mut(root2) = root1;
        root1
    }

    /// TODO: Naive implementation, for a potentially more efficient one see
    /// [this paper](https://dl.acm.org/doi/10.1145/2636922), but I think it
    /// would triple the memory usage (and be hella more complicated)
    ///
    /// There's also [this cool paper](https://link.springer.com/article/10.1007/s10817-017-9431-7),
    /// although I don't think it covers deletions
    pub fn delete(&mut self, query: Id) {
        let parent = self.parent(query);

        self.parents.remove(usize::from(query));

        let mut new_root: Option<Id> = None;
        for idx in 0..self.parents.len() {
            if parent == query {
                // Deleted a root node so choose a new root for the children, if any
                if self.parents[idx] == query {
                    if new_root.is_none() {
                        new_root = Some(Id::from(idx));
                    }
                    self.parents[idx] = new_root.unwrap();
                }
            } else {
                // Deleting a non-root node
                if self.parents[idx] == query {
                    self.parents[idx] = parent;
                }
            }
            if self.parents[idx] > query {
                self.parents[idx] = Id::from(usize::from(self.parents[idx]) - 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ids(us: impl IntoIterator<Item = usize>) -> Vec<Id> {
        us.into_iter().map(|u| u.into()).collect()
    }

    #[test]
    fn union_find() {
        let n = 10;
        let id = Id::from;

        let mut uf = UnionFind::default();
        for _ in 0..n {
            uf.make_set();
        }

        // test the initial condition of everyone in their own set
        assert_eq!(uf.parents, ids(0..n));

        // build up one set
        uf.union(id(0), id(1));
        uf.union(id(0), id(2));
        uf.union(id(0), id(3));

        // build up another set
        uf.union(id(6), id(7));
        uf.union(id(6), id(8));
        uf.union(id(6), id(9));

        // this should compress all paths
        for i in 0..n {
            uf.find_mut(id(i));
        }

        // indexes:         0, 1, 2, 3, 4, 5, 6, 7, 8, 9
        let expected = vec![0, 0, 0, 0, 4, 5, 6, 6, 6, 6];
        assert_eq!(uf.parents, ids(expected));
    }

    #[test]
    fn delete() {
        let mut union_find = UnionFind::default();
        for _ in 0..10 {
            union_find.make_set();
        }

        union_find.union(Id::from(0), Id::from(1));
        union_find.union(Id::from(0), Id::from(2));
        union_find.union(Id::from(0), Id::from(3));

        union_find.union(Id::from(6), Id::from(7));
        union_find.union(Id::from(7), Id::from(8));
        union_find.union(Id::from(8), Id::from(9));

        assert_eq!(union_find.parents, ids(vec![0, 0, 0, 0, 4, 5, 6, 6, 7, 8]));

        union_find.delete(Id::from(0));
        assert_eq!(union_find.parents, ids(vec![0, 0, 0, 3, 4, 5, 5, 6, 7]));

        union_find.delete(Id::from(4));
        assert_eq!(union_find.parents, ids(vec![0, 0, 0, 3, 4, 4, 5, 6]));

        union_find.delete(Id::from(4));
        assert_eq!(union_find.parents, ids(vec![0, 0, 0, 3, 4, 4, 5]));
    }
}
