use std::{
    collections::{HashMap, HashSet},
    hint::black_box,
    time::Instant,
};

type N = u32;
type ID = char;
const SOURCE: ID = 'a';

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

fn main() {
    let edges: HashMap<ID, HashMap<ID, N>> = [
        ('a', [('b', 4), ('c', 4)].into()),
        ('b', [('c', 1), ('d', 7)].into()),
        ('c', [('e', 3)].into()),
        ('d', [('f', 5)].into()),
        ('e', [('d', 1), ('f', 8)].into()),
        ('f', HashMap::new()),
    ]
    .into();

    const RUNS: u32 = 10_000;

    let start = Instant::now();
    for _ in 0..RUNS {
        let a = v2(&edges);
        black_box(a);

            // for target in ['a', 'b', 'c', 'd', 'e', 'f'] {
    //     let mut s = vec![];
    //     let mut u = target;

    //     if prev.contains_key(&u) || u == SOURCE {
    //         loop {
    //             s.insert(0, u);

    //             if prev.contains_key(&u) {
    //                 u = prev[&u];
    //             } else {
    //                 break;
    //             }
    //         }
    //     }

    //     // println!("To get to {target:?} from {SOURCE:?}, you must go: {s:?}");
    // }
    }
    println!("Took {:?}.", start.elapsed() / RUNS);
}
