use csv_generator::generator_mod::generator;

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};


fn csv_benchmark(c: &mut Criterion) {
    let nb = black_box(
        200
    );

    c.bench_function(
        "csv gen",
        |b| b.iter(|| generator(nb))
    );
}

criterion_group!(benches, csv_benchmark);
criterion_main!(benches);