use std::collections::{HashMap, HashSet};

type N = u32;

fn main() {
    let edges: HashMap<char, HashMap<char, N>> = [
            ('a', [('b', 4), ('c', 4)].into()),
            ('b', [('c', 1), ('d', 7)].into()),
            ('c', [('e', 3)].into()),
            ('d', [('f', 5)].into()),
            ('e', [('d', 1), ('f', 8)].into()),
            ('f', HashMap::new())
        ].into();


    let mut dist = HashMap::<char, N>::new();
    let mut prev = HashMap::<char, char>::new();
    let mut current_search = HashSet::new();

    for (vert, _) in edges.clone() {
        dist.insert(vert, N::MAX);
        current_search.insert(vert);
    }
    dist.insert(SOURCE, 0);

    let grab_min = |list, distance: HashMap<char, N>| {
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

            for (neighbour, cost) in (&edges[&u]).into_iter().filter(|(v, _)| current_search.contains(*v)) {
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


    const SOURCE: char = 'a';
    for target in ['a', 'b', 'c', 'd', 'e', 'f'] {
        let mut s = vec![];
        let mut u = target;

        if prev.contains_key(&u) || u == SOURCE {
            loop {
                s.insert(0, u);

                if prev.contains_key(&u) {
                    u = prev[&u];
                } else {
                    break;
                }
            }
        }

        println!("To get to {target:?} from {SOURCE:?}, you must go: {s:?}");
    }



}
