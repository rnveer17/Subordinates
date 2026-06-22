use std::time::Instant;
use std::hint::black_box;
use subordinates::{subordinates_dfs, subordinates_iterative};
use std::fs;

fn main() {
    let test_dir = "tests/inout";
    let mut test_cases = Vec::new();

    if let Ok(entries) = fs::read_dir(test_dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("in") {
                let output_file = path.with_extension("out");
                if output_file.exists() {
                    test_cases.push(path);
                }
            }
        }
    }

    if test_cases.is_empty() {
        println!("No test cases found. Run `cargo build` first.");
        return;
    }

    test_cases.sort_by_key(|p| {
        p.file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .parse::<u32>()
            .unwrap_or(0)
    });

    println!("{:>10} | {:>15} | {:>15} | {:>15}",
             "n", "Recursive (ms)", "Iterative (ms)", "Diff (ms)");
    println!("{:-<10}-+-{:-<15}-+-{:-<15}-+-{:-<15}", "", "", "", "");

    for input_file in &test_cases {
        let content = fs::read_to_string(input_file).unwrap();
        let mut lines = content.lines();
        let n: usize = lines.next().unwrap().trim().parse().unwrap_or(0);

        if n == 0 {
            continue;
        }

        let mut children = vec![Vec::new(); n + 1];
        let boss_line = lines.next().unwrap_or("");
        let mut boss_iter = boss_line.split_whitespace();

        for i in 2..=n {
            let boss: usize = boss_iter.next().unwrap().parse().unwrap();
            children[boss].push(i);
        }

        let start = Instant::now();
        let _ = black_box(subordinates_dfs(&children));
        let dur_rec = start.elapsed();

        let start = Instant::now();
        let _ = black_box(subordinates_iterative(&children));
        let dur_iter = start.elapsed();

        let rec_ms = dur_rec.as_secs_f64() * 1000.0;
        let iter_ms = dur_iter.as_secs_f64() * 1000.0;
        let diff_ms = rec_ms - iter_ms;

        println!(
            "{:>10} | {:>15.2} | {:>15.2} | {:>15.2}",
            n, rec_ms, iter_ms, diff_ms
        );
    }
}