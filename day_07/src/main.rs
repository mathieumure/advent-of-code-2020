use std::fs;
use std::collections::{HashMap};
use regex::Regex;
use pathfinding::prelude::dfs;

fn parse_to_graph (file: &str) -> HashMap<String, Vec<(String, u32)>> {
    let file_content = fs::read_to_string(file).expect("Something went wrong reading the file");
    let lines = file_content.lines();
    let mut graph: HashMap<String, Vec<(String, u32)>> = HashMap::new();
    let re = Regex::new(r"^(.*) bags contain (.*)$").unwrap();
    let re_children = Regex::new(r"(\d)+ ([\w ]+) bag[s]?[,.]").unwrap();
    for line in lines {
        let groups = re.captures(line).unwrap();
        let container = String::from(&groups[1]);
        let rest = &groups[2];
        let mut children: Vec<(String, u32)> = Vec::new();

        if rest != "no other bags." {
            children = re_children.captures_iter(rest).map(| cap | (String::from(&cap[2]), cap[1].parse::<u32>().unwrap())).collect();
        }
        graph.insert(container,children );
    }

    return graph;
}

fn part_1 (file: &str) {
    let graph = parse_to_graph(file);

    let a: Vec<Vec<String>> = graph.keys().map(| key | dfs(
        String::from(key),
        | n | graph.get(n).unwrap().into_iter().map(|it| it.0.clone()),
        | n | n.eq("shiny gold")
    ).unwrap_or(Vec::new()))
        .filter(|x| x.len() > 1).collect();

    println!("Result: {:?}", a.len());
}

fn compute(start: &String, graph: &HashMap<String, Vec<(String, u32)>>) -> u32 {
    let other_bags = graph.get(start).unwrap();
    if other_bags.len() == 0 {
        return 1
    }

    let mut sum = 0;
    for child_bag in other_bags {
       sum += child_bag.1 * compute(&child_bag.0, graph);
    }

    return 1 + sum
}

fn part_2 (file: &str) {
    let graph = parse_to_graph(file);

    let result = compute(&String::from("shiny gold"), &graph);

    println!("Result: {:?}", result - 1);
}

fn main() {
    part_1("src/source.txt");
    part_2("src/source.txt");
}
