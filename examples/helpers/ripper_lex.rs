use std::process::Command;

#[allow(dead_code)]
pub fn ripper_lex(filepath: &str) -> Result<String, String> {
    let out = Command::new("ruby")
            .args(&["examples/helpers/ripper_lex.rb", filepath])
            .output()
            .map_err(|_| "failed to execute process".to_owned() )?;

    if out.status.success() {
        String::from_utf8(out.stdout).map_err(|_| "non-utf8 output".to_owned())
    } else {
        println!("{}", String::from_utf8_lossy(&out.stderr).to_owned());
        Err("non-zero exit code".to_owned())
    }
}
