use std::ops;

use aoclib::{input, output};

#[derive(Debug, PartialEq, Eq)]
struct Rule {
    pub lower: i32,
    pub upper: i32,
}

#[derive(Debug)]
struct Update {
    pub values: Vec<i32>,
    pub correct: bool,
}

fn main() {
    let input = input("input");

    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rules: Vec<Rule> = rules
        .lines()
        .map(|l| l.split_once("|").unwrap())
        .map(|(l, r)| (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap()))
        .map(|(lhs, rhs)| Rule {
            lower: lhs,
            upper: rhs,
        })
        .collect();
    let mut updates: Vec<Update> = updates
        .lines()
        .map(|l| l.split(",").map(|x| x.parse::<i32>().unwrap()).collect())
        .map(|values| Update {
            values,
            correct: false,
        })
        .collect();

    p1(&rules, &mut updates);
    p2(&rules, &updates);
}

#[derive(Debug)]
struct RuledStack<'a> {
    buf: Vec<i32>,
    rules: &'a [Rule],
}

impl<'a> RuledStack<'a> {
    fn new(rules: &'a [Rule]) -> RuledStack<'a> {
        RuledStack {
            buf: Vec::new(),
            rules,
        }
    }

    /// Pushes element on the stack. Returning false if rules are harmed.
    fn push(&mut self, x: i32) -> bool {
        // if x|buf is a rule we can't satisfy it
        for rule in self.rules.iter().filter(|&r| r.lower == x) {
            // check if buffer contains rhs
            if self.buf.iter().any(|&b| rule.upper == b) {
                // we broke the rule
                return false;
            }
        }
        // all rules satisfied
        self.buf.push(x);
        true
    }

    /// Inserts the element at the correct location to satisfy all rules.
    fn insert(&mut self, x: i32) {
        // iterate from left to right, don't pass upper bounds
        let len_before = self.len();
        for (i, &b) in self.buf.iter().enumerate() {
            if self.rules.contains(&Rule { lower: x, upper: b }) {
                // place it here
                self.buf.insert(i, x);
                break;
            }
        }
        if len_before == self.len() {
            self.buf.push(x);
        }
    }

    fn clear(&mut self) {
        self.buf.clear();
    }

    fn len(&self) -> usize {
        self.buf.len()
    }
}

impl<'a> ops::Index<usize> for RuledStack<'a> {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buf[index]
    }
}

/// Solves part one and flags correct updates.
#[inline]
fn p1(rules: &[Rule], updates: &mut Vec<Update>) {
    let mut stack = RuledStack::new(rules);
    let mut sum = 0;

    for update in updates {
        for &x in update.values.iter() {
            if !stack.push(x) {
                break;
            }
        }
        if stack.len() == update.values.len() {
            // valid update
            sum += stack[stack.len() / 2];
            update.correct = true;
        }
        stack.clear();
    }

    output(sum);
}

#[inline]
fn p2(rules: &[Rule], updates: &[Update]) {
    let mut stack = RuledStack::new(rules);
    let mut sum = 0;

    for update in updates.iter().filter(|&u| !u.correct) {
        for &x in update.values.iter() {
            stack.insert(x);
        }
        sum += stack[stack.len() / 2];
        stack.clear();
    }

    output(sum);
}
