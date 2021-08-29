#[cfg(feature = "run-examples")]
mod real;

#[cfg(feature = "run-examples")]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    real::tokenize::main()
}

#[cfg(not(feature = "run-examples"))]
fn main() {
    println!("'tokenize' example must be executed with 'run-examples' feature")
}
