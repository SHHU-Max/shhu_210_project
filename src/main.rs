// Shaotai Hu DS210 Project
// Does California city planning create a logical 
// and efficient road network?
// Note:
// The data is directed

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
    // not really used, but good to have
    let mut end_points: Vec<usize> = Vec::new();
    for (i, j) in graph.outedges.iter().enumerate(){
        if j.len() == 0{
            end_points.push(i);
        }
    }

    fn average_distances(){
        let mut road_edges = reader::read_txt("roadNetCAProject.txt");
        road_edges.sort();
    
        // node_count should be equal to 1971280+1=1971281
        let n = node_count("roadNetCAProject.txt");
    
        let graph = Graph::create_directed(n, &road_edges);
        let mut ran_sample: Vec<usize> = Vec::new();
        
        // random sampling
        let sample_count = 2000;
        for _ in 0..sample_count{
            let sample = random::sample();
            ran_sample.push(sample);
        }

        // average distances via breadth first search
        // each vertex in random sample average distance to every other vertex in the graph
        // sample_average_dis holds the format of 
        // vertex/node, its average distance to all other vertex/node in the graph
        let mut sample_average_dis: Vec<(usize, usize)> = Vec::new();
        for vtx in ran_sample{
            let start = vtx;
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

            // I take a random node's all distances to every other node on the graph
            // sum those distances then divide by a count
            // push that node and its average distance to every other node into a vector.
            let mut total_dis:usize = 0;
            let mut temp_count: usize = 0;
            for v in 0..graph.n{
                temp_count += 1;
                if distance[v] != None{
                    total_dis += distance[v].unwrap();
                }
            }
            let aver_dis = total_dis/temp_count;
            //println!("Vertex {} 's average distance to every other vertex is {}", start, aver_dis);
            // closeness centrality
            sample_average_dis.push((start, aver_dis));
        }
    
        // the 2000 sample is randomly selected thus represents the total population
        let mut average_travel_distance: usize = 0;
        for (_ss, jj) in sample_average_dis{
            average_travel_distance += jj;
        }
        let total_aver_travel = average_travel_distance/sample_count;
        println!
        ("The overall average travel distance in CA from / 
        random location A to random location B is {:?}", total_aver_travel);
    }

        // centrality
    // finding top 50 node with highest degree centrality
    // higher = more importance, as it could be downtown area
    fn centrality(){
        // using reader to read data into "roadnetwork"
        let mut road_edges = reader::read_txt("roadNetCAProject.txt");
        road_edges.sort();

        // node_count should be equal to 1971280+1=1971281
        let n = node_count("roadNetCAProject.txt");

        let graph = Graph::create_directed(n, &road_edges);
        let mut centrality_measure: Vec<(usize, usize)> = Vec::new();
        for (node, edges) in graph.outedges.iter().enumerate(){
            //println!("{:?}, {:?}", node, edges.len());
            centrality_measure.push((edges.len(), node));
        }
        centrality_measure.sort();
        centrality_measure.reverse();
    
        let mut ccc = 0;
        println!("Top 50 most degree centered nodes:");
        for (xx, yy) in centrality_measure{
            if ccc < 51{
                ccc += 1;
                let start = yy;
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
                let mut total_dis:usize = 0;
                let mut temp_count: usize = 0;
                for v in 0..graph.n{
                    temp_count += 1;
                    if distance[v] != None{
                        total_dis += distance[v].unwrap();
                    }
                }
                let aver_dis = total_dis/temp_count;
                println!("degree: {:?}, node: {:?}, distance to every other node: {:?}", xx, yy, aver_dis);
            }
        }

    }
    // average_distances() takes roughly 2 minutes to run
    average_distances();
    centrality();
    
}
    #[test]
    fn project_test(){
        
    }

    #[test]
    fn project_test_two(){
        
    }

