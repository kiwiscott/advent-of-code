use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug)]
pub struct Graph {
    pub vertices: BTreeMap<u32, Vec<u32>>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            vertices: BTreeMap::new(),
        }
    }

    //Bidirectional Edge
    pub fn add_bi_edge(&mut self, vertex: u32, edge: u32) {
        self.add_new_edge(vertex, edge, true);
    }

    //Unidirectional Edge
    pub fn add_edge(&mut self, vertex: u32, edge: u32) {
        self.add_new_edge(vertex, edge, false);
    }

    fn add_new_edge(&mut self, vertex: u32, edge: u32, bidirectional: bool) {
        //Add edges must exist
        if bidirectional {
            self.vertices.entry(edge).or_insert(Vec::new()).push(vertex)
        } else {
            self.vertices.entry(edge).or_insert(Vec::new());
        }
        //Add a vertex with the path to another vertex
        self.vertices.entry(vertex).or_insert(Vec::new()).push(edge);

        if vertex == 330983 || vertex ==  364276 ||edge == 330983 || edge ==  364276
        {
            println!("v:{:?} e:{:?}",vertex,edge);
        }
    }

    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn num_edges(&self) -> usize {
        self.vertices.values().map(|v| v.len()).sum()
    }

    pub fn nodes(&self) -> Vec<u32> {
        self.vertices.keys().cloned().collect()
    }
    pub fn node_connections(&self, node: u32) -> Vec<u32> {
        match self.vertices.get(&node) {
            Some(v) => v.iter().cloned().collect(),
            None => vec![],
        }
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut str = "";

        fmt.write_fmt(format_args!(
            "nodes[{:?}] connections[{:?}] ",
            self.num_vertices(),
            self.num_edges()
        ))?;

        for (node, conn) in &self.vertices {
            fmt.write_str(str)?;
            fmt.write_fmt(format_args!("(n:{:?} c:{:?})", node, conn))?;
            str = ", ";
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn has_two_edges() {
        let mut dg = Graph::new();
        dg.add_edge(1, 1);
        dg.add_edge(1, 2);
        assert_eq!(2, dg.num_edges(), "{:?}", dg.vertices);
    }
    #[test]
    fn has_two_vertices() {
        let mut dg = Graph::new();
        dg.add_edge(1, 1);
        dg.add_edge(2, 2);
        assert_eq!(2, dg.num_vertices());
    }
}
