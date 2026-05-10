use super::expression::ir_expression;
use super::value::ir_value;
use super::*;
use crate::parser::{IterationExpr, Operation, Value, VectorOp};
use std::collections::HashMap;
pub(super) fn ir_iteration(
    iteration: &IterationExpr,
    variables: &mut HashMap<String, usize>,
    index: usize,
    outer: Option<Operation>,
) -> Vec<Command> {
    let mut commands = Vec::new();
    match iteration.operation {
        Some(Operation::Vector(op)) => match op {
            VectorOp::Unpack => {
                // assume vector is on top of the stack
                // create variables for the index and the vector pointer
                let mut k: usize = 0;
                let idx: usize;
                loop {
                    if variables.contains_key(&format!("--idx-{}", k)) {
                        k += 1;
                    } else {
                        idx = register_variable(variables, format!("--idx-{}", k));
                        break;
                    }
                }
                k = 0;
                let vec_ptr: usize;
                loop {
                    if variables.contains_key(&format!("--vec-{}", k)) {
                        k += 1;
                    } else {
                        vec_ptr = register_variable(variables, format!("--vec-{}", k));
                        break;
                    }
                }
                for value in iteration.left.iter().chain(iteration.right.iter()) {
                    let vector: usize = match value {
                        Value::Name(s) => register_variable(variables, s.clone()),
                        _ => panic!("Non-name node found inside vector unpacking!"),
                    };
                    commands.push(Command::Load(vector));
                    commands.push(Command::Store(vec_ptr));
                    commands.push(Command::Put(StackValue::Int(0)));
                    commands.push(Command::Store(idx));
                    // put neultral element to the stack before the first iteration (0 for addition, 1 for multiplication)
                    match outer {
                        Some(Operation::Computation(Computation::Add)) => {
                            commands.push(Command::Put(StackValue::Int(0)));
                        }
                        Some(Operation::Computation(Computation::Mul)) => {
                            commands.push(Command::Put(StackValue::Int(1)));
                        }
                        _ => (),
                    }
                    let label = index + commands.len();
                    // for loop
                    match outer {
                        Some(Operation::Comparison(_)) => {
                            todo!("vector unpacking inside comparison not supported")
                        }
                        Some(Operation::Computation(computation)) => match computation {
                            Computation::Add | Computation::Sub => {
                                // get value at idx and add it to the current value on the stack
                                commands.push(Command::Load(vec_ptr));
                                commands.push(Command::Load(idx));
                                commands.push(Command::Get);
                                commands.push(Command::Add);
                            }
                            Computation::Mul | Computation::Div => {
                                // get value at idx and multiply it to the current value on the stack
                                commands.push(Command::Load(vec_ptr));
                                commands.push(Command::Load(idx));
                                commands.push(Command::Get);
                                commands.push(Command::Mul);
                            }
                            _ => panic!("Unsupported computation operation for vector unpacking!"),
                        },
                        Some(Operation::Set) => {
                            panic!("Cannot use vector unpacking inside assignment!")
                        }
                        Some(Operation::Logic(logic_op)) => {
                            commands.push(Command::Load(vec_ptr));
                            commands.push(Command::Load(idx));
                            commands.push(Command::Get);
                            commands.push(operation_to_command(Operation::Logic(logic_op)));
                        }
                        _ => panic!("Unsupported outer operation for vector unpacking!"),
                    };
                    // exit condition: idx + 1 < len(vector) (next iteration will try to access idx + 1, so we need to check if it's out of bounds)
                    commands.push(Command::Load(idx));
                    commands.push(Command::Put(StackValue::Int(1)));
                    commands.push(Command::Add);
                    commands.push(Command::Dup);
                    commands.push(Command::Store(idx));
                    commands.push(Command::Load(vec_ptr));
                    commands.push(Command::Len);
                    commands.push(Command::Ls);
                    commands.push(Command::Jmp(label));
                }
            }
            VectorOp::Pack => {
                commands.push(Command::New);
                for node in iteration.left.iter().chain(iteration.right.iter()) {
                    match node {
                        Value::Expression(expr) => {
                            commands.append(&mut ir_expression(
                                expr,
                                variables,
                                index + commands.len(),
                            ));
                        }
                        Value::Number(_) | Value::Name(_) => {
                            commands.append(&mut ir_value(
                                node,
                                variables,
                                index + commands.len(),
                                None,
                            ));
                        }
                        _ => panic!("Non-expression node found inside iteration!"),
                    }
                    commands.push(Command::Push);
                }
            }
        },
        Some(_) => panic!("Non-vector operation found inside iteration!"),
        None => todo!(),
    }
    commands
}
