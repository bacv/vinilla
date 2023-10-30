use std::{cell::RefCell, rc::Rc};

use vinilla_term::Term;
use vte::{Params, Perform};

pub struct Processor {
    term: Rc<RefCell<Term>>,
}

impl Processor {
    pub fn new(term: Rc<RefCell<Term>>) -> Self {
        Self { term }
    }
}

impl Perform for Processor {
    fn print(&mut self, c: char) {
        let mut term = self.term.borrow_mut();
        term.input(c);
    }

    fn execute(&mut self, _byte: u8) {
        todo!()
    }

    fn hook(&mut self, _params: &Params, _intermediates: &[u8], _ignore: bool, _c: char) {
        todo!()
    }

    fn put(&mut self, _byte: u8) {
        todo!();
    }

    fn unhook(&mut self) {
        todo!();
    }

    fn osc_dispatch(&mut self, _params: &[&[u8]], _bell_terminated: bool) {
        todo!();
    }

    fn csi_dispatch(&mut self, _params: &Params, _intermediates: &[u8], _ignore: bool, _c: char) {
        todo!();
    }

    fn esc_dispatch(&mut self, _intermediates: &[u8], _ignore: bool, _byte: u8) {
        todo!();
    }
}
