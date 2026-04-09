use std::collections::VecDeque;

pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let num_courses = num_courses as usize;

    let mut in_degree = vec![0; num_courses];
    let mut adj = vec![vec![]; num_courses];

    for p in &prerequisites {
        let a = p[0] as usize;
        let b = p[1] as usize;
        in_degree[a] += 1;
        adj[b].push(a);
    }

    let mut zero_courses: VecDeque<usize> = in_degree
        .iter()
        .enumerate()
        .filter_map(|(i, &degree)| if degree == 0 { Some(i) } else { None })
        .collect();

    let mut order = Vec::with_capacity(num_courses);

    while let Some(course) = zero_courses.pop_front() {
        order.push(course as i32);
        for &next in &adj[course] {
            in_degree[next] -= 1;
            if in_degree[next] == 0 {
                zero_courses.push_back(next);
            }
        }
    }

    if order.len() == num_courses {
        order
    } else {
        vec![]
    }
}
