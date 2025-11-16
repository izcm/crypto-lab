use crate::number_theory::factors::{gcd, ord_p};
use crate::number_theory::modular_arithmetic::inverse::inverse;
use std::io::{self, Write};

pub fn run() {
    loop {
        println!("\n=== 🧮 Number Theory ===");
        println!("1) Extended gcd(a, b)");
        println!("2) Compute ord_p(n, p)");
        println!("3) Compute modular inverse");

        match ask("👉 Enter your choice (or type 'exit') ").trim() {
            "1" => {
                let a: i64 = ask("a = ").parse().unwrap_or(0);
                let b: i64 = ask("b = ").parse().unwrap_or(0);
                let (g, u, v) = gcd::extended_gcd(a, b);
                println!("gcd = {}, u = {}, v = {}", g, u, v);
            }
            "2" => {
                let n: u64 = ask("n = ").parse().unwrap_or(0);
                let p: u64 = ask("p = ").parse().unwrap_or(0);
                println!("ord_{}({}) = {}", p, n, ord_p::ord_p(n, p));
            }
            "3" => {
                let a: u64 = ask("a = ").parse().unwrap_or(0);
                let p: u64 = ask("p = ").parse().unwrap_or(0);
                match inverse(a, p) {
                    Some(inv) => println!("{}⁻¹ mod {} = {}", a, p, inv),
                    None => println!("No inverse exists because gcd({}, {}) ≠ 1", a, p),
                }
            }
            "exit" => {
                println!("💅 bye.");
                break;
            }
            _ => println!("❌ invalid choice"),
        }
    }
}

fn ask(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
