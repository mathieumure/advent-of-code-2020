use std::{fs, process};
use std::collections::{HashMap, HashSet};

fn dfs_recursive(graph: &HashMap<u32, Vec<(u32, u32)>>, current_vertex: (u32, u32), mut stack: Vec<(u32, u32)>) {
    stack.push(current_vertex);

    for neighbor in &graph[&current_vertex.0] {
        dfs_recursive(graph, neighbor.clone(), stack.clone());
    }

    if graph.len() == stack.len() {
        println!("{:?}", stack);
        let diff_1 = stack.clone().into_iter().filter(|it| it.1 == 1).count();
        let diff_3 = stack.clone().into_iter().filter(|it| it.1 == 3).count() + 1;
        println!("1: {} // 3: {}", diff_1, diff_3);
        println!("Result: {}", diff_1 * diff_3);
        process::exit(0);
    }

    stack.pop();
}

fn part_1 (lines: &HashSet<u32>) -> u32 {
    let mut graph: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();

    for line in lines {
        let mut children: Vec<(u32, u32)> = Vec::new();
        for i in 1..4 {
            let potential_child = line + i;
            if lines.contains(&potential_child) {
                children.push((line + i, i));
            }
        }
        graph.insert(line.clone(), children);
    }

    let stack:Vec<(u32, u32)>  = Vec::new();

    dfs_recursive(&graph, (0, 0), stack);

    return 0;
}

fn main() {
    const FILE_PATH: &str = "src/test.txt";
    let file_content = fs::read_to_string(FILE_PATH).expect(&["Cannot open ", FILE_PATH].join(""));
    let mut lines: HashSet<u32> = file_content.lines().map(|it| it.parse::<u32>().unwrap()).collect();
    lines.insert(0);
    // part_1(&lines);
    part_2(&lines)
}
