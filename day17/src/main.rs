use core::panic;

use aoclib::{input, output};

#[derive(Debug)]
#[allow(dead_code)] // std::mem::transmute
enum Op {
    /// The adv instruction (opcode 0) performs division. The numerator is the value in the A register. The denominator is found by raising 2 to the power of the instruction's combo operand. (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.) The result of the division operation is truncated to an integer and then written to the A register.
    Adv,
    /// The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand, then stores the result in register B.
    Bxl,
    /// The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the B register.
    Bst,
    /// The jnz instruction (opcode 3) does nothing if the A register is 0. However, if the A register is not zero, it jumps by setting the instruction pointer to the value of its literal operand; if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
    Jnz,
    /// The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)
    Bxc,
    /// The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value. (If a program outputs multiple values, they are separated by commas.)
    Out,
    /// The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register. (The numerator is still read from the A register.)
    Bdv,
    /// The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register. (The numerator is still read from the A register.)
    Cdv,
}

impl From<i8> for Op {
    fn from(value: i8) -> Self {
        assert!((0..8).contains(&value));
        unsafe { std::mem::transmute(value) }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operand {
    Literal(i64),
    RegA,
    RegB,
    RegC,
}

impl From<i8> for Operand {
    fn from(value: i8) -> Self {
        Self::Literal(value as i64)
    }
}

impl Operand {
    fn as_combo(&self) -> Operand {
        match *self {
            Operand::Literal(v) => {
                if (0..=3).contains(&v) {
                    Operand::Literal(v)
                } else {
                    match v {
                        4 => Self::RegA,
                        5 => Self::RegB,
                        6 => Self::RegC,
                        _ => panic!(),
                    }
                }
            }
            x => x,
        }
    }
}

#[derive(Debug)]
struct Instruction {
    op: Op,
    v: Operand,
}

struct Computer {
    a: i64,
    b: i64,
    c: i64,
    ip: usize,
    rom: Vec<i8>,
    stdout: Vec<i8>,
}

impl Computer {
    fn new(a: i64, b: i64, c: i64, rom: Vec<i8>) -> Self {
        Computer {
            a,
            b,
            c,
            ip: 0,
            rom,
            stdout: Vec::new(),
        }
    }

    fn operand(&self, o: Operand) -> i64 {
        match o.as_combo() {
            Operand::Literal(v) => v,
            Operand::RegA => self.a,
            Operand::RegB => self.b,
            Operand::RegC => self.c,
        }
    }

    pub fn run(&mut self) -> String {
        while let Some(ist) = self.next() {
            self.execute(ist)
        }
        self.flush()
    }

    fn execute(&mut self, ist: Instruction) {
        match ist.op {
            Op::Adv => {
                self.a >>= self.operand(ist.v);
            }
            Op::Bxl => {
                if let Operand::Literal(l) = ist.v {
                    self.b ^= l;
                } else {
                    unimplemented!("BXL only supports literal operands.");
                }
            }
            Op::Bst => {
                self.b = self.operand(ist.v) & 0b111;
            }
            Op::Jnz => {
                if let Operand::Literal(addr) = ist.v {
                    if self.a != 0 {
                        self.ip = addr as usize;
                    }
                } else {
                    unimplemented!("JNZ only supports literal operands.");
                }
            }
            Op::Bxc => {
                self.b ^= self.c;
            }
            Op::Out => {
                let val = (self.operand(ist.v) & 0b111) as i8;
                self.stdout.push(val);
            }
            Op::Bdv => {
                self.b = self.a >> self.operand(ist.v);
            }
            Op::Cdv => {
                self.c = self.a >> self.operand(ist.v);
            }
        };
    }

    /// Flushes stdout and returns it as a String.
    fn flush(&mut self) -> String {
        let str = self
            .stdout
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>();
        self.stdout.clear();
        str.join(",")
    }
}

impl Iterator for Computer {
    type Item = Instruction;
    /// Fetches the next instruction
    fn next(&mut self) -> Option<Self::Item> {
        if self.ip >= self.rom.len() {
            None
        } else {
            let op = Op::from(self.rom[self.ip]);
            let v = Operand::from(self.rom[self.ip + 1]);
            self.ip += 2;
            Some(Instruction { op, v })
        }
    }
}

fn main() {
    let input = input("input");

    let mut lines = input.lines();
    let mut regs = [0; 3];
    (0..3).for_each(|i| {
        regs[i] = lines
            .next()
            .unwrap()
            .split_once(":")
            .unwrap()
            .1
            .trim()
            .parse()
            .unwrap();
    });
    lines.next(); // empty line
    let program = lines.next().unwrap().split_once(":").unwrap().1.trim();
    let rom: Vec<i8> = program.split(",").flat_map(|x| x.parse()).collect();

    let mut computer = Computer::new(regs[0], regs[1], regs[2], rom);

    p1(&mut computer);
    p2(&mut computer);
}

#[inline]
fn p1(computer: &mut Computer) {
    output(computer.run());
}

#[inline]
fn p2(computer: &mut Computer) {
    output(find(&computer.rom, 0).expect("No solution."));
}

/// Recurse over rom checking and removing the last element and build up the answer from there.
/// This is COMPLETELY DEPENDENT ON THE PUZZLE INPUT and only works if the program ends in 5,5,0,3,3,0.
/// Using this out of the box will not work and requires changes at the marked region.
fn find(rom: &[i8], mut answer: i64) -> Option<i64> {
    if rom.is_empty() {
        return Some(answer);
    }
    answer <<= 3;
    for i in 0..8 {
        // vvv THIS IS INPUT DEPENDENT vvv
        let a = answer + i;
        let mut b = a & 7;
        b ^= 6;
        let c = a >> b;
        b ^= c;
        b ^= 4;
        // ^^^ THIS IS INPUT DEPENDENT ^^^
        if (b & 7) as i8 == *rom.last().unwrap() {
            match find(rom.split_last().unwrap().1, a) {
                Some(v) => return Some(v),
                None => continue,
            }
        }
    }
    None
}
