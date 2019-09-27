use std::env;
use computer_v1::solve_eq;

fn faillable_main() -> Result<String, Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
	if args.len() == 2 {
		return solve_eq(&args[1])
	}
	return Err(format!("Wrong number of arguments."))?

}

fn main() {
    ::std::process::exit(match faillable_main() {
       Ok(output) => {
           println!("{}", output);
           0
       },
       Err(err) => {
           eprintln!("Error: {}", err);
           1
       }
    });
}

