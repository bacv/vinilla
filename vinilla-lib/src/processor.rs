use alloc::rc::Rc;
use core::cell::RefCell;

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

    fn execute(&mut self, byte: u8) {
        match byte as char {
            '\n' => {
                let mut term = self.term.borrow_mut();
                term.new_line();
            }
            '\r' => {
                let mut term = self.term.borrow_mut();
                term.carriage_return();
            }
            '\x08' => {
                // Backspace
                let mut term = self.term.borrow_mut();
                term.backspace();
            }
            _ => {
                todo!()
            }
        }
    }

    fn hook(&mut self, _params: &Params, _intermediates: &[u8], _ignore: bool, _c: char) {
        todo!()
    }

    fn put(&mut self, byte: u8) {
        self.print(byte as char);
    }

    fn unhook(&mut self) {
        todo!();
    }

    fn osc_dispatch(&mut self, _params: &[&[u8]], _bell_terminated: bool) {
        todo!();
    }

    fn csi_dispatch(&mut self, params: &Params, _intermediates: &[u8], _ignore: bool, c: char) {
        let mut term = self.term.borrow_mut();
        let mut params_iter = params.iter();

        let mut next_param_or = |default: u16| match params_iter.next() {
            Some(&[param, ..]) if param != 0 => param,
            _ => default,
        };

        match c {
            'C' => {
                // Move cursor to the right
                let n = next_param_or(1) as usize;
                term.move_cursor_right(n);
            }
            'A' => {
                // Move cursor up
                let n = next_param_or(1) as usize;
                term.move_cursor_up(n);
            }
            _ => {
                todo!()
            }
        }
    }

    fn esc_dispatch(&mut self, _intermediates: &[u8], _ignore: bool, _byte: u8) {
        todo!();
    }
}
