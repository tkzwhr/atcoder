mod tests;

#[allow(dead_code)]
mod num_theory {
    use std::collections::{BTreeMap, BTreeSet};

    pub struct PrimeFactorization(pub BTreeMap<usize, usize>);

    impl PrimeFactorization {
        pub fn new(mut n: usize) -> Self {
            if n < 2 {
                return PrimeFactorization(BTreeMap::new());
            }

            let primes = eratosthenes(n);
            let mut res = BTreeMap::new();

            for p in primes {
                let mut times = 0;
                while n % p == 0 {
                    times += 1;
                    n /= p;
                }
                if times > 0 {
                    res.insert(p, times);
                }
            }

            PrimeFactorization(res)
        }
    }

    pub fn eratosthenes(n: usize) -> BTreeSet<usize> {
        if n < 2 {
            return BTreeSet::new();
        }
        if n == 2 {
            return BTreeSet::from([2]);
        }
        if n == 3 {
            return BTreeSet::from([2, 3]);
        }

        let mut sieve = vec![true; n + 1];
        let mut res = BTreeSet::from([2, 3]);

        let mut check = |p: usize| {
            if sieve[p] {
                res.insert(p);

                let mut j = p * p;
                while j <= n {
                    sieve[j] = false;
                    j += p;
                }
            }
        };

        for i in 1..=(n + 1) / 6 {
            check(i * 6 - 1);
            if i * 6 + 1 <= n {
                check(i * 6 + 1);
            }
        }
        res
    }
}

#[allow(dead_code)]
mod grouped_map {
    use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
    use std::hash::Hash;

    #[derive(Default)]
    pub struct GroupedHashMap<T, U>(pub HashMap<T, HashSet<U>>);

    pub type IndexedGroupedHashMap<U> = GroupedHashMap<usize, U>;

    impl<T, U> GroupedHashMap<T, U> {
        pub fn add(&mut self, key: T, value: U)
        where
            T: Eq + Hash,
            U: Eq + Hash,
        {
            if let Some(h) = self.0.get_mut(&key) {
                h.insert(value);
            } else {
                self.0.insert(key, HashSet::from([value]));
            }
        }

        pub fn remove(&mut self, key: &T, value: &U)
        where
            T: Eq + Hash,
            U: Eq + Hash,
        {
            let mut is_empty = false;
            if let Some(h) = self.0.get_mut(&key) {
                h.remove(&value);
                is_empty = h.is_empty();
            }
            if is_empty {
                self.0.remove(&key);
            }
        }
    }

    #[derive(Default)]
    pub struct GroupedBTreeMap<T, U>(pub BTreeMap<T, BTreeSet<U>>);

    pub type IndexedGroupedBTreeMap<U> = GroupedBTreeMap<usize, U>;

    impl<T, U> GroupedBTreeMap<T, U> {
        pub fn add(&mut self, key: T, value: U)
        where
            T: Ord,
            U: Ord,
        {
            if let Some(h) = self.0.get_mut(&key) {
                h.insert(value);
            } else {
                self.0.insert(key, BTreeSet::from([value]));
            }
        }

        pub fn remove(&mut self, key: &T, value: &U)
        where
            T: Ord,
            U: Ord,
        {
            let mut is_empty = false;
            if let Some(h) = self.0.get_mut(&key) {
                h.remove(&value);
                is_empty = h.is_empty();
            }
            if is_empty {
                self.0.remove(&key);
            }
        }
    }
}

#[allow(dead_code)]
mod graph {
    use std::cmp::Reverse;
    use std::collections::{BinaryHeap, HashMap, HashSet};
    use std::hash::{Hash, Hasher};

    pub type IndexedGraph<U> = Graph<usize, U>;

    pub struct Graph<T, U> {
        data: HashMap<T, HashSet<Node<T, U>>>,
    }

    impl<T, U> Graph<T, U> {
        pub fn new() -> Graph<T, U> {
            Graph {
                data: HashMap::new(),
            }
        }

        pub fn joint(&mut self, src: T, dest: T, data: U, bidirectional: bool)
        where
            T: Hash + Eq + Clone,
            U: Clone,
        {
            if bidirectional {
                self.register(dest.clone(), src.clone(), data.clone());
            }

            self.register(src, dest, data);
        }

        fn register(&mut self, key: T, value: T, data: U)
        where
            T: Hash + Eq + Clone,
            U: Clone,
        {
            if let Some(values) = self.data.get_mut(&key) {
                values.replace(Node(value, data));
            } else {
                let mut values = HashSet::new();
                values.insert(Node(value, data));
                self.data.insert(key, values);
            }
        }

        pub fn next(&self, key: &T) -> Option<&HashSet<Node<T, U>>>
        where
            T: Hash + Eq,
        {
            self.data.get(key)
        }
    }

    pub struct Node<T, U>(pub T, pub U);

    impl<T, U> Hash for Node<T, U>
    where
        T: Hash,
    {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state)
        }
    }

    impl<T, U> PartialEq for Node<T, U>
    where
        T: PartialEq,
    {
        fn eq(&self, other: &Self) -> bool {
            self.0.eq(&other.0)
        }
    }

    impl<T, U> Eq for Node<T, U> where T: PartialEq {}

    impl<T> Graph<T, usize>
    where
        T: Hash + Eq + Ord + Clone,
    {
        pub fn solve_by_dijkstra(&self, start: T) -> HashMap<T, usize> {
            let mut dp: HashMap<T, usize> = HashMap::new();
            dp.insert(start.clone(), 0);

            let mut p_queue = BinaryHeap::new();
            p_queue.push(Reverse((0, start)));

            let mut reached = HashSet::new();

            while let Some(Reverse((weight, pos))) = p_queue.pop() {
                if reached.contains(&pos) {
                    continue;
                }

                if let Some(next) = self.next(&pos) {
                    for Node(p, w) in next {
                        let new_weight = *w + weight;
                        if *dp.get(p).unwrap_or(&usize::MAX) > new_weight {
                            dp.insert(p.clone(), new_weight);
                            p_queue.push(Reverse((new_weight, p.clone())));
                        }
                    }
                }

                reached.insert(pos.clone());
            }

            dp
        }
    }
}
