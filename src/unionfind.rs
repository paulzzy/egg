use crate::Id;
use std::fmt::Debug;

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde-1", derive(serde::Serialize, serde::Deserialize))]
pub struct UnionFind {
    // TODO: Oof doubling memory usage and maybe destroying the cache hit rate
    // by using Option<Id> instead of Id. Any way to use NonZeroU32 or similar?
    parents: Vec<Option<Id>>,
}

impl UnionFind {
    pub fn make_set(&mut self) -> Id {
        let id = Id::from(self.parents.len());
        self.parents.push(Some(id));
        id
    }

    pub fn size(&self) -> usize {
        self.parents.iter().filter(|node| node.is_some()).count()
    }

    fn parent(&self, query: Id) -> Option<Id> {
        self.parents[usize::from(query)]
    }

    fn parent_mut(&mut self, query: Id) -> Option<&mut Id> {
        (&mut self.parents[usize::from(query)]).as_mut()
    }

    pub fn find(&self, mut current: Id) -> Option<Id> {
        while current != self.parent(current)? {
            current = self.parent(current)?
        }
        Some(current)
    }

    pub fn find_mut(&mut self, mut current: Id) -> Option<Id> {
        while current != self.parent(current)? {
            let grandparent = self.parent(self.parent(current)?)?;
            *self.parent_mut(current)? = grandparent;
            current = grandparent;
        }
        Some(current)
    }

    /// Given two leader ids, unions the two eclasses making root1 the leader.
    pub fn union(&mut self, root1: Id, root2: Id) -> Option<Id> {
        *self.parent_mut(root2)? = root1;
        Some(root1)
    }

    /// TODO: Naive implementation, for a potentially more efficient one see
    /// [this paper](https://dl.acm.org/doi/10.1145/2636922), but I think it
    /// would triple the memory usage (and be hella more complicated)
    ///
    /// There's also [this cool paper](https://link.springer.com/article/10.1007/s10817-017-9431-7),
    /// although I don't think it covers deletions
    pub fn delete(&mut self, query: Id) {
        let parent = self.parent(query);

        self.parents[usize::from(query)] = None;

        let mut new_root: Option<Id> = None;
        for idx in 0..self.parents.len() {
            if parent == Some(query) {
                // Deleted a root node so choose a new root for the children, if any
                if self.parents[idx] == Some(query) {
                    if new_root.is_none() {
                        new_root = Some(Id::from(idx));
                    }
                    self.parents[idx] = new_root;
                }
            } else {
                // Deleting a non-root node
                if self.parents[idx] == Some(query) {
                    self.parents[idx] = parent;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ids(us: impl IntoIterator<Item = Option<usize>>) -> Vec<Option<Id>> {
        us.into_iter().map(|u| u.map(|id| id.into())).collect()
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
        assert_eq!(uf.parents, ids((0..n).map(Some)));

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
        let expected = vec![
            Some(0),
            Some(0),
            Some(0),
            Some(0),
            Some(4),
            Some(5),
            Some(6),
            Some(6),
            Some(6),
            Some(6),
        ];
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

        assert_eq!(
            union_find.parents,
            ids(vec![
                Some(0),
                Some(0),
                Some(0),
                Some(0),
                Some(4),
                Some(5),
                Some(6),
                Some(6),
                Some(7),
                Some(8)
            ])
        );

        // Deletion leaves vacant nodes to avoid changing IDs (which correspond with indices)
        // Since 0 is a root node, its children are assigned a new root (1)
        union_find.delete(Id::from(0));
        assert_eq!(
            union_find.parents,
            ids(vec![
                None,
                Some(1),
                Some(1),
                Some(1),
                Some(4),
                Some(5),
                Some(6),
                Some(6),
                Some(7),
                Some(8)
            ])
        );

        //
        union_find.delete(Id::from(4));
        assert_eq!(
            union_find.parents,
            ids(vec![
                None,
                Some(1),
                Some(1),
                Some(1),
                None,
                Some(5),
                Some(6),
                Some(6),
                Some(7),
                Some(8)
            ])
        );

        union_find.delete(Id::from(6));
        assert_eq!(
            union_find.parents,
            ids(vec![
                None,
                Some(1),
                Some(1),
                Some(1),
                None,
                Some(5),
                None,
                Some(7),
                Some(7),
                Some(8)
            ])
        );
    }
}
