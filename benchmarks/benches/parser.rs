use criterion::{criterion_group, criterion_main, Criterion};
fn bench(c:&mut Criterion){c.bench_function("parser_placeholder",|b|b.iter(||1+1));}
criterion_group!(benches,bench);
criterion_main!(benches);
