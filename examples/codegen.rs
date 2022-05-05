#[cfg(feature = "codegen-example")]
fn main() {
    use serde::Serialize;

    #[derive(Debug, Default, Serialize, Clone)]
    struct TokenId {
        comment: &'static str,
        name: &'static str,
        value: &'static str,
    }

    let tokens: &[TokenId] = &include!("../target/tokens.rs");

    fn print_usage_and_exit() -> ! {
        eprintln!("Usage: codegen_token_ids --template <template.liquid> --write-to <outfile>");
        std::process::exit(1);
    }

    let mut args = std::env::args().collect::<Vec<_>>();
    let mut get_arg = |key: &str| {
        let key_idx = args
            .iter()
            .enumerate()
            .find(|&(_idx, e)| e == key)
            .unwrap_or_else(|| {
                eprintln!("Unable to get {} CLI argument", key);
                print_usage_and_exit()
            })
            .0;
        let _key = args.remove(key_idx);
        if key_idx >= args.len() {
            eprintln!("No {} CLI option given", key);
            print_usage_and_exit();
        }
        let value = args.remove(key_idx);
        value
    };
    let template_path = get_arg("--template");
    let output_path = get_arg("--write-to");

    use lib_ruby_parser_nodes::{reexports::liquid::value, LiquidTemplate};

    let rendered = LiquidTemplate::new(template_path)
        .with_global("tokens", value!(tokens.to_vec()))
        .render();
    std::fs::write(output_path, rendered).unwrap();
}

#[cfg(not(feature = "codegen-example"))]
fn main() {}
