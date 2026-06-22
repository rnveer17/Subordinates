use subordinates::subordinates_iterative;

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();

    let mut children = vec![Vec::new(); n + 1];

    for i in 2..=n {
        let boss: usize = iter.next().unwrap().parse().unwrap();
        children[boss].push(i);
    }

    let result = subordinates_iterative(&children);

    for i in 1..=n {
        print!("{} ", result[i]);
    }
    println!();
}