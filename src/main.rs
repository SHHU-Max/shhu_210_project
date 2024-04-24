// Shaotai Hu DS210 Project
// Does California city planning create a logical 
// and efficient road network?
// Note:
// The website says the data is undirected, but after downloading it
// it says it is directed in the data file itself
// so I will treat it as directed data.

mod reader;
mod random;

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

// function to get n for Graph
fn node_count(path: &str) -> usize {
    let mut count: usize = 0;
    let mut data: ListOfEdges = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader{
        let line_str = line.expect("error reading");
        let v: Vec<&str> = line_str.trim().split_whitespace().collect();
        let x = v[0].parse::<usize>().unwrap();
        let y = v[1].parse::<usize>().unwrap();
        data.push((x,y));
    }
    for (i, j) in data{
        if j > count{
            count = j;
        }
    }
    return count+1;
}
fn main() {
// git commands
// git init
// git add .
// git commit -m "first"
// git remote add origin link
// git push origin master

    // using reader to read data into "roadnetwork"
    let mut road_edges = reader::read_txt("roadNetCAProject.txt");
    road_edges.sort();

    // node_count should be equal to 1971280+1=1971281
    let n = node_count("roadNetCAProject.txt");

    let graph = Graph::create_directed(n, &road_edges);

    //these are dead ends in the road
    let mut end_points: Vec<usize> = Vec::new();
    for (i, j) in graph.outedges.iter().enumerate(){
        if j.len() == 0{
            end_points.push(i);
        }
    }

    // breadth first search
    // to be used for average shortest path later
    let start = random::start();
    let mut distance: Vec<Option<usize>> = vec![None;graph.n];
    distance[start] = Some(0);
    let mut queue: VecDeque<Vertex> = VecDeque::new();
    queue.push_back(start);

    while let Some(v) = queue.pop_front() {
        for i in graph.outedges[v].iter(){
                if let None = distance[*i] {
                    distance[*i] = Some(distance[v].unwrap()+1);
                    queue.push_back(*i);
            }   
        }
    }

    for v in 0..graph.n{
        // if it is not a dead end
        // since dead ends have none for distance
        if distance[v] != None{
            println!("distance from vertex {} to {} is {}", start, v, distance[v].unwrap());
        }
    }
    
}

