use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;

/// Subsets are used for the Naked and Hidden strategies.
/// For Hidden N (Hidden Single, Hidden Pair, ...), we must find a set of
/// N values that appear only in N positions within a unit.
/// Naked N (Naked Pair, Naked Triple, etc.) is the "reverse", we must find
/// a set of N positions where only N values appear within a unit.
/// It's the same basic search problem. In both cases we are looking for a set
/// of N subsets, the union of which contains N distinct values.
/// For lack of a better term I will call such a set "critical", by analogy
/// with algebra (underdetermined/critical/overdetermined system of equations).
pub struct Subsets<K, V>(HashMap<K, HashSet<V>>);

impl<K, V> Subsets<K, V>
where
    K: Hash + PartialEq + Eq + Clone,
    V: Hash + PartialEq + Eq + Clone,
{
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, k: K, v: V) {
        self.0.entry(k).or_insert_with(HashSet::new).insert(v);
    }

    fn rec_find_critical_sets(
        &self,
        order: usize,
        path_ks: &HashSet<K>,
        path_vs: &HashSet<V>,
    ) -> Vec<HashSet<K>> {
        if path_ks.len() == order {
            return vec![path_ks.clone()];
        }
        let mut v = vec![];
        for (k, vs) in &self.0 {
            if path_ks.contains(k) {
                continue;
            }
            let mut path_vs = path_vs.clone();
            path_vs.extend(vs.iter().cloned());
            if path_vs.len() <= order {
                let mut path_ks = path_ks.clone();
                path_ks.insert(k.clone());
                for ks in self.rec_find_critical_sets(order, &path_ks, &path_vs) {
                    if !v.contains(&ks) {
                        v.push(ks);
                    }
                }
            }
        }
        v
    }

    fn get_vs(&self, ks: &HashSet<K>) -> HashSet<V> {
        let mut vs = HashSet::new();
        for k in ks {
            if let Some(k_vs) = self.0.get(k) {
                vs.extend(k_vs.iter().cloned());
            }
        }
        vs
    }

    pub fn find_critical_sets(&self, order: usize) -> Vec<(HashSet<K>, HashSet<V>)> {
        self.rec_find_critical_sets(order, &HashSet::new(), &HashSet::new())
            .iter()
            .map(|ks| (ks.clone(), self.get_vs(ks)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use maplit::hashset;

    #[test]
    fn test_critical_set_order_1() {
        let mut ss: Subsets<u8, u8> = Subsets::new();
        ss.insert(1, 1);
        ss.insert(2, 2);
        ss.insert(2, 3);
        ss.insert(3, 3);
        let csets = ss.find_critical_sets(1);
        assert_eq!(csets.len(), 2);
        assert!(csets.contains(&(hashset![1], hashset![1])));
        assert!(csets.contains(&(hashset![3], hashset![3])));
    }

    #[test]
    fn test_critical_set_order_2() {
        let mut ss: Subsets<u8, u8> = Subsets::new();
        ss.insert(1, 1);
        ss.insert(2, 2);
        ss.insert(2, 3);
        ss.insert(3, 2);
        ss.insert(3, 3);
        let csets = ss.find_critical_sets(2);
        assert_eq!(csets.len(), 1);
        assert!(csets.contains(&(hashset![2, 3], hashset![2, 3])));
    }

    #[test]
    fn test_critical_set_order_3() {
        let mut ss: Subsets<u8, u8> = Subsets::new();
        ss.insert(1, 1);
        ss.insert(1, 2);
        ss.insert(2, 2);
        ss.insert(2, 3);
        ss.insert(3, 1);
        ss.insert(3, 3);
        ss.insert(4, 4);
        ss.insert(4, 5);
        ss.insert(5, 5);
        let csets = ss.find_critical_sets(3);
        assert_eq!(csets.len(), 1);
        assert!(csets.contains(&(hashset![1, 2, 3], hashset![1, 2, 3])));
    }
}

pub struct Order(pub usize);

impl From<Order> for usize {
    fn from(order: Order) -> Self {
        order.0
    }
}

impl From<usize> for Order {
    fn from(n: usize) -> Self {
        Order(n)
    }
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            if self.0 == 1 {
                "Single".to_string()
            } else if self.0 == 2 {
                "Pair".to_string()
            } else if self.0 == 3 {
                "Triple".to_string()
            } else if self.0 == 4 {
                "Quad".to_string()
            } else {
                format!("Order<{}>", self.0)
            }
        )
    }
}
