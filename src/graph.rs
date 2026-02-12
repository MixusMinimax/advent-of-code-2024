pub use a_star::NoPathFound;
pub use a_star::a_star_rev;
use std::cmp;
use std::collections::HashMap;

pub use bfs_impl::bfs;

mod a_star {
    use std::collections::{HashMap, HashSet};
    use std::fmt::Formatter;
    use std::hash::Hash;
    use std::{error, fmt};

    #[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
    pub struct NoPathFound;

    impl fmt::Display for NoPathFound {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "no path found")
        }
    }

    impl error::Error for NoPathFound {}

    /// returns the path in reverse order because it might be needed, and it would be inefficient to
    /// reverse it twice in that case.
    pub fn a_star_rev<Node, Edge, Neighbors>(
        start: &Node,
        is_goal: impl Fn(&Node) -> bool,
        get_neighbors: impl Fn(&Node) -> Neighbors,
        heuristic: impl Fn(&Node) -> i64,
        distance: impl Fn(&Node, &Edge, &Node) -> i64,
    ) -> Result<(Vec<(Node, Edge)>, Node), NoPathFound>
    where
        Node: Clone + Eq + Hash,
        Edge: Clone,
        Neighbors: IntoIterator<Item = (Node, Edge)>,
    {
        let mut open_set = HashSet::from([start.clone()]);
        let mut came_from = HashMap::<_, (Node, Edge)>::new();
        let mut g_score = HashMap::from([(start.clone(), 0i64)]);
        let mut f_score = HashMap::from([(start.clone(), heuristic(start))]);

        while let Some(current) = open_set
            .iter()
            .min_by_key(|&s| f_score.get(s).copied().unwrap_or(i64::MAX))
        {
            if is_goal(current) {
                let mut total_path = Vec::new();
                let goal = current.clone();
                let mut current = current;
                while came_from.contains_key(current) {
                    let prev = came_from.get(current).unwrap();
                    current = &prev.0;
                    total_path.push(prev.clone());
                }
                return Ok((total_path, goal));
            }

            let current = current.clone();
            open_set.remove(&current);

            for (neighbor, edge) in get_neighbors(&current) {
                let tentative_g_score = g_score
                    .get(&current)
                    .map(|s| *s + distance(&current, &edge, &neighbor))
                    .unwrap_or(i64::MAX);
                if tentative_g_score < g_score.get(&neighbor).copied().unwrap_or(i64::MAX) {
                    came_from.insert(neighbor.clone(), (current.clone(), edge.clone()));
                    g_score.insert(neighbor.clone(), tentative_g_score);
                    let h = heuristic(&neighbor);
                    let h = if h == i64::MAX {
                        i64::MAX
                    } else {
                        tentative_g_score + h
                    };
                    f_score.insert(neighbor.clone(), h);
                    open_set.insert(neighbor);
                }
            }
        }

        Err(NoPathFound)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_a_star() {
            let points = [
                [3.4, 2.1],
                [5.0, 4.0],
                [9.0, 6.0],
                [4.0, 7.0],
                [8.0, 1.0],
                [7.0, 2.0],
            ];
            let neighbors: [&[_]; _] = [
                &[1, 5],    // 0
                &[0, 3],    // 1
                &[4, 5, 3], // 2
                &[1, 2],    // 3
                &[2, 5],    // 4
                &[0, 2, 4], // 5
            ];
            let start = 0;
            let goal = 2;
            let result = a_star_rev(
                &start,
                |n| *n == goal,
                |a| neighbors[*a].iter().map(|b| (*b, ())).collect::<Vec<_>>(),
                |a| vecmath::vec2_len(vecmath::vec2_sub(points[*a], points[goal])) as i64,
                |a, _, b| vecmath::vec2_len(vecmath::vec2_sub(points[*a], points[*b])) as i64,
            )
            .unwrap()
            .0;
            let path: Vec<usize> = result.iter().rev().map(|(n, _)| *n).chain([goal]).collect();
            assert_eq!(path, vec![0, 5, 2]);
        }
    }
}

mod bfs_impl {
    use std::collections::{HashSet, VecDeque};
    use std::hash::Hash;

    trait INode: Clone + Hash + Eq {}
    impl<T: Clone + Hash + Eq> INode for T {}

    struct BfsIter<
        Node: INode,
        IsGoal: Fn(&Node) -> bool,
        Neighbors: IntoIterator<Item = Node>,
        GetNeighbors: Fn(&Node) -> Neighbors,
    > {
        explored_set: HashSet<Node>,
        frontier: VecDeque<Node>,
        is_goal: IsGoal,
        get_neighbors: GetNeighbors,
    }

    impl<
        Node: INode,
        IsGoal: Fn(&Node) -> bool,
        Neighbors: IntoIterator<Item = Node>,
        GetNeighbors: Fn(&Node) -> Neighbors,
    > BfsIter<Node, IsGoal, Neighbors, GetNeighbors>
    {
        fn new(start: Node, is_goal: IsGoal, get_neighbors: GetNeighbors) -> Self {
            Self {
                explored_set: HashSet::from([start.clone()]),
                frontier: VecDeque::from([start]),
                is_goal,
                get_neighbors,
            }
        }
    }

    impl<
        Node: INode,
        IsGoal: Fn(&Node) -> bool,
        Neighbors: IntoIterator<Item = Node>,
        GetNeighbors: Fn(&Node) -> Neighbors,
    > Iterator for BfsIter<Node, IsGoal, Neighbors, GetNeighbors>
    {
        type Item = Node;

        fn next(&mut self) -> Option<Self::Item> {
            while let Some(current) = self.frontier.pop_front() {
                if (self.is_goal)(&current) {
                    return Some(current);
                }
                for n in (self.get_neighbors)(&current) {
                    if self.explored_set.contains(&n) {
                        continue;
                    }
                    self.explored_set.insert(n.clone());
                    self.frontier.push_back(n);
                }
            }

            None
        }
    }

    #[allow(private_bounds)]
    pub fn bfs<
        Node: INode,
        IsGoal: Fn(&Node) -> bool,
        Neighbors: IntoIterator<Item = Node>,
        GetNeighbors: Fn(&Node) -> Neighbors,
    >(
        start: Node,
        is_goal: IsGoal,
        get_neighbors: GetNeighbors,
    ) -> impl IntoIterator<Item = Node> {
        BfsIter::new(start, is_goal, get_neighbors)
    }
}

pub fn tsp(n: u16, dist: impl Fn(u16, u16) -> i32) -> i32 {
    let mut g = HashMap::new();
    for k in 0..n {
        g.insert((1u64 << k, k), dist(0, k));
    }

    for s in 2..=n - 1 {
        for sub in 0u64..(1u64 << (n - 1)) {
            let sub = sub << 1;
            if sub.count_ones() as u16 == s {
                for k in 0..n {
                    if ((1 << k) & sub) != 0 {
                        let mut result = i32::MAX;
                        for m in 0..n {
                            if m != k && ((1 << m) & sub) != 0 {
                                result = cmp::min(result, g[&(sub & !(1 << k), m)] + dist(m, k));
                            }
                        }
                        g.insert((sub, k), result);
                    }
                }
            }
        }
    }

    (1..n)
        .map(|k| g[&(((1u64 << n) - 1) & !1u64, k)] + dist(k, 0))
        .min()
        .unwrap()
}

pub fn inv_tsp(n: u16, dist: impl Fn(u16, u16) -> i32) -> i32 {
    -tsp(n, |a, b| -dist(a, b))
}
