use stringdiff::{annotate_strings, colorize};

fn read_input(arg: &str) -> String {
    if arg == "-" {
        std::io::read_to_string(std::io::stdin()).expect("failed to read stdin")
    } else if std::path::Path::new(arg).exists() {
        std::fs::read_to_string(arg).expect("failed to read file")
    } else {
        arg.to_string()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <left> <right>", args[0]);
        eprintln!("  Each argument can be a literal string, a file path, or '-' for stdin.");
        std::process::exit(1);
    }
    let left = read_input(&args[1]);
    let right = read_input(&args[2]);
    let result = annotate_strings(&left, &right);
    colorize(&result, &mut std::io::stdout());
}
