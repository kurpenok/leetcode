use std::collections::VecDeque;

pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
    let mut graph = vec![vec![]; n as usize];

    for edge in &edges {
        let u = edge[0];
        let v = edge[1];
        graph[u as usize].push(v);
        graph[v as usize].push(u);
    }

    let mut visited = vec![false; n as usize];
    let mut queue = VecDeque::new();
    queue.push_back(source);
    visited[source as usize] = true;

    while let Some(current) = queue.pop_front() {
        if current == destination {
            return true;
        }
        for edge in &graph[current as usize] {
            if !visited[*edge as usize] {
                queue.push_back(*edge);
                visited[*edge as usize] = true;
            }
        }
    }

    false
}
