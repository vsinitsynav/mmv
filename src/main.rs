use mmv::{r#move::MoveError, run_pipeline};
use std::io::{stdout, Write};
use std::process::ExitCode;

fn print_error(x: MoveError) {
    let _ = writeln!(stdout(), "mmv: {}", x);
}
fn main() -> ExitCode {
    let x = run_pipeline();
    if x.is_err() {
        let _ = x.map_err(print_error);
        return ExitCode::from(42);
    }
    let _ = writeln!(stdout(), "Successfully completed");
    ExitCode::SUCCESS
}
