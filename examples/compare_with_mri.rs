#[cfg(feature = "run-examples")]
mod real;

#[cfg(feature = "run-examples")]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    real::compare_with_mri::main()
}

#[cfg(not(feature = "run-examples"))]
fn main() {
    println!("'compare_with_mri' example must be executed with 'run-examples' feature")
}
