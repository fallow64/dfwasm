#![no_std]
#![no_main]

mod df {
    mod imports {
        extern "C" {
            pub fn putc(c: u8);
            pub fn putstr(s: *const u8, len: usize);
        }
    }

    pub fn putc(c: u8) {
        unsafe { imports::putc(c) }
    }

    pub fn putstr(s: &[u8]) {
        unsafe { imports::putstr(s.as_ptr(), s.len()) }
    }
}

const TAPE_SIZE: usize = 100;

struct Program<'a> {
    code: &'a [u8],
    pc: usize,
    tape: [u8; TAPE_SIZE],
    tape_ptr: usize,
}

impl<'a> Program<'a> {
    pub fn new(code: &'a [u8]) -> Self {
        Self {
            code,
            pc: 0,
            tape: [0; TAPE_SIZE],
            tape_ptr: 0,
        }
    }

    pub fn step(&mut self) -> bool {
        if self.pc < 0 || self.pc >= self.code.len() {
            return false;
        }

        match self.code[self.pc] {
            b'>' => {
                self.tape_ptr = (self.tape_ptr.wrapping_add(1)) % TAPE_SIZE;
            }
            b'<' => {
                self.tape_ptr = (self.tape_ptr.wrapping_sub(1)) % TAPE_SIZE;
            }
            b'+' => {
                if let Some(val) = self.tape.get_mut(self.tape_ptr) {
                    *val = val.wrapping_add(1);
                }
            }
            b'-' => {
                if let Some(val) = self.tape.get_mut(self.tape_ptr) {
                    *val = val.wrapping_sub(1);
                }
            }
            b'.' => {
                if let Some(val) = self.tape.get(self.tape_ptr) {
                    df::putc(*val);
                }
            }
            b',' => {
                // Input handling is not implemented
            }
            b'[' => {
                if self.tape.get(self.tape_ptr).copied() == Some(0) {
                    let mut bracket_count = 1;
                    while bracket_count > 0 && self.pc < self.code.len() - 1 {
                        self.pc += 1;
                        if self.code.get(self.pc).copied() == Some(b'[') {
                            bracket_count += 1;
                        } else if self.code.get(self.pc).copied() == Some(b']') {
                            bracket_count -= 1;
                        }
                    }
                }
            }
            b']' => {
                if self.tape.get(self.tape_ptr).copied() != Some(0) {
                    let mut bracket_count = 1;
                    while bracket_count > 0 && self.pc > 0 {
                        self.pc -= 1;
                        if self.code.get(self.pc).copied() == Some(b']') {
                            bracket_count += 1;
                        } else if self.code.get(self.pc).copied() == Some(b'[') {
                            bracket_count -= 1;
                        }
                    }
                }
            }
            _ => {}
        }

        self.pc = self.pc.wrapping_add(1);

        true
    }
}

#[no_mangle]
pub extern "C" fn execute(program: *const u8, size: usize) {
    // Convert the pointer to a slice
    let slice = unsafe { core::slice::from_raw_parts(program, size) };

    // Create a new program instance
    let mut prog = Program::new(slice);

    while prog.step() {
        // Continue stepping through the program
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable();
}
