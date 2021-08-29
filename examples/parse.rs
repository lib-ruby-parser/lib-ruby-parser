#[cfg(feature = "run-examples")]
mod real;

#[cfg(feature = "run-examples")]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    real::parse::main()
}

#[cfg(not(feature = "run-examples"))]
fn main() {
    println!("'parse' example must be executed with 'run-examples' feature")
}
