#[cfg(test)]
mod grouped_map {
    use crate::grouped_map::*;
    use std::collections::{BTreeSet, HashSet};

    #[test]
    fn grouped_hash_map() {
        let mut ghm = IndexedGroupedHashMap::default();

        ghm.add(0, 1);
        ghm.add(0, 2);
        ghm.add(1, 3);
        ghm.add(1, 4);
        ghm.add(2, 5);

        ghm.remove(&0, &1);
        ghm.remove(&1, &3);
        ghm.remove(&1, &4);
        ghm.remove(&2, &6);

        assert_eq!(ghm.0.get(&0), Some(&HashSet::from([2])));
        assert_eq!(ghm.0.get(&1), None);
        assert_eq!(ghm.0.get(&2), Some(&HashSet::from([5])));
    }

    #[test]
    fn grouped_b_tree_map() {
        let mut gbtm = IndexedGroupedBTreeMap::default();

        gbtm.add(0, 1);
        gbtm.add(0, 2);
        gbtm.add(1, 3);
        gbtm.add(1, 4);
        gbtm.add(2, 5);

        gbtm.remove(&0, &1);
        gbtm.remove(&1, &3);
        gbtm.remove(&1, &4);
        gbtm.remove(&2, &6);

        assert_eq!(gbtm.0.get(&0), Some(&BTreeSet::from([2])));
        assert_eq!(gbtm.0.get(&1), None);
        assert_eq!(gbtm.0.get(&2), Some(&BTreeSet::from([5])));
    }
}

#[cfg(test)]
mod graph {
    use crate::graph::{Graph, Node};
    use std::collections::HashSet;

    fn next(graph: &Graph<usize, usize>, src: &usize) -> Option<HashSet<(usize, usize)>> {
        if let Some(hset) = graph.next(src) {
            let result = hset
                .iter()
                .map(|&Node(p, v)| (p, v))
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
        assert_eq!(next(&graph, &1), Some(HashSet::from([(2, 10), ])));
        assert_eq!(next(&graph, &2), Some(HashSet::from([(3, 20), ])));
        assert_eq!(next(&graph, &3), Some(HashSet::from([(1, 30), (4, 60), ])));
        assert_eq!(next(&graph, &4), Some(HashSet::from([(3, 40), ])));
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
        assert_eq!(next(&graph, &1), Some(HashSet::from([(2, 10), (3, 30), ])));
        assert_eq!(next(&graph, &2), Some(HashSet::from([(1, 10), (3, 20), ])));
        assert_eq!(
            next(&graph, &3),
            Some(HashSet::from([(1, 30), (2, 20), (4, 60), ]))
        );
        assert_eq!(next(&graph, &4), Some(HashSet::from([(3, 60), ])));
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
