# Camlot

**C**amlot (or CaMLOT, haven't decided yet) is **a** **M**L-family **L**anguage **O**ptimised for **T**ooling.

Main purpose of this language is to be something for me to write a language server for, giving the ability to make whatever design choices that suit this task.
In general, I work on it to research IDE-friendly traits of PL design, learn tools and techniques used in top language servers, and for fun.

Camlot is very much work-in-progress: for now there's only parser, incomplete typechecker, a simple language server and an almost minimal vscode extension.

Takes strong inspiration from [rust-analyzer](https://github.com/rust-lang/rust-analyzer/).

## Language

As for the language itself, it is very close to a simply typed lambda calculus with let bindings, grouped in top-level, mutually recursive definitions.

Syntax, on the other hand, is a blend of traditional ML-family PLs and Rust.
