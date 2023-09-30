use std::collections::HashMap;

use arrayvec::ArrayVec;

use crate::{ID, N, SOURCE};

pub fn dijkstra(graph: &HashMap<ID, HashMap<ID, N>>) -> (HashMap<ID, N>, HashMap<ID, ID>) {
    let cap = graph.len();
    let mut visited = ArrayVec::<_, 6>::new();

    let mut distances = HashMap::with_capacity(cap);
    for node in graph.keys() {
        distances.insert(*node, u32::MAX);
    }
    distances.insert(SOURCE, 0);

    let mut previous = HashMap::<char, char>::with_capacity(cap);

    loop {
        let (id, distance) = distances
            .iter()
            .filter(|(id, _)| !visited.contains(*id))
            .min_by(|(_, d1), (_, d2)| d1.cmp(d2))
            .unwrap();
        let (id, distance) = (*id, *distance);

        visited.push(id);

        if visited.len() == graph.len() {
            break;
        }

        let edges = graph.get(&id).unwrap();
        for (int_id, length) in edges {
            if !visited.contains(int_id) {
                let new_dist = distance + length;
                if new_dist < distances[int_id] {
                    distances.insert(*int_id, new_dist);
                    previous.insert(*int_id, id);
                }
            }
        }
    }

    (distances, previous)
}
