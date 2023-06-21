#![deny(clippy::style)]
#![deny(clippy::perf)]
#![deny(clippy::pedantic)]

mod bf;

use std::io::stdin;
use std::io::stdout;

use bf::BFTerp;

const HELLO_WORLD: &str = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

fn main() {
    let mut out = stdout().lock();
    let mut inp = stdin().lock();
    let mut bfterp = BFTerp::new(0x100, HELLO_WORLD, &mut inp, &mut out).unwrap();
    bfterp.run().unwrap();
}
