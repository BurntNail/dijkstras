use arrayvec::ArrayVec;
use std::collections::{HashMap, HashSet};

pub mod bstkl;

pub type N = u32;
pub type ID = char;
pub const SOURCE: ID = 'a';
const SOURCE_: usize = SOURCE as usize;

pub fn v3_cheating_normal_array(
    hm_edges: &HashMap<ID, HashMap<ID, N>>,
) -> (HashMap<ID, N>, HashMap<ID, ID>) {
    const LEN: usize = 6;

    let mut edges = [[None; LEN]; LEN];
    let mut dist = [N::MAX; LEN];
    let mut prev = [None; LEN];

    for (vert, vert_edges) in hm_edges.clone().into_iter() {
        let inner = &mut edges[(vert as usize) - SOURCE_];

        for (inner_vert, cost) in vert_edges {
            inner[(inner_vert as usize) - SOURCE_] = Some(cost);
        }
    }
    dist[0] = 0;

    let mut visited = 0;
    let visited_contains = |visited: usize, index: usize| visited & (1 << index) > 0;

    loop {
        let Some((u, initial_cost)) = dist
            .iter()
            .enumerate()
            .filter(|(index, _cost)| !visited_contains(visited, *index))
            .min_by_key(|(_index, cost)| **cost)
        else {
            break;
        };
        let initial_cost = *initial_cost;
        visited |= 1 << u;

        for (neighbour, cost) in edges[u]
            .iter()
            .enumerate()
            .filter_map(|(v, cost)| cost.map(|cost| (v, cost)))
            .filter(|(v, _cost)| !visited_contains(visited, *v))
        {
            let alt = initial_cost + cost;
            if alt < dist[neighbour] {
                dist[neighbour] = alt;
                prev[neighbour] = Some(u);
            }
        }
    }

    (
        {
            dist.into_iter()
                .enumerate()
                .map(|(index, cost)| ((index + SOURCE_) as u8 as char, cost))
                .collect()
        },
        {
            prev.into_iter()
                .enumerate()
                .filter(|(_a, b)| b.is_some())
                .map(|(a, b)| {
                    (
                        (a + SOURCE_) as u8 as char,
                        (b.unwrap() + SOURCE_) as u8 as char,
                    )
                })
                .collect()
        },
    )
}

pub fn v3_cheating(hm_edges: &HashMap<ID, HashMap<ID, N>>) -> (HashMap<ID, N>, HashMap<ID, ID>) {
    const LEN: usize = 6;

    let mut edges = ArrayVec::<_, LEN>::new();
    (0..LEN).for_each(|_| edges.push(ArrayVec::<_, LEN>::new()));
    let mut dist = ArrayVec::<N, LEN>::new();
    let mut prev = ArrayVec::<Option<usize>, LEN>::new();

    for (vert, vert_edges) in hm_edges.clone().into_iter() {
        let inner = &mut edges[(vert as usize) - SOURCE_];
        (0..LEN).for_each(|_| inner.push(None));

        for (inner_vert, cost) in vert_edges {
            inner[(inner_vert as usize) - SOURCE_] = Some(cost);
        }

        dist.push(N::MAX);
        prev.push(None);
    }
    dist[0] = 0;

    let mut visited = 0;
    let visited_contains = |visited: usize, index: usize| visited & (1 << index) > 0;

    loop {
        let Some((u, initial_cost)) = dist
            .iter()
            .enumerate()
            .filter(|(index, _cost)| !visited_contains(visited, *index))
            .min_by_key(|(_index, cost)| **cost)
        else {
            break;
        };
        let initial_cost = *initial_cost;
        visited |= 1 << u;

        for (neighbour, cost) in (&edges[u])
            .into_iter()
            .enumerate()
            .filter_map(|(v, cost)| cost.map(|cost| (v, cost)))
            .filter(|(v, _cost)| !visited_contains(visited, *v))
        {
            let alt = initial_cost + cost;
            if alt < dist[neighbour] {
                dist[neighbour] = alt;
                prev[neighbour] = Some(u);
            }
        }
    }

    (
        {
            dist.into_iter()
                .enumerate()
                .map(|(index, cost)| ((index + SOURCE_) as u8 as char, cost))
                .collect()
        },
        {
            prev.into_iter()
                .enumerate()
                .filter(|(_a, b)| b.is_some())
                .map(|(a, b)| {
                    (
                        (a + SOURCE_) as u8 as char,
                        (b.unwrap() + SOURCE_) as u8 as char,
                    )
                })
                .collect()
        },
    )
}

pub fn v3(hm_edges: &HashMap<ID, HashMap<ID, N>>) -> (HashMap<ID, N>, HashMap<ID, ID>) {
    let len = hm_edges.len();

    let mut edges = vec![vec![]; len];
    for (vert, vert_edges) in hm_edges.clone() {
        let inner = &mut edges[(vert as usize) - SOURCE_];
        (0..len).for_each(|_| inner.push(None));
        for (inner_vert, cost) in vert_edges {
            inner[(inner_vert as usize) - SOURCE_] = Some(cost);
        }
    }

    let mut dist = vec![N::MAX; len];
    dist[0] = 0;
    let mut prev = vec![None; len];

    let mut visited = 0;
    let visited_contains = |visited: usize, index: usize| visited & (1 << index) > 0;

    loop {
        let Some((u, initial_cost)) = dist
            .iter()
            .enumerate()
            .filter(|(index, _cost)| !visited_contains(visited, *index))
            .min_by_key(|(_index, cost)| **cost)
        else {
            break;
        };
        let initial_cost = *initial_cost;
        visited |= 1 << u;

        for (neighbour, cost) in edges[u]
            .iter()
            .enumerate()
            .filter_map(|(v, cost)| cost.map(|cost| (v, cost)))
            .filter(|(v, _cost)| !visited_contains(visited, *v))
        {
            let alt = initial_cost + cost;
            if alt < dist[neighbour] {
                dist[neighbour] = alt;
                prev[neighbour] = Some(u);
            }
        }
    }

    (
        {
            dist.into_iter()
                .enumerate()
                .map(|(index, cost)| ((index + SOURCE_) as u8 as char, cost))
                .collect()
        },
        {
            prev.into_iter()
                .enumerate()
                .filter(|(_a, b)| b.is_some())
                .map(|(a, b)| {
                    (
                        (a + SOURCE_) as u8 as char,
                        (b.unwrap() + SOURCE_) as u8 as char,
                    )
                })
                .collect()
        },
    )
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
        let Some((u, initial_cost)) = dist
            .iter()
            .filter(|(v, _)| !visited.contains(*v))
            .map(|(a, b)| (*a, *b))
            .min_by(|(_, a), (_, b)| a.cmp(b))
        else {
            break;
        };
        visited.insert(u);

        for (neighbour, cost) in edges[&u].iter().filter(|(v, _)| !visited.contains(*v)) {
            let alt = initial_cost + cost;
            if alt < dist[neighbour] {
                dist.insert(*neighbour, alt);
                prev.insert(*neighbour, u);
            }
        }
    }

    (dist, prev)
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

        working
    };

    while let Some(u) = grab_min(current_search.clone(), dist.clone()) {
        current_search.remove(&u);

        for (neighbour, cost) in edges[&u]
            .iter()
            .filter(|(v, _)| current_search.contains(*v))
        {
            let alt = dist[&u] + cost;
            if alt < dist[neighbour] {
                dist.insert(*neighbour, alt);
                prev.insert(*neighbour, u);
            }
        }
    }

    (dist, prev)
}
