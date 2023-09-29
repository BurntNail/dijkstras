mod bitvec;

use bitvec::ExpandableBitVec;
use std::collections::{HashMap, HashSet};

pub type N = u32;
pub type ID = char;
pub const SOURCE: ID = 'a';
const SOURCE_: usize = SOURCE as usize;

pub fn v3(hm_edges: &HashMap<ID, HashMap<ID, N>>) -> (HashMap<ID, N>, HashMap<ID, ID>) {
    let len = hm_edges.len();

    let mut edges = vec![vec![]; len];
    for (vert, vert_edges) in hm_edges.clone() {
        let size = vert_edges.keys().max().copied().unwrap_or(SOURCE) as usize - SOURCE_ + 1;
        let mut inner = vec![N::MAX; size];
        for (inner_vert, cost) in vert_edges {
            inner[(inner_vert as usize) - SOURCE_] = cost;
        }
        edges[(vert as usize) - SOURCE_] = inner;
    } 

    let mut dist = vec![N::MAX; len];
    dist[0] = 0;
    let mut prev = vec![None; len];



    let mut visited = ExpandableBitVec::new(len);

    loop {
        let Some((u, initial_cost)) = dist.iter().enumerate().filter(|(index, _cost)| {
            !visited.index(*index)
        }).min_by_key(|(_index, cost)| **cost) else {break};
        let initial_cost = *initial_cost;
        visited.set(u, true);

        for (neighbour, cost) in (&edges[u]).into_iter().enumerate().filter(|(v, cost)| !visited.index(*v) && **cost != N::MAX) {
            let alt = initial_cost + cost;
            if alt < dist[neighbour] {
                dist[neighbour] = alt;
                prev[neighbour] = Some(u);
            }
        }
    }

    return ({
        dist.into_iter().enumerate().map(|(index, cost)| ((index + SOURCE_) as u8 as char, cost)).collect()
    }, {
        prev.into_iter().enumerate().filter(|(_a, b)| b.is_some()).map(|(a, b)| ((a + SOURCE_) as u8 as char, (b.unwrap() + SOURCE_) as u8 as char)).collect()
    });
}

pub fn v2(edges: &HashMap<ID, HashMap<ID, N>>) -> (HashMap<ID, N>, HashMap<ID, ID>) {
    let len = edges.len();
    let mut dist = HashMap::<ID, N>::with_capacity(len);
    let mut prev = HashMap::<ID, ID>::with_capacity(len);
    let mut all_to_be_searched = HashSet::with_capacity(len);

    for (vert, _) in edges.clone() {
        dist.insert(vert, N::MAX);
        all_to_be_searched.insert(vert);
    }
    dist.insert(SOURCE, 0);

    let mut visited = HashSet::with_capacity(len);

    loop {
        let Some((u, initial_cost)) = dist.iter().filter(|(v, _)| !visited.contains(*v)).map(|(a, b)| (*a, *b)).min_by(|(_, a), (_, b)| a.cmp(b)) else {
            break;
        };
        visited.insert(u);

        for (neighbour, cost) in (&edges[&u])
            .into_iter()
            .filter(|(v, _)| !visited.contains(*v))
        {
            let alt = initial_cost + cost;
            if alt < dist[neighbour] {
                dist.insert(*neighbour, alt);
                prev.insert(*neighbour, u);
            }
        }
    }

    return (dist, prev);
}

pub fn v1(edges: &HashMap<ID, HashMap<ID, N>>) -> (HashMap<ID, N>, HashMap<ID, ID>) {
    let mut dist = HashMap::<ID, N>::new();
    let mut prev = HashMap::<ID, ID>::new();
    let mut current_search = HashSet::new();

    for (vert, _) in edges.clone() {
        dist.insert(vert, N::MAX);
        current_search.insert(vert);
    }
    dist.insert(SOURCE, 0);

    let grab_min = |list, distance: HashMap<ID, N>| {
        let mut working = None;
        let mut working_dist = N::MAX;
        for c in list {
            if distance[&c] < working_dist {
                working = Some(c);
                working_dist = distance[&c];
            }
        }

        return working;
    };

    loop {
        if let Some(u) = grab_min(current_search.clone(), dist.clone()) {
            current_search.remove(&u);

            for (neighbour, cost) in (&edges[&u])
                .into_iter()
                .filter(|(v, _)| current_search.contains(*v))
            {
                let alt = dist[&u] + cost;
                if alt < dist[neighbour] {
                    dist.insert(*neighbour, alt);
                    prev.insert(*neighbour, u);
                }
            }
        } else {
            break;
        }
    }

    return (dist, prev);
}
