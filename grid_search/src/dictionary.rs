use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::collections::hash_map::Keys;
use std::collections::HashMap;
use std::hash::Hash;

/// crates.io contains a number of tries and various text automata things.
/// I feel like its inconcievable the thing I want isn't already up there
/// somewhere; but I couldn't find it.
///
/// What I want is this:
///   - a multi-tree edge-keyed on a sequence - probably of characters
///   - the ability to "prune" WHOLE subtrees, or better yet:
///       choose subtrees to consider from a known possible set
///   - the ability to enumerate the whole thing (after pruning or activation)

#[derive(Debug, Serialize, Deserialize)]
pub struct SproutableTrie<K: Eq + Hash, V> {
    value: Option<V>,
    children: HashMap<K, SproutableTrie<K, V>>,
}

impl<K: Eq + Hash, V> SproutableTrie<K, V> {
    pub fn new() -> Self {
        Self {
            value: None,
            children: HashMap::new(),
        }
    }

    pub fn insert<I, Ki>(&mut self, ks: I, v: V)
    where
        I: IntoIterator<Item = Ki>,
        Ki: ToOwned<Owned = K> + Borrow<K>,
    {
        let mut p = self;
        for k in ks {
            p = if p.children.contains_key(k.borrow()) {
                p.children.get_mut(k.borrow()).unwrap()
            } else {
                p.children.insert(k.to_owned(), Self::new());
                p.children.get_mut(k.borrow()).unwrap()
            }
        }
        p.value = Some(v);
    }
}

#[derive(Clone)]
pub struct SproutedTrie<'a, K: Hash + Eq, V> {
    keys: Vec<&'a K>,
    node: &'a SproutableTrie<K, V>,

    active_children: HashMap<&'a K, SproutedTrie<'a, K, V>>,
}

impl<'a, K: Hash + Eq, V> SproutedTrie<'a, K, V> {
    pub fn new(source_root: &'a SproutableTrie<K, V>) -> Self {
        Self {
            keys: vec![],
            node: source_root,
            active_children: HashMap::new(),
        }
    }

    pub fn avaliable_seeds(&self) -> Keys<K, SproutableTrie<K, V>> {
        self.node.children.keys()
    }

    pub fn sprout(&mut self, k: &'a K) -> &mut Self {
        let seed = self.node.children.get(k);
        if seed.is_none() {
            return self;
        }
        let mut seedling = Self::new(seed.unwrap());
        seedling.keys = self.keys.clone();
        seedling.keys.push(k);
        self.active_children.insert(k, seedling);
        self.active_children.get_mut(k).unwrap()
    }

    pub fn flatten(self) -> Vec<(Vec<&'a K>, &'a V)> {
        let mut this = if self.node.value.is_some() {
            vec![(self.keys, self.node.value.as_ref().unwrap())]
        } else {
            vec![]
        };
        if self.active_children.is_empty() {
            this
        } else {
            this.extend(
                self.active_children
                    .into_iter()
                    .flat_map(|(_, c)| c.flatten()),
            );
            this
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, iter::FromIterator};

    use super::*;

    #[test]
    fn sproutable() {
        let mut s: SproutableTrie<char, bool> = SproutableTrie::new();
        s.insert("all".chars(), true);
        s.insert("altitude".chars(), true);
        s.insert("alpi".chars(), true);
        s.insert("alpine".chars(), false);
        s.insert("alpined".chars(), true);

        let mut st = SproutedTrie::new(&s);
        let mut stp = &mut st;
        assert_eq!(st.avaliable_seeds().collect::<Vec<&char>>(), vec![&'a']);
        stp = st.sprout(&'a');
        assert_eq!(stp.avaliable_seeds().collect::<Vec<&char>>(), vec![&'l']);
        stp = stp.sprout(&'l');
        assert_eq!(
            (stp.avaliable_seeds().collect::<HashSet<&char>>()),
            HashSet::from_iter(vec![&'l', &'t', &'p'].into_iter())
        );
        stp.sprout(&'l');

        let stp = stp.sprout(&'p');
        let stp = stp.sprout(&'i');
        let stp = stp.sprout(&'n');
        let stp = stp.sprout(&'e');
        stp.sprout(&'d');

        assert_eq!(
            st.flatten()
                .into_iter()
                .collect::<HashSet<(Vec<&char>, &bool)>>(),
            HashSet::from_iter(
                vec![
                    (vec![&'a', &'l', &'l'], &true),
                    (vec![&'a', &'l', &'p', &'i'], &true),
                    (vec![&'a', &'l', &'p', &'i', &'n', &'e'], &false),
                    (vec![&'a', &'l', &'p', &'i', &'n', &'e', &'d'], &true),
                ]
                .into_iter()
            )
        )
    }
}
