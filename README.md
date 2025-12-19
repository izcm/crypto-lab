# crypto-lab

Small Rust CLI playground for basic number theory routines and simple substitution ciphers.

## What it does

- Runs an interactive menu with two sections: number theory utilities and substitution ciphers.
- Number theory: GCD and extended GCD, prime checks, Euler phi, ord_p, modular inverse (Fermat or extended Euclid), multiplicative order, and primitive roots for supported moduli.
- Substitution ciphers: Caesar shift encrypt/decrypt that keeps non-letters as-is.

## Run it

```
cargo run
```

Follow the prompts to choose a section and enter inputs.

## Project layout

- src/main.rs: menu loop and screen handling.
- src/number_theory: basic helpers plus modular arithmetic utilities (inverse, mod pow, order, primitive roots).
- src/sub_cipher: menu and Caesar cipher solver.

## Notes

- Designed for learning, not production-grade cryptography.
