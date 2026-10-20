use rosidl_runtime_rs::Sequence;
use std::{hint::black_box, time::Instant};

fn measure<T: rosidl_runtime_rs::SequenceAlloc + Clone>(name: &str, input: Vec<T>) {
    let mut times = Vec::new();
    for _ in 0..100 {
        let input = black_box(input.clone());
        let start = Instant::now();
        let output: Sequence<T> = input.into();
        times.push(start.elapsed().as_nanos());
        black_box(output);
    }
    times.sort_unstable();
    println!(
        "{name}: median_ns={} size={}",
        times[50],
        size_of::<Sequence<T>>()
    );
}

fn main() {
    measure("u8_8MiB", vec![42u8; 8 * 1024 * 1024]);
    measure("f32_1M", vec![0.25f32; 1_000_000]);
    measure(
        "string_10k",
        vec![rosidl_runtime_rs::String::from("sequence"); 10_000],
    );
}
