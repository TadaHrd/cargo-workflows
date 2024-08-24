use colored::Colorize;

fn main() {
    if let Err(e) = cargo_workflows::main() {
        println!(
            "{error}{colon} {e}",
            error = "error".bold().red(),
            colon = ":".bold().white()
        );
        std::process::exit(1);
    }
}
