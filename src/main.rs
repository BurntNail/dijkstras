use std::collections::HashMap;

use dijkstras::{v3_cheating, ID, N};

fn main () {
    let edges: HashMap<ID, HashMap<ID, N>> = [
        ('a', [('b', 4), ('c', 4)].into()),
        ('b', [('c', 1), ('d', 7)].into()),
        ('c', [('e', 3)].into()),
        ('d', [('f', 5)].into()),
        ('e', [('d', 1), ('f', 8)].into()),
        ('f', HashMap::new()),
    ]
    .into();

    v3_cheating(&edges);
}