use stringdiff::{annotate_strings, colorize};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <left> <right>", args[0]);
        std::process::exit(1);
    }
    let result = annotate_strings(&args[1], &args[2]);
    colorize(&result, &mut std::io::stdout());
}
