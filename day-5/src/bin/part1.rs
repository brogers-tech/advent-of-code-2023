use advent_of_code::read_file;

use std::collections::HashMap;
use std::ops::RangeInclusive;

#[derive(Debug)]
struct Edge {
    src_rng: RangeInclusive<usize>,
    dst_rng: RangeInclusive<usize>,
    range: usize,
}

fn get_seeds(data: &mut dyn Iterator<Item = String>) -> Vec<usize> {
    data.next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split(" ")
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn get_almanac_data(data: &mut dyn Iterator<Item = String>) -> Vec<Vec<String>> {
    let mut res = vec![];

    let mut block = Vec::new();
    for mut line in data.skip(1) {
        if line.is_empty() {
            res.push(block);
            block = Vec::new();
        } else {
            if block.is_empty() {
                line = line.split(" ").next().unwrap().to_string();
            }
            block.push(line);
        }
    }

    res
}

fn create_graph(data: Vec<Vec<String>>) -> HashMap<String, HashMap<String, Vec<Edge>>> {
    data.iter()
        .map(|vert_data| {
            let mut i_vert_data = vert_data.iter();
            let mut header = i_vert_data
                .next()
                .unwrap()
                .split("-to-")
                .take(2)
                .map(|s| s.to_string());
            let (from_lbl, to_lbl) = (header.next().unwrap(), header.next().unwrap());

            let mut edges = Vec::new();
            for edge_data in i_vert_data {
                let mut n = edge_data.split(" ").map(|s| s.parse::<usize>().unwrap());
                let (dst, src, range) = (n.next().unwrap(), n.next().unwrap(), n.next().unwrap());
                let edge = Edge { src_rng: src..=src+range-1, dst_rng: dst..=dst+range-1, range };

                edges.push(edge);
            }

            let mut hm = HashMap::new();
            hm.insert(to_lbl, edges);

            (from_lbl, hm)
        })
        .collect()
}

fn main() {
    let mut data = read_file("input.txt");
    let seeds = get_seeds(&mut data);
    let almanac_data = get_almanac_data(&mut data);
    let graph = create_graph(almanac_data);

    println!("Seeds: {seeds:?}");
    println!("Map data: {graph:#?}");
}
