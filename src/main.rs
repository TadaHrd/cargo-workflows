use colored::Colorize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(e) = cargo_workflows::main() {
        println!(
            "{error}{colon} {e}",
            error = "error".bold().red(),
            colon = ":".bold().white()
        );
        std::process::exit(1);
    }

    Ok(())
}
