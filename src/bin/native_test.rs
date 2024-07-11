#![deny(clippy::style)]
#![deny(clippy::perf)]
#![deny(clippy::pedantic)]

use std::io::stdin;
use std::io::stdout;

use bf_jit::native::Native;
use bf_jit::prog::Interpreter;

const TEST_PROG: &str = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

fn main() {
    let mut out = stdout().lock();
    let mut inp = stdin().lock();
    let mut bfterp = Interpreter::new(TEST_PROG, 0x100, &mut inp, &mut out).unwrap();
    bfterp.optimize();
    let mut fast: Native<32, _, _> = bfterp.into();
    fast.run();
    println!("{:?}", &fast[0..8]);
}
