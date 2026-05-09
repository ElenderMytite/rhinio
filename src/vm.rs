use std::collections::HashMap;

use crate::parser::{AstNode, Comparison, Computation, Expression, Logic, Operation, Value};
#[derive(Debug, Clone, Copy)]
enum StackValue {
    Int(isize),
    Bool(bool),
}
pub enum Command {
    //computation
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    // stack operations
    Put(StackValue),
    Pop,
    // comparision
    Gt,
    Geq,
    Ls,
    Leq,
    Eq,
    Neq,
    // logic
    Not,
    And,
    Or,
    Xor,
    Nor,
    Nand,
    // variable operations
    Load(usize),
    Store(usize),
}
pub fn ir(root: AstNode, variables: &mut HashMap<String, usize>) -> Vec<Command> {
    let mut commands = Vec::new();
    match root {
        AstNode::Expression(expression) => match expression.operation {
            Some(op) => match op {
                Operation::Comparison(comparison) => {
                    assert_eq!(expression.left.len(), expression.right.len());
                    for i in 0..expression.left.len() {
                        commands.append(&mut ir_value(&expression.left[i], variables));
                        commands.append(&mut ir_value(&expression.right[i], variables));
                        commands.push(match comparison {
                            Comparison::Greater => Command::Gt,
                            Comparison::Less => Command::Ls,
                            Comparison::Equal => Command::Eq,
                            Comparison::GreaterOrEqual => Command::Geq,
                            Comparison::LessOrEqual => Command::Leq,
                            Comparison::NotEqual => Command::Neq,
                        })
                    }
                }
                Operation::Computation(computation) => match computation {
                    Computation::Add => {
                        let mut pasted = 0;
                        for (idx, value) in expression.left.iter().enumerate() {
                            if pasted >= 2 {
                                commands.push(Command::Add);
                            }
                            commands.append(&mut ir_value(value, variables));
                            pasted += 1;
                        }
                        for (idx, value) in expression.right.iter().enumerate() {
                            if pasted >= 2 {
                                commands.push(Command::Add);
                            }
                            commands.append(&mut ir_value(value, variables));
                        }
                    }
                    Computation::Sub => todo!(),
                    Computation::Mul => todo!(),
                    Computation::Div => todo!(),
                    Computation::Mod => todo!(),
                },
                Operation::Logic(logic) => todo!(),
            },
            None => {
                if expression.left.len() == 1 {
                    commands.append(&mut ir_value(&expression.left[0], variables));
                }
            }
        },
        AstNode::BlockVec(ast_nodes) => todo!(),
        AstNode::BlockCode(ast_nodes) => todo!(),
    }
    commands
}
fn ir_value(value: &Value, variables: &mut HashMap<String, usize>) -> Vec<Command> {
    let mut commands = Vec::new();
    match value {
        Value::Name(s) => {
            if variables.contains_key(&s) {
                commands.push(Command::Load(variables[&s]));
            } else {
                variables.insert(*s, variables.len());
                commands.push(Command::Load(variables[&s]));
            }
        }
        Value::Number(x) => commands.push(Command::Put(StackValue::Int(*x))),
        Value::Expression(expr) => {
            commands.append(&mut ir(AstNode::Expression(*expr), variables));
        }
    }
    commands
}
