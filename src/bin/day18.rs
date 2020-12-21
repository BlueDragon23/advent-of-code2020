use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

// a + b * c
#[derive(Clone, Debug)]
struct Expression {
    values: Vec<Value>,
    ops: Vec<Op>
}

#[derive(Clone, Debug)]
enum Value {
    Number(u64),
    Expression(Box<Expression>)
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Op {
    Add,
    Multiply
}

fn main() {
    let f = File::open("input/input18_1.txt").unwrap();
    let reader = BufReader::new(f);
    let result = reader
        .lines()
        .map(|line| parse_expression(&mut line.unwrap().chars().into_iter()))
        .map(|e| {
            // print_expression(e.clone());
            // println!("");
            e
        })
        .map(|e| evaluate_expression_part2_2(e))
        .collect::<Vec<_>>();

    println!("{:?}", result.iter().sum::<u64>());
}

fn evaluate_expression_part2_2(expression: Expression) -> u64 {
    // println!("{:?}", expression);
    // Evaluate the first add
    let add_op = expression.ops
        .iter()
        .position(|o| *o == Op::Add);
    match add_op {
        Some(i) => {
            let new_val = evaluate_value(expression.values[i].clone()) + 
                evaluate_value(expression.values[i + 1].clone());
            if expression.ops.len() == 1 {
                new_val
            } else {
                let mut child_values = expression.values.clone();
                let _ = child_values
                    .splice(i..i+2, vec![Value::Number(new_val)])
                    .collect::<Vec<_>>();
                let mut child_ops = expression.ops.clone();
                child_ops.remove(i);
                evaluate_expression_part2_2(Expression { 
                    values: child_values,
                    ops: child_ops
                })
            }
        },
        None => {
            let new_val = evaluate_value(expression.values[0].clone()) * 
                evaluate_value(expression.values[1].clone());
            if expression.ops.len() == 1 {
                new_val
            } else {
                let mut child_values = expression.values.clone();
                let _ = child_values
                    .splice(..2, vec![Value::Number(new_val)])
                    .collect::<Vec<_>>();
                let mut child_ops = expression.ops.clone();
                child_ops.remove(0);
                evaluate_expression_part2_2(Expression { 
                    values: child_values,
                    ops: child_ops
                })
            }
        }
    }
    // If no adds, evaluate multiply
    // If there's ops left, recurse
}

#[allow(dead_code)]
fn evaluate_expression_part2(expression: Expression) -> u64 {
    // Evaluate addition first
    let sum_ops: Vec<usize> = expression.ops
        .iter()
        .enumerate()
        .filter(|(_, op)| **op == Op::Add)
        .map(|(i, _)| i)
        .collect();
    let product_ops: Vec<usize> = expression.ops
        .iter()
        .enumerate()
        .filter(|(_, op)| **op == Op::Multiply)
        .map(|(i, _)| i)
        .collect();
    // 1 + 2 * 3 + 4 * 5 * 6
    //   0   1   2   3   4
    //   3   *   7   * 5 * 6
    // 1 + 2 + 3 + 4
    //   0   1   2
    //   3   5   7
    //       10
    // If adjacent, add + subtract middle?
    let mut sums = vec![];
    for i in sum_ops {
        let left = evaluate_value(expression.values[i].clone());
        let right = evaluate_value(expression.values[i + 1].clone());
        sums.push((i, left, right));
    }
    // End up with Vec<(lower, upper, value)>
    let sum_merged: Vec<(usize, usize, u64)> = sums
        .iter()
        .fold(vec![], |mut xs, (i, left, right)| {
            if xs.is_empty() || xs.last().unwrap().0 != i - 1 {
                // xs is empty or xs.last != i - 1, push it
                xs.push((*i, i + 1, left + right));
            } else {
                // ranges overlap, add right
                let previous = xs.pop().unwrap();
                let new = (previous.0, *i + 1, previous.2 + right);
                xs.push(new);
            }
            xs
        });
    // 1 + 2 * 3 + 4 * 5 + 6
    // 0   1   2   3   4   5
    // (0, 1, 3), (2, 3, 7), (4, 5, 11)
    println!("sum merged: {:?}", sum_merged);
    if product_ops.is_empty() {
        // if there's no products, we already added the entire expression
        return sum_merged[0].2
    }

    // 1 * 2 * 3
    // 
    let mut products = vec![];
    for i in product_ops {
        let left = find_sum_expr(&sum_merged, i)
            .unwrap_or_else(|| (i, i, evaluate_value(expression.values[i].clone())));
        // if i in sums, use that
        let right = find_sum_expr(&sum_merged, i + 1)
            .unwrap_or_else(|| (i + 1, i + 1, evaluate_value(expression.values[i + 1].clone())));
        // Expand out lower/upper bounds
        products.push((left.0, right.1, left.2, right.2));
    }
    // 1 + 2 * 3 + 4 * 5 + 6
    //   3   *   7   *   11
    // 0   1   2   3   4   5
    // (0, 2, 3, 7), (3, 6, 7, 11)
    println!("product merged: {:?}", products);

    products
        .iter()
        .fold(vec![], |mut xs: Vec<(usize, usize, u64)>, (lower, upper, left, right)| {
            if xs.is_empty() || xs.last().unwrap().1 < lower - 1 {
                // xs is empty or xs.last != i - 1, push it
                xs.push((*lower, *upper, left * right));
            } else {
                // ranges overlap, add right
                let previous = xs.pop().unwrap();
                let new = (previous.0, *upper, previous.2 * right);
                xs.push(new);
            }
            xs
        })
        .iter()
        .fold(1, |acc, (_, _, value)| {
            acc * value
        })
}

#[allow(dead_code)]
fn find_sum_expr(sum_merged: &Vec<(usize, usize, u64)>, i: usize) -> Option<(usize, usize, u64)> {
    for (lower, upper, value) in sum_merged {
        if i >= *lower && i <= *upper {
            return Some((*lower, *upper, *value));
        }
    }
    None
}

fn evaluate_value(value: Value) -> u64 {
    match value {
        Value::Number(x) => x,
        Value::Expression(e) => evaluate_expression_part2_2(*e)
    }
}

#[allow(dead_code)]
fn evaluate_expression(expression: Expression) -> u64 {
    let first = expression.values[0].to_owned();
    let mut sum = match first {
        Value::Number(x) => x,
        Value::Expression(e) => evaluate_expression(*e)
    };
    for (i, v) in expression.values.into_iter().skip(1).enumerate() {
        let operand = match v {
            Value::Number(x) => x,
            Value::Expression(e) => evaluate_expression(*e)
        };
        match expression.ops[i] {
            Op::Add => sum += operand,
            Op::Multiply => sum *= operand
        };
    }
    sum
}

#[allow(dead_code)]
fn print_expression(expression: Expression) {
    print!("(");
    for (i, v) in expression.values.into_iter().enumerate() {
        match v {
            Value::Number(x) => print!("{}", x),
            Value::Expression(e) => print_expression(*e)
        }
        if i < expression.ops.len() {
            match expression.ops[i] {
                Op::Add => print!("+"),
                Op::Multiply => print!("*")
            };
        }
    }
    print!(")");
}

fn parse_expression<I>(iterator: &mut I) -> Expression where 
    I: Iterator<Item = char> {
    // if iterator.peekable().next().is_none() {
    //     return
    // }
    let mut values = vec![];
    let mut ops = vec![];
    let a = parse_value(iterator);
    values.push(a);
    loop {
        let next = iterator.next();
        if next.is_none() || next.unwrap() == ')' {
            break
        }
        // we already got the first space
        let op = match iterator.next().unwrap() {
            '*' => Op::Multiply,
            '+' => Op::Add,
            x => panic!("Invalid op {}", x)
        };
        ops.push(op);
        iterator.next(); // space
        let b= parse_value(iterator);
        values.push(b);
    }
    Expression { values, ops }
}

fn parse_value<I>(iterator: &mut I) -> Value where 
    I: Iterator<Item = char> {
    let c = iterator.next().unwrap();
    if c == '(' {
        let expression = parse_expression(iterator);
        Value::Expression(Box::new(expression))
    } else {
        Value::Number(parse_digit(c))
    }
}

fn parse_digit(digit: char) -> u64 {
    digit.to_digit(10).unwrap().into()
}