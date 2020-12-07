#[macro_use]
extern crate lazy_static;

use dg::depth_first_search::DepthFirstSearch;
use dg::graph::Graph;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    println!("C:{}", now.elapsed().as_secs());

    let graph = graph_from_file(&config.filepath);

    println!("G:{}", now.elapsed().as_secs());


    println!("{:?}", graph.node_connections(330983));

    let dfs = DepthFirstSearch::new(graph);
    println!("DFS:{}", now.elapsed().as_secs());

    
    for (f, t) in config.path_queries {
        println!(
            "Reachable: From {:?} to {:?} is {:?} ",
            f,
            t,
            dfs.nodes_connected(f, t)
        );
    }

    /*
            cargo run ./algs4-data/largeDG.txt 330983:364276
            "330983 364276"
            "962477 227579"
            " 92125 711396"
            "318636 215952"
            " 16592 983528"
            "258326 123075"
            "492041 821051"
            " 93037 164079"
            "718467 643829"
            "616299  31038"
            "955730 739951"
            " 79089 149683"
            "800890 869706"

    */

    println!("NCK:{}", now.elapsed().as_secs());

    println!("Config: {:?} ", dfs.g.num_vertices());
}

struct Config {
    filepath: String,
    path_queries: Vec<(u32, u32)>,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let p = args[1].clone();
        let mut v = Vec::<(u32, u32)>::new();

        for a in args.iter().skip(2) {
            let t: Vec<&str> = a.trim().rsplitn(2, ":").collect();
            let to = t.get(0).unwrap().parse::<u32>().unwrap();
            let fr = t.get(1).unwrap().parse::<u32>().unwrap();
            v.push((fr, to));
        }
        Config {
            filepath: p,
            path_queries: v,
        }
    }
}

lazy_static! {
    static ref RE: Regex = Regex::new("^\\s*(\\d*)\\s+(\\d*)\\s*$").unwrap();
}

fn graph_from_file(filename: &str) -> Graph {
    let path = Path::new(filename);
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    //Stop recreating the same variable every time
    let mut dg = Graph::new();
    let mut ufr = 0 as u32;
    let mut uto = 0 as u32;
    let mut s = String::new();

    reader.lines().for_each(|l| {
        dg.add_edge(ufr, uto);
        s = l.unwrap();

        match RE.captures(&s) {
            Some(cap) => {
                ufr = *&cap[1].parse::<u32>().unwrap();
                uto = *&cap[2].parse::<u32>().unwrap();
                dg.add_edge(ufr, uto);
            }
            None =>(), //println!("NOMATCH: {:?}", s),
        }
    });
    dg
}
