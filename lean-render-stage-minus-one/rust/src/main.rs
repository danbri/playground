use stage_minus_one::{fixture, render_text, Environment};

fn main() {
    let name = std::env::args().nth(1).unwrap_or_else(|| "nested".into());
    let Some(doc) = fixture(&name) else { eprintln!("unknown fixture: {name}"); std::process::exit(2) };
    println!("{}", render_text(Environment::default(), &doc));
}
