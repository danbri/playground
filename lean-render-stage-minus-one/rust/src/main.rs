use stage_minus_one::{fixture, generated_fixture, render_text, Environment};

fn main() {
    let mut args = std::env::args().skip(1);
    let first = args.next().unwrap_or_else(|| "nested".into());
    let doc = if first == "gen" {
        let n = args.next().and_then(|s| s.parse::<u32>().ok()).unwrap_or_else(|| {
            eprintln!("generated fixture requires an integer index");
            std::process::exit(2)
        });
        generated_fixture(n)
    } else {
        fixture(&first).unwrap_or_else(|| {
            eprintln!("unknown fixture: {first}");
            std::process::exit(2)
        })
    };
    println!("{}", render_text(Environment::default(), &doc));
}
