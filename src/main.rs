// Shaotai Hu DS210 Project
// Does California city planning create a logical 
// and efficient road network?
// Note:
// The website says the data is undirected, but after downloading it
// it says it is directed in the data file itself
// so I will treat it as directed data.

mod reader;

use std::fs::File;
use std::io::prelude::*;
use rand::prelude::*;
use std::collections::VecDeque;

type Vertex = usize;
type ListOfEdges = Vec<(Vertex, Vertex)>;
type AdjacencyLists = Vec<Vec<Vertex>>;
struct Graph {
    n: usize,
    outedges: AdjacencyLists,
}

// impl functions for Graph to make directed graph
impl Graph {
    fn add_directed_edges(&mut self, edges:&ListOfEdges) {
        for (u,v ) in edges {
            self.outedges[*u].push(*v);
        }
    }
    fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut(){
            l.sort();
        }
    }
    fn create_directed(n: usize, edges: &ListOfEdges) -> Graph{
        let mut g = Graph{n,outedges:vec![vec![];n]};
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g
    }
}

fn main() {
    // using reader to read data into "roadnetwork"
    //
    //
    let mut road_edges = reader::read_txt("roadNetCAProject.txt");
}

