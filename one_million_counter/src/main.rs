use std::time::Instant;

fn count_to_one_million() -> f64 {
    let start_time = Instant::now();

    for _ in 1..=1_000_000_000 {
        // Do nothing, just iterate
    }

    let end_time = start_time.elapsed();

    let elapsed_time = end_time.as_secs() as f64 + end_time.subsec_nanos() as f64 * 1e-9;
    elapsed_time
}

fn main() {
    let elapsed_time = count_to_one_million();
    println!("Rust: Counted to 1 million in {:.6} seconds.", elapsed_time);
}

