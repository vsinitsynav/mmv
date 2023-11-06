use mmv::run_pipeline;
use std::io::{stdout, Write};
fn main() {
    match run_pipeline() {
        Ok(()) => println!("Successfully completed"),
        Err(err) => {
            let _ = write!(stdout(), "mmv: {}", err);
            std::process::exit(1)
        }
    }
}
