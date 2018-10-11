#[macro_use]
extern crate criterion;
extern crate lzf;

use criterion::Criterion;

fn bench_lzf_compression(c: &mut Criterion) {
    static KB: usize = 1024;

    let mut data = Vec::new();
    data.push(KB);
    data.push(2 * KB);
    data.push(4 * KB);
    data.push(6 * KB);

    c.bench_function_over_inputs(
        "lzf 0",
        |b, &size| {
            let buffer = std::iter::repeat(0u8).take(size).collect::<Vec<_>>();
            b.iter(|| lzf::compress(&buffer).unwrap());
        },
        data.clone(),
    );

    c.bench_function_over_inputs(
        "lzf 17",
        |b, &size| {
            let buffer = std::iter::repeat(17u8).take(size).collect::<Vec<_>>();
            b.iter(|| lzf::compress(&buffer).unwrap());
        },
        data.clone(),
    );

    c.bench_function_over_inputs("lzf lorem", |b, &size| {
        let lorem = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua.";
        let mut buffer = String::new();
        while buffer.len() < size {
            buffer.push_str(lorem);
        }
        b.iter(|| lzf::compress(&buffer.as_bytes()[0..size]).unwrap());
    }, data.clone());
}

criterion_group!(benches, bench_lzf_compression);
criterion_main!(benches);
