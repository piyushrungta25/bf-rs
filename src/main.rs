use std::env::args;
use std::fs::File;
use std::io::{stdin, stdout, Read, Write};

struct BF {
    mem: Vec<u8>,
    ptr: usize,
    file_contents: Vec<u8>,
    file_ptr: usize,
}

impl BF {
    fn new(file_path: &str) -> Self {
        let mut file = File::open(file_path).unwrap();
        let mut buf = Vec::new();
        let _ = file.read_to_end(&mut buf);
        BF {
            mem: vec![0],
            ptr: 0,
            file_contents: buf,
            file_ptr: 0,
        }
    }

    fn inc_mem(&mut self) {
        let num: u8 = *self.mem.get(self.ptr).unwrap();
        let (num, _) = num.overflowing_add(1);
        self.mem[self.ptr] = num;
    }

    fn dec_mem(&mut self) {
        let num: u8 = *self.mem.get(self.ptr).unwrap();
        let (num, _) = num.overflowing_sub(1);
        self.mem[self.ptr] = num;
    }

    fn inc_ptr(&mut self) {
        if self.mem.len() == self.ptr + 1 {
            self.mem.push(0);
        }

        self.ptr += 1;
    }

    fn dec_ptr(&mut self) {
        if self.ptr > 0 {
            self.ptr -= 1;
        }
    }

    fn read_byte(&mut self) {
        let byte = stdin().bytes().next().unwrap().unwrap();
        self.mem[self.ptr] = byte;
    }

    fn write_byte(&mut self) {
        stdout().write(&[self.mem[self.ptr]]).ok();
        let _ = stdout().flush().is_ok();
    }

    fn open_loop(&mut self) {
        if self.mem[self.ptr] == 0 {
            let mut open_count = 0;
            loop {
                self.file_ptr += 1;
                if self.file_ptr >= self.file_contents.len() {
                    break;
                }

                if self.file_contents[self.file_ptr] == '[' as u8 {
                    open_count += 1;
                } else if self.file_contents[self.file_ptr] == ']' as u8 {
                    if open_count == 0 {
                        break;
                    } else {
                        open_count -= 1;
                    }
                }
            }
        }
    }

    fn close_loop(&mut self) {
        if self.mem[self.ptr] != 0 {
            let mut close_count = 0;
            loop {
                if self.file_ptr == 0 || self.file_ptr == 1 {
                    self.file_ptr = 0;
                    break;
                }
                self.file_ptr -= 1;

                if self.file_contents[self.file_ptr] == ']' as u8 {
                    close_count += 1;
                } else if self.file_contents[self.file_ptr] == '[' as u8 {
                    if close_count == 0 {
                        break;
                    } else {
                        close_count -= 1;
                    }
                }
            }
        }
    }

    fn interpret(&mut self) {
        loop {
            if self.file_ptr >= self.file_contents.len() {
                break;
            }

            let cur_char = self.file_contents[self.file_ptr];

            match cur_char as char {
                '+' => self.inc_mem(),
                '-' => self.dec_mem(),
                '>' => self.inc_ptr(),
                '<' => self.dec_ptr(),
                '[' => self.open_loop(),
                ']' => self.close_loop(),
                '.' => self.write_byte(),
                ',' => self.read_byte(),
                _ => {} // comment
            }

            self.file_ptr += 1;
        }
    }
}

fn main() {
    let file_path = args().nth(1).unwrap();
    let mut bf = BF::new(&*file_path);
    bf.interpret();
}
