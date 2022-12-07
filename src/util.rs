use std::{collections::HashMap, hash::Hash};

pub struct Tree<K, V> {
    pub value: V,
    pub children: HashMap<K, Tree<K, V>>,
}

impl<K, V> Tree<K, V> {
    pub fn fold<S, F, G>(&self, init: S, combine: &mut F, add_value: &mut G) -> S
    where
        S: Copy,
        F: FnMut(S, S) -> S,
        G: FnMut(S, &V) -> S,
    {
        let mut result = init;
        for child in self.children.values() {
            let child_result = child.fold(init, combine, add_value);
            result = combine(result, child_result);
        }
        add_value(result, &self.value)
    }
}

impl<K, V: Default> Default for Tree<K, V> {
    fn default() -> Self {
        Self {
            value: Default::default(),
            children: Default::default(),
        }
    }
}

pub struct TreeZipper<K, V> {
    pub cursor: Tree<K, V>,
    pub parent: Option<Box<(K, TreeZipper<K, V>)>>,
}

impl<K, V> TreeZipper<K, V> {
    pub fn new(root: Tree<K, V>) -> Self {
        Self {
            cursor: root,
            parent: None,
        }
    }

    pub fn pop(&mut self)
    where
        K: Eq + Hash,
    {
        let (key, mut parent) = *self.parent.take().unwrap();
        std::mem::swap(
            &mut self.cursor,
            parent.cursor.children.get_mut(&key).unwrap(),
        );
        *self = parent;
    }

    pub fn push(&mut self, key: K)
    where
        K: Eq + Hash + Clone,
        V: Default,
    {
        let mut new = TreeZipper::new(Tree::default());
        std::mem::swap(
            &mut new.cursor,
            self.cursor.children.entry(key.clone()).or_default(),
        );
        std::mem::swap(&mut new, self);
        self.parent = Some(Box::new((key, new)));
    }

    pub fn root(mut self) -> Tree<K, V>
    where
        K: Eq + Hash,
        V: Default,
    {
        while self.parent.is_some() {
            self.pop();
        }
        self.cursor
    }
}
