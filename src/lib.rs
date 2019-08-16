use std::io::{stdin, stdout, Read, Stdin, Stdout, Write};

mod utils;
use utils::HexDump;

pub mod parser;
use parser::{parse, Token};

pub struct BF<T, U> {
    mem: Vec<u8>,
    ptr: usize,
    tokens: Vec<Token>,
    token_ptr: usize,
    out_sink: T,
    input_src: U,
}

impl BF<Stdout, Stdin> {
    pub fn new() -> Self {
        BF {
            mem: vec![0],
            ptr: 0,
            tokens: Vec::new(),
            token_ptr: 0,
            out_sink: stdout(),
            input_src: stdin(),
        }
    }
}

impl<T: Write, U: Read> BF<T, U> {
    pub fn with_sinks(out_sink: T, input_src: U) -> Self {
        BF {
            mem: vec![0],
            ptr: 0,
            tokens: Vec::new(),
            token_ptr: 0,
            out_sink,
            input_src,
        }
    }

    fn inc_mem(&mut self, n: u8) {
        let num: u8 = *self.mem.get(self.ptr).unwrap();
        let (num, _) = num.overflowing_add(n);
        self.mem[self.ptr] = num;
    }

    fn inc_ptr(&mut self, n: i32) {
        if n < 0 {
            self.ptr = self.ptr.saturating_sub(n.abs() as usize);
        } else {
            let extra_memory_cells = n - (self.mem.len() - 1 - self.ptr) as i32;
            if extra_memory_cells > 0 {
                let mut v = vec![0; extra_memory_cells as usize];
                self.mem.append(&mut v);
            }
            self.ptr += n as usize;
        }
    }

    fn read_byte(&mut self) {
        let mut buf = [0];
        self.input_src.read_exact(&mut buf).ok();
        self.mem[self.ptr] = buf[0];
    }

    fn write_byte(&mut self) {
        self.out_sink.write(&[self.mem[self.ptr]]).ok();
        let _ = self.out_sink.flush().is_ok();
    }

    fn open_loop(&mut self, loop_end: usize) {
        if self.mem[self.ptr] == 0 {
            self.token_ptr = loop_end;
        }
    }

    fn close_loop(&mut self, loop_start: usize) {
        if self.mem[self.ptr] != 0 {
            self.token_ptr = loop_start;
        }
    }

    pub fn interpret(&mut self, file_contents: Vec<u8>) {
        self.tokens = parse(file_contents);
        self.token_ptr = 0;
        loop {
            if self.token_ptr >= self.tokens.len() {
                break;
            }

            let cur_token = self.tokens[self.token_ptr].clone();

            match cur_token {
                Token::Inc(n) => self.inc_mem(n),
                Token::Shift(n) => self.inc_ptr(n),
                Token::Write => self.write_byte(),
                Token::Read => self.read_byte(),
                Token::LoopStart(_, end) => self.open_loop(end),
                Token::LoopEnd(start, _) => self.close_loop(start),
            }

            self.token_ptr += 1;
        }
    }

    fn reset_state(&mut self) {
        self.mem = vec![0];
        self.ptr = 0;
    }

    pub fn start_interactive_loop(&mut self) {
        let mut buf = String::new();
        loop {
            buf.clear();
            print!(">>> ");
            stdout().flush().unwrap();
            stdin().read_line(&mut buf).unwrap();
            match buf.trim() {
                "dump" => {
                    println!("-----------");
                    println!("Memory Dump");
                    println!("-----------");
                    HexDump::new(&self.mem).dump().unwrap();
                }
                "reset" => self.reset_state(),
                _ => self.interpret(buf.clone().into_bytes()),
            }
        }
    }
}
