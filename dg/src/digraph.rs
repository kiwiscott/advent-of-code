use std::collections::BTreeMap;

pub struct DirectedGraph {
    vertices: BTreeMap<u32, Vec<u32>>,
}

impl DirectedGraph {
    pub fn new() -> DirectedGraph {
        DirectedGraph {
            vertices: BTreeMap::new(),
        }
    }

    pub fn add_edge(&mut self, vertex: u32, edge: u32) {
        //Add edges must exist
        self.vertices.entry(edge).or_insert(Vec::new());
        //Add a vertex with the path to another vertex
        self.vertices.entry(vertex).or_insert(Vec::new()).push(edge);
    }

    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn num_edges(&self) -> usize {
        self.vertices.values().map(|v| v.len()).sum()
    }

    pub fn dfs(&self,vertex :u32){
        let mut visited = Vec::<u32>::new();

        let walk = |node: u32| {
            visited.push(node); 
            for i in self.vertices.entry(node){
                walk(i); 
            }
        }; 
        
        println!(visited); 

    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn has_two_edges() {
        let mut dg = DirectedGraph::new();
        dg.add_edge(1, 1);
        dg.add_edge(1, 2);
        assert_eq!(2, dg.num_edges(),"{:?}", dg.vertices);
    }
    #[test]
    fn has_two_vertices() {
        let mut dg = DirectedGraph::new();
        dg.add_edge(1, 1);
        dg.add_edge(2, 2);
        assert_eq!(2, dg.num_vertices());
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

        let mut dg = DirectedGraph::new();
        for (v, d) in &a {
            dg.add_edge(*v as u32, *d as u32);
        }

        assert_eq!(13, dg.num_vertices(), "{:?}", dg.vertices);
        assert_eq!(22, dg.num_edges(), "{:?}", dg.vertices);

    }
}
