use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dijkstras::*;
use std::collections::HashMap;

fn criterion_benchmark(c: &mut Criterion) {
    let edges: HashMap<ID, HashMap<ID, N>> = [
        ('a', [('b', 4), ('c', 4)].into()),
        ('b', [('c', 1), ('d', 7)].into()),
        ('c', [('e', 3)].into()),
        ('d', [('f', 5)].into()),
        ('e', [('d', 1), ('f', 8)].into()),
        ('f', HashMap::new()),
    ]
    .into();

    const EXPECTED: &[&[char]] = &[&['a'], &['a', 'b'], &['a', 'c'], &['a', 'c', 'e', 'd'], &['a', 'c', 'e'], &['a', 'c', 'e', 'd', 'f']];

    c.bench_function("latest_version", |b| {
        b.iter(|| {
            let (dist, prev) = v3(&edges);
            black_box(dist);

            let mut s = Vec::with_capacity(6);
            for (i, target) in ['a', 'b', 'c', 'd', 'e', 'f'].into_iter().enumerate() {
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

                if &s != EXPECTED[i] {
                    panic!("Got {s:?}, Expected {:?}", EXPECTED[i]);
                }

                black_box(&s);
                s.clear();
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
