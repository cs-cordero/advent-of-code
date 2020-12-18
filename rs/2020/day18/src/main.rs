use advent_of_code::*;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Term {
    Scalar(u64),
    Add,
    Multiply,
    Open,
    Close,
}

fn main() {
    let input = read_input_as_lines("2020/day18/src/input.txt");
    let answer1 = input
        .iter()
        .map(|line| evaluate_math(line, false))
        .sum::<u64>();

    let answer2 = input
        .iter()
        .map(|line| evaluate_math(line, true))
        .sum::<u64>();

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}

fn evaluate_math(s: &str, add_has_precedence_over_mul: bool) -> u64 {
    if s.is_empty() {
        return 0;
    }

    let mut stack = Vec::<Term>::with_capacity(100);
    let parsed = s
        .replace('(', "( ")
        .replace(')', " )")
        .split(' ')
        .map(|term| match term {
            "+" => Term::Add,
            "*" => Term::Multiply,
            "(" => Term::Open,
            ")" => Term::Close,
            _ => Term::Scalar(
                term.parse()
                    .unwrap_or_else(|_| panic!("Invalid value {}", term)),
            ),
        })
        .collect::<Vec<_>>();

    for term in parsed {
        match term {
            Term::Scalar(..) => {
                stack.push(term);
                stack = collapse_stack(stack, false, add_has_precedence_over_mul);
            }
            Term::Add | Term::Multiply | Term::Open => {
                stack.push(term);
            }
            Term::Close => {
                stack = collapse_stack(stack, true, add_has_precedence_over_mul);
                stack = collapse_stack(stack, false, add_has_precedence_over_mul);
            }
        }
    }

    stack = collapse_stack(stack, true, add_has_precedence_over_mul);
    get_from_scalar(stack.pop().unwrap())
}

fn collapse_stack(mut stack: Vec<Term>, remove_open: bool, addition_precedence: bool) -> Vec<Term> {
    if stack.is_empty() {
        return stack;
    }

    let mut value = get_from_scalar(stack.pop().unwrap());
    while stack
        .last()
        .filter(|&term| *term != Term::Open)
        .filter(|&term| (!addition_precedence || (remove_open || *term != Term::Multiply)))
        .is_some()
    {
        let operation = stack.pop().unwrap();
        let lhs = get_from_scalar(stack.pop().unwrap());
        match operation {
            Term::Add => value += lhs,
            Term::Multiply => value *= lhs,
            _ => panic!("Invalid operation {:?}", operation),
        };
    }
    if remove_open && stack.last().filter(|&term| *term == Term::Open).is_some() {
        stack.pop().unwrap();
    }
    stack.push(Term::Scalar(value));
    stack
}

#[inline]
fn get_from_scalar(term: Term) -> u64 {
    match term {
        Term::Scalar(value) => value,
        _ => panic!("Term is not a Term::Scalar! [{:?}]", term),
    }
}
