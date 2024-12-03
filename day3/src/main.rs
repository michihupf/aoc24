use aoclib::{input, output};

#[derive(Debug, Clone)]
struct Mul {
    pub lhs: i32,
    pub rhs: i32,
    pub enabled: bool,
}

impl Mul {
    fn calc(&self, ignore_enalbe: bool) -> i32 {
        if ignore_enalbe || self.enabled {
            self.lhs * self.rhs
        } else {
            0
        }
    }

    fn new(lhs: i32, rhs: i32) -> Self {
        Mul {
            lhs,
            rhs,
            enabled: true,
        }
    }
}

#[derive(Debug)]
enum ParserState {
    FindToken,
    FindNum(u8),
}

fn main() {
    use ParserState::*;
    let input = input("input");

    // parse the input
    let mut state = FindToken;
    let mut current_mul = Mul::new(0, 0);
    let mut i = 0;
    let mut do_muls = true;

    let mut muls = Vec::new();
    while i < input.len() {
        match state {
            FindToken => {
                while i + 4 <= input.len() {
                    if &input[i..i + 4] == "mul(" {
                        state = FindNum(b',');
                        // shift i by 3 to skip 'ul('
                        i += 3;
                        break;
                    } else if &input[i..i + 4] == "do()" {
                        do_muls = true;
                        i += 3;
                        break;
                    } else if i + 7 <= input.len() && &input[i..i + 7] == "don't()" {
                        do_muls = false;
                        i += 6;
                        break;
                    }
                    i += 1;
                }
            }
            FindNum(end_char) => {
                // match number characters until ','
                let mut num: Vec<u8> = Vec::new();
                let mut ends_with_char = false;
                while i < input.len() {
                    let c = input.as_bytes()[i];
                    if c.is_ascii_digit() {
                        num.push(c);
                    } else {
                        if c == end_char {
                            ends_with_char = true;
                        }
                        break;
                    }
                    i += 1;
                }
                if ends_with_char && !num.is_empty() {
                    if end_char == b',' {
                        current_mul.lhs = String::from_utf8(num).unwrap().parse::<i32>().unwrap();
                        state = FindNum(b')');
                    } else {
                        current_mul.rhs = String::from_utf8(num).unwrap().parse::<i32>().unwrap();
                        current_mul.enabled = do_muls;
                        muls.push(current_mul.clone());
                        state = FindToken;
                    }
                } else {
                    state = FindToken;
                }
            }
        }
        i += 1;
    }

    p1(&muls);
    p2(&muls);
}

#[inline]
fn p1(muls: &[Mul]) {
    let result: i32 = muls.iter().map(|m| m.calc(true)).sum();

    output(result);
}

#[inline]
fn p2(muls: &[Mul]) {
    let result: i32 = muls.iter().map(|m| m.calc(false)).sum();

    output(result);
}
