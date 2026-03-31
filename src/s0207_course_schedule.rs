use std::collections::VecDeque;

pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let num_courses = num_courses as usize;

    let mut in_degree = vec![0; num_courses];
    let mut adj = vec![Vec::new(); num_courses];

    for p in &prerequisites {
        let a = p[0] as usize;
        let b = p[1] as usize;
        in_degree[a] += 1;
        adj[b].push(a);
    }

    let mut zero_courses: VecDeque<usize> = in_degree
        .iter()
        .enumerate()
        .filter_map(|(course, &degree)| if degree == 0 { Some(course) } else { None })
        .collect();

    let mut taken = 0;
    while let Some(u) = zero_courses.pop_front() {
        taken += 1;
        for &v in &adj[u] {
            in_degree[v] -= 1;
            if in_degree[v] == 0 {
                zero_courses.push_back(v);
            }
        }
    }

    taken == num_courses
}
