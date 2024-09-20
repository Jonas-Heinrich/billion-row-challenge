//! Feel free to totally change the structure of this project to fit your needs. The only
//! restriction is that you should take `--input-file` as a CLI argument and print the solution
//! to stdout in the format described in the README.md file.

use std::path::PathBuf;

mod util;

fn compute_solution(input_path: PathBuf) -> String {
    // TODO: 1. Change the author in the Cargo.toml file to your name. (Does not have to be your real name.)
    // TODO: 2. Implement your solution and tests here.

    let _ = input_path;
    String::from("output")
}

fn main() {
    let cli = util::Cli::create();
    let input_path = cli.get_input_path();

    let solution = compute_solution(input_path);
    println!("{}", solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let input = "input";
        let actual_output = compute_solution(PathBuf::from(input));
        let expected_output = "output";

        assert!(
            actual_output.starts_with("out"),
            "Example boolean assertion failed"
        );
        assert_eq!(
            actual_output, expected_output,
            "Example comparative assertion failed"
        );
    }

    // You can add or modify the tests to your heart's content here 😉.
}
