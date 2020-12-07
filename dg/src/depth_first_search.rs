use crate::graph::Graph;
use std::collections::HashSet;

pub struct DepthFirstSearch {
    pub g: Graph,
}

impl DepthFirstSearch {
    pub fn new(g: Graph) -> DepthFirstSearch {
        DepthFirstSearch { g }
    }

    pub fn all_paths(&self, from: u32, to: u32) -> Vec<Vec<u32>> {
        let mut all_paths = Vec::<Vec<u32>>::new();

        for conn in self.g.node_connections(from) {
            let mut path = Vec::new();
            path.push(from);
            for res in self.find_path(path, conn, to) {
                all_paths.push(res);
            }
        }
        all_paths.sort_by(|a, b| a.len().cmp(&b.len()));
        return all_paths;
    }

    fn find_path(&self, path: Vec<u32>, from: u32, to: u32) -> Vec<Vec<u32>> {
        let mut all_paths = Vec::<Vec<u32>>::new();

        if from == to {
            let mut l = path.clone();
            l.push(from);
            all_paths.push(l);
        } else {
            //Check all Children
            for conn in self.g.node_connections(from) {
                if path.contains(&conn) {
                    continue;
                }
                let mut l = path.clone();
                l.push(from);
                for res in self.find_path(l, conn, to) {
                    all_paths.push(res);
                }
            }
        }
        return all_paths;
    }

    pub fn nodes_connected(&self, from: u32, to: u32) -> bool {
        let mut visited = HashSet::<u32>::new();
        let mut to_process = Vec::new();

        to_process.push(from);

        while let Some(i) = to_process.pop() {
            if i == to{
                return true; 
            }

            if !visited.contains(&i) {
                visited.insert(i);

                for v in self.g.node_connections(i) {
                    if v == to{
                        return true; 
                    }
                    to_process.push(v);
                }
            }
        }

        visited.contains(&to)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn directed_graph_path_test() {
        let a = [
            (0, 5),
            (2, 4),
            (2, 3),
            (1, 2),
            (0, 1),
            (3, 4),
            (3, 5),
            (0, 2),
        ];

        let mut dg = Graph::new();
        for (v, d) in &a {
            dg.add_edge(*v as u32, *d as u32);
        }

        let f = DepthFirstSearch::new(dg);

        let p = f.all_paths(0, 5);
        assert_eq!(p.len(), 3, "Three Paths Expected {:?}", p);

        let mut e: Vec<u32> = vec![0, 5];
        assert!(p.contains(&e), "Missing {:?}", e);

        e = vec![0, 1, 2, 3, 5];
        assert!(p.contains(&e), "Missing {:?}", e);

        e = vec![0, 2, 3, 5];
        assert!(p.contains(&e), "Missing {:?}", e);
    }

    #[test]
    fn undirected_graph_path_test() {
        let a = [
            (0, 5),
            (2, 4),
            (2, 3),
            (1, 2),
            (0, 1),
            (3, 4),
            (3, 5),
            (0, 2),
        ];

        let mut dg = Graph::new();
        for (v, d) in &a {
            dg.add_bi_edge(*v as u32, *d as u32);
        }

        let f = DepthFirstSearch::new(dg);

        let p = f.all_paths(0, 5);
        assert_eq!(p.len(), 5, "Five Paths Expected {:?}", p);

        let mut e: Vec<u32> = vec![0, 5];
        assert!(p.contains(&e), "Missing {:?}", e);

        e = vec![0, 2, 3, 5];
        assert!(p.contains(&e), "Missing {:?}", e);

        e = vec![0, 1, 2, 3, 5];
        assert!(p.contains(&e), "Missing {:?}", e);

        e = vec![0, 2, 4, 3, 5];
        assert!(p.contains(&e), "Missing {:?}", e);

        e = vec![0, 1, 2, 4, 3, 5];
        assert!(p.contains(&e), "Missing {:?}", e);
    }

    #[test]
    fn straight_path() {
        let a = [(0, 1), (1, 2), (2, 3), (3, 4)];

        let mut dg = Graph::new();
        for (v, d) in &a {
            dg.add_edge(*v as u32, *d as u32);
        }

        let f = DepthFirstSearch::new(dg);

        let p = f.all_paths(0, 4);
        assert!(p.len() == 1, "{:?}", p);
        assert_eq!(p, [[0, 1, 2, 3, 4]], "{:?}", p);

        let p = f.all_paths(0, 6);
        assert!(p.len() == 0, "{:?}", p);

        let p = f.all_paths(1, 3);
        assert!(p.len() == 1, "{:?}", p);
        assert_eq!(p, [[1, 2, 3]], "{:?}", p);
    }
    #[test]
    fn build_graph() {
        let a = [
            (4, 2),
            (2, 3),
            (3, 2),
            (6, 0),
            (0, 1),
            (2, 0),
            (11, 12),
            (12, 9),
            (9, 10),
            (9, 11),
            (7, 9),
            (10, 12),
            (11, 4),
            (4, 3),
            (3, 5),
            (6, 8),
            (8, 6),
            (5, 4),
            (0, 5),
            (6, 4),
            (6, 9),
            (7, 6),
        ];

        let mut dg = Graph::new();
        for (v, d) in &a {
            dg.add_edge(*v as u32, *d as u32);
        }

        assert_eq!(13, dg.num_vertices(), "{:?}", dg);
        assert_eq!(22, dg.num_edges(), "{:?}", dg);

        let dfs = DepthFirstSearch::new(dg);

        assert!(!dfs.nodes_connected(1, 5), "Node should not be reachable");
        assert!(dfs.nodes_connected(2, 1), "Node should be reachable");
    }
}
