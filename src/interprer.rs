use console::Term;

use crate::{Command, Commands, error::BfError};

pub struct Runtime {
    mem: Vec<u8>,
    ptr: usize,
    max_len: usize,
}

#[derive(Clone)]
pub enum InterprerError {
    VariableIntegerOverflow,
    VariableIntegerUnderflow,
    MemoryPointerUnderflow,
}

impl BfError for InterprerError {
    #[inline(always)]
    fn error_type(&self) -> String {
        "Runtime".to_string()
    }

    fn description(&self) -> String {
        match self {
            InterprerError::MemoryPointerUnderflow => "memory pointer underflow",
            InterprerError::VariableIntegerUnderflow => "variable underflow",
            InterprerError::VariableIntegerOverflow => "variable overflow",
        }
        .to_string()
    }
}

impl Default for Runtime {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Runtime {
    #[inline]
    pub fn new() -> Self {
        Self {
            mem: vec![0],
            ptr: 0,
            max_len: 0,
        }
    }

    pub fn exec(&mut self, code: &Commands) -> Result<(), InterprerError> {
        for cmd in &code.0 {
            match cmd {
                Command::Plus => self.change_variable(|x| {
                    x.checked_add(1)
                        .ok_or(InterprerError::VariableIntegerOverflow)
                })?,
                Command::Minus => self.change_variable(|x| {
                    x.checked_sub(1)
                        .ok_or(InterprerError::VariableIntegerUnderflow)
                })?,

                Command::MoveLeft => {
                    self.ptr = self
                        .ptr
                        .checked_sub(1)
                        .ok_or(InterprerError::MemoryPointerUnderflow)?
                }
                Command::MoveRight => self.ptr += 1,

                Command::Put => print!("{}", self.get_at_ptr() as char),
                Command::Get => self.set_at_ptr(Term::stdout().read_char().unwrap() as u8),

                Command::Loop(cmds) => {
                    while self.get_at_ptr() != 0 {
                        self.exec(&Commands(cmds.clone()))?
                    }
                }
            }
        }

        Ok(())
    }

    #[inline]
    fn make_access_at(&mut self, at: usize) {
        if at <= self.max_len {
            return;
        }

        (self.max_len..at).for_each(|_| self.mem.push(0));
        self.max_len = at;
    }

    #[inline]
    fn set_mem_at(&mut self, at: usize, val: u8) {
        self.make_access_at(at);
        self.mem[at] = val;
    }

    #[inline]
    fn set_at_ptr(&mut self, val: u8) {
        self.set_mem_at(self.ptr, val)
    }

    #[inline]
    fn get_mem_at(&mut self, at: usize) -> u8 {
        self.make_access_at(at);
        self.mem[at]
    }

    #[inline]
    fn get_at_ptr(&mut self) -> u8 {
        self.get_mem_at(self.ptr)
    }

    #[inline]
    fn change_variable(
        &mut self,
        mut f: impl FnMut(u8) -> Result<u8, InterprerError>,
    ) -> Result<(), InterprerError> {
        let val = self.get_at_ptr();
        let res = f(val);

        self.set_at_ptr(res.clone()?);

        res.map(|_| ())
    }
}
