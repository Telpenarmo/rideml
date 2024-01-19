use std::io::Read;

fn main() {
    let mode = parse_args();
    eprintln!("mode: {:?}", mode);

    match mode {
        Mode::Parse => parse_input(),
        Mode::REPL => {
            eprintln!("Repl mode is not yet implemented.");
            std::process::exit(1);
        }
        Mode::LSP => {
            eprintln!("Lsp mode is not yet implemented.");
            std::process::exit(1);
        }
    }
}

#[derive(Debug)]
enum Mode {
    Parse,
    REPL,
    LSP,
}

fn parse_args() -> Mode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        Mode::Parse
    } else {
        match args[1].as_str().to_lowercase().as_str() {
            "repl" => Mode::REPL,
            "parse" => Mode::Parse,
            "lsp" => Mode::LSP,
            _ => Mode::REPL,
        }
    }
}

fn parse_input() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).expect("stdin");
    let parse = parser::parse(&input);
    println!("{}\n", parse.debug_tree());
}
