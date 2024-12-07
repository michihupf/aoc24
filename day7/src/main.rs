use aoclib::{input, output};

#[derive(Debug)]
struct Expr {
    result: u64,
    operands: Vec<u64>,
}

impl From<&str> for Expr {
    fn from(value: &str) -> Self {
        let (res, r) = value.split_once(":").unwrap();
        let operands = r
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        Expr {
            result: res.parse::<u64>().unwrap(),
            operands,
        }
    }
}

fn main() {
    let input = input("input");

    // fill expression list
    let exprs: Vec<Expr> = input.lines().map(Expr::from).collect();

    p1(&exprs);
    p2(&exprs);
}

#[inline]
fn p1(exprs: &[Expr]) {
    output(
        exprs
            .iter()
            .filter(|expr| is_possible_2ops(expr))
            .map(|e| e.result)
            .sum::<u64>(),
    );
}

/// Returns true of the expression can be calculated using (+) and (*).
fn is_possible_2ops(expr: &Expr) -> bool {
    // we have 2^(|expr.operands| - 1) possiblities, we can use bitmasks
    let mut ops = 0u32;
    loop {
        // apply ops from bitmask: 0=+, 1=*
        let mut result = expr.operands[0];
        for (i, x) in expr.operands.iter().skip(1).enumerate() {
            match ops & (1 << i) != 0 {
                false => result += x,
                true => result *= x,
            }
            if result > expr.result {
                break;
            }
        }
        if result == expr.result {
            break true;
        }
        ops += 1;
        if ops >= (1 << expr.operands.len()) {
            // we went through all combinations
            break false;
        }
    }
}

#[inline]
fn p2(exprs: &[Expr]) {
    output(
        exprs
            .iter()
            .filter(|e| is_possible_3ops(e))
            .map(|e| e.result)
            .sum::<u64>(),
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Operator {
    Add,
    Mul,
    Cat,
}

impl Operator {
    fn apply(&self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Operator::Add => lhs + rhs,
            Operator::Mul => lhs * rhs,
            Operator::Cat => {
                // use string conversion
                (lhs.to_string() + &rhs.to_string()).parse::<u64>().unwrap()
            }
        }
    }

    /// Transforms the Operator to the next one in line. Returns true if "carry" occured.
    fn next(&mut self) -> bool {
        let op = match self {
            Operator::Add => Operator::Mul,
            Operator::Mul => Operator::Cat,
            Operator::Cat => Operator::Add,
        };
        *self = op;
        op == Operator::Add
    }
}

fn is_possible_3ops(expr: &Expr) -> bool {
    // we now have 3^(|expr.operands| - 1) possibilities, so no bitmask :(
    let mut ops = vec![Operator::Add; expr.operands.len() - 1];

    loop {
        // apply ops
        let mut result = expr.operands[0];
        for (i, &x) in expr.operands.iter().skip(1).enumerate() {
            result = ops[i].apply(result, x);
            if result > expr.result {
                break;
            }
        }
        if result == expr.result {
            break true;
        }
        // create next possibility
        for op in ops.iter_mut() {
            if !op.next() {
                // no carry occured, stop the increment
                break;
            }
        }
        if ops.iter().all(|op| *op == Operator::Add) {
            // we went full circle
            break false;
        }
    }
}
