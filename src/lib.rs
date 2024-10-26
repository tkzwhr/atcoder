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
            Graph { data: HashMap::new() }
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

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::graph::{Graph, Node};

    fn next(graph: &Graph<usize, usize>, src: &usize) -> Option<HashSet<(usize, usize)>> {
        if let Some(hset) = graph.next(src) {
            let result = hset.iter()
                .map(|&Node(p, v)| {
                    (p, v)
                })
                .collect::<HashSet<_, _>>();
            Some(result)
        } else {
            None
        }
    }

    #[test]
    fn test_unidirectional_joint() {
        // Arrange
        let mut graph = Graph::new();

        // Act
        graph.joint(1, 2, 10, false);
        graph.joint(2, 3, 20, false);
        graph.joint(3, 1, 30, false);
        graph.joint(4, 3, 40, false);
        graph.joint(3, 4, 50, false);
        graph.joint(3, 4, 60, false);

        // Assert
        assert_eq!(next(&graph, &1), Some(HashSet::from([
            (2, 10),
        ])));
        assert_eq!(next(&graph, &2), Some(HashSet::from([
            (3, 20),
        ])));
        assert_eq!(next(&graph, &3), Some(HashSet::from([
            (1, 30),
            (4, 60),
        ])));
        assert_eq!(next(&graph, &4), Some(HashSet::from([
            (3, 40),
        ])));
    }

    #[test]
    fn test_bidirectional_joint() {
        // Arrange
        let mut graph = Graph::new();

        // Act
        graph.joint(1, 2, 10, true);
        graph.joint(2, 3, 20, true);
        graph.joint(3, 1, 30, true);
        graph.joint(4, 3, 40, true);
        graph.joint(3, 4, 60, true);

        // Assert
        assert_eq!(next(&graph, &1), Some(HashSet::from([
            (2, 10),
            (3, 30),
        ])));
        assert_eq!(next(&graph, &2), Some(HashSet::from([
            (1, 10),
            (3, 20),
        ])));
        assert_eq!(next(&graph, &3), Some(HashSet::from([
            (1, 30),
            (2, 20),
            (4, 60),
        ])));
        assert_eq!(next(&graph, &4), Some(HashSet::from([
            (3, 60),
        ])));
    }

    #[test]
    fn test_joint_performance() {
        // Arrange
        let mut graph = Graph::new();

        // Act
        for i in 0..200_000 {
            graph.joint(i, i, i, true);
        }
    }

    #[test]
    fn test_solve_by_dijkstra() {
        // Arrange
        let mut graph: Graph<usize, usize> = Graph::new();
        graph.joint(1, 2, 15, true);
        graph.joint(1, 4, 20, true);
        graph.joint(2, 3, 65, true);
        graph.joint(2, 5, 4, true);
        graph.joint(3, 6, 50, true);
        graph.joint(4, 5, 30, true);
        graph.joint(5, 6, 8, true);

        // Act
        let result = graph.solve_by_dijkstra(1);

        // Assert
        dbg!(&result);
        assert_eq!(result.get(&1), Some(&0));
        assert_eq!(result.get(&2), Some(&15));
        assert_eq!(result.get(&3), Some(&77));
        assert_eq!(result.get(&4), Some(&20));
        assert_eq!(result.get(&5), Some(&19));
        assert_eq!(result.get(&6), Some(&27));
    }

    #[test]
    fn test_solve_by_dijkstra_performance() {
        // Arrange
        let mut graph = Graph::new();

        // Act
        for i in 1..200_000 {
            graph.joint(i, i + 1, i, true);
        }
        let result = graph.solve_by_dijkstra(1);

        // Assert
        assert_eq!(result.get(&200_000), Some(&19_999_900_000));
    }
}