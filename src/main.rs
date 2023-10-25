use mmv::run_pipeline;
fn main() {
    match run_pipeline() {
        Ok(()) => println!("Successfully completed"),
        Err(err) => {
            print!("mmv: {}", err);
            std::process::exit(1)
        }
    }
}
