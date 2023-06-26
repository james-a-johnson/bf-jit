#![deny(clippy::style)]
#![deny(clippy::perf)]
#![deny(clippy::pedantic)]

use std::io::stdin;
use std::io::stdout;

use bf_jit::prog::Interpreter;

const HELLO_WORLD: &str = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

fn main() {
    let mut out = stdout().lock();
    let mut inp = stdin().lock();
    let mut bfterp = Interpreter::new(HELLO_WORLD, 0x100, &mut inp, &mut out).unwrap();
    bfterp.interpret().unwrap();
}
