use std::process::Command;

pub fn ripper_lex(filepath: &str) -> Result<String, &'static str> {
    let out = Command::new("ruby")
        .args(&["examples/helpers/ripper_lex.rb", filepath])
        .output()
        .map_err(|_| "failed to execute process")?;

    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    if !stderr.is_empty() {
        println!("ripper_lex stderr:\n{}", stderr);
    }

    if out.status.success() {
        String::from_utf8(out.stdout).map_err(|_| "non-utf8 output")
    } else {
        println!("{}", String::from_utf8_lossy(&out.stderr).into_owned());
        Err("non-zero exit code")
    }
}
