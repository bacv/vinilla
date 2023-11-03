use std::cell::RefCell;
use std::rc::Rc;
use vinilla::{Parser, Processor, Term};

fn main() {
    let term = Rc::new(RefCell::new(Term::new(1, 13)));
    let mut state = Parser::new();
    let mut processor = Processor::new(term.clone());

    for byte in "Hello, World!".as_bytes() {
        state.advance(&mut processor, *byte);
    }

    let cells_result: Vec<char> = term.borrow().renderable_content().map(|c| c.c).collect();
    println!("Terminal cells: {:?}", cells_result);
}
