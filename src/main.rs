use std::env;
use computer_v1::solve_eq;

fn main() {
    let args: Vec<String> = env::args().collect();
	if args.len() == 2 {
		solve_eq(&args[1]);
	}
}
