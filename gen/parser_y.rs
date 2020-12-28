extern crate rust_bison_skeleton;
use std::path::Path;

const PARSER_Y: &str = "src/parser.y";
const PARSER_RS: &str = "src/parser.rs";
const PARSER_FEATURES_Y: &str = "src/parser.features.y";
const PARSER_FEATURES_RS: &str = "src/parser.features.rs";

pub fn generate_parser_y() {
    println!("cargo:rerun-if-changed={}", PARSER_Y);

    apply_features();
    compile_with_bison();
    std::fs::rename(PARSER_FEATURES_RS, PARSER_RS).unwrap();
    std::fs::remove_file(PARSER_FEATURES_Y).unwrap();
}

const ALL_FEATURES: &[&'static str] = &["lsp-error-recovery"];

#[derive(Debug, Default)]
struct Features {
    enabled: Vec<&'static str>,
    disabled: Vec<&'static str>,
}

impl Features {
    pub fn is_enabled(&self, f: &str) -> bool {
        self.enabled.contains(&f)
    }

    pub fn is_disabled(&self, f: &str) -> bool {
        self.disabled.contains(&f)
    }
}

fn features() -> Features {
    let mut features = Features::default();

    if cfg!(feature = "lsp-error-recovery") {
        features.enabled.push("lsp-error-recovery")
    } else {
        features.disabled.push("lsp-error-recovery")
    }

    features
}

fn apply_features() {
    let mut lines = vec![];
    let parser_y = std::fs::read_to_string(PARSER_Y).unwrap();
    let features = features();
    eprintln!("{:?}", features);

    let mut skip_lines = false;

    for (idx, line) in parser_y.lines().enumerate() {
        if line.ends_with("end skip") {
            eprintln!("Stopping skip at {}", idx);
            skip_lines = false;
        }

        if skip_lines {
            continue;
        }

        for feature in ALL_FEATURES {
            if line.contains("skip if") && line.contains(feature) && features.is_enabled(feature) {
                eprintln!("Starting skip at {}", idx);
                skip_lines = true;
                continue;
            }

            if line.contains("skip unless")
                && line.contains(feature)
                && features.is_disabled(feature)
            {
                eprintln!("Starting skip at {}", idx);
                skip_lines = true;
                continue;
            }
        }

        lines.push(line);
    }

    std::fs::write(PARSER_FEATURES_Y, lines.join("\n")).unwrap();
}

fn compile_with_bison() {
    match rust_bison_skeleton::process_bison_file(&Path::new(PARSER_FEATURES_Y)) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Failed to generate grammar.\n{:#?}", err);
            std::process::exit(1);
        }
    }
}
