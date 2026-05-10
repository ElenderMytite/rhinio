use std::collections::HashMap;
mod expression;
mod iteration;
mod value;
use crate::parser::{AstNode, Comparison, Computation, Logic, Operation};
use crate::vm::StackValue;
#[derive(Debug, Clone)]
pub enum Command {
    //computation
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    // stack operations
    Dup,
    Put(StackValue),
    Cls,
    // comparison
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
    // control flow,
    // Set the instruction pointer to specified value if on the top of the stack is true value,
    Jmp(usize),
    // vector operations,
    // returns the length of the vector on the top of the stack
    Len,
    // creates new vector and pushes it to the stack
    New,
    // pushes the top of the stack to the vector below it on the stack
    Push,
    // pops the vector on the top of the stack and puts the result to the stack
    Pop,
    // pops the index from the stack, copies the vector below it on the stack, pushes the value at the index to the stack
    Get,
}
/* example unpacking ir for :
 *
 */
pub fn ir(
    root: AstNode,
    variables: &mut HashMap<String, usize>,
    /* index of the first command of this function */ index: usize,
) -> Vec<Command> {
    let mut commands = Vec::new();
    match root {
        AstNode::Iteration(iteration) => {
            commands.append(&mut iteration::ir_iteration(
                &iteration,
                variables,
                index + commands.len(),
                None,
            ));
        }
        AstNode::Expression(expression) => {
            commands.append(&mut expression::ir_expression(
                &expression,
                variables,
                index + commands.len(),
            ));
        }
        AstNode::BlockCode(nodes) => {
            for node in nodes {
                commands.append(&mut ir(node, variables, index + commands.len()));
                commands.push(Command::Cls);
            }
        }
    }
    commands
}

fn register_variable(env: &mut HashMap<String, usize>, variable: String) -> usize {
    if !env.contains_key(&variable) {
        env.insert(variable.clone(), env.len());
    }
    env[&variable]
}
fn operation_to_command(op: Operation) -> Command {
    match op {
        Operation::Set => Command::Store(0),
        Operation::Comparison(comparison) => match comparison {
            Comparison::Greater => Command::Gt,
            Comparison::Less => Command::Ls,
            Comparison::Equal => Command::Eq,
            Comparison::GreaterOrEqual => Command::Geq,
            Comparison::LessOrEqual => Command::Leq,
            Comparison::NotEqual => Command::Neq,
        },
        Operation::Computation(computation) => match computation {
            Computation::Add => Command::Add,
            Computation::Sub => Command::Sub,
            Computation::Mul => Command::Mul,
            Computation::Div => Command::Div,
            Computation::Mod => Command::Mod,
        },
        Operation::Logic(logic) => match logic {
            Logic::And => Command::And,
            Logic::Or => Command::Or,
            Logic::Xor => Command::Xor,
            Logic::Nand => Command::Nand,
            Logic::Nor => Command::Nor,
            Logic::Not => Command::Not,
        },

        _ => panic!("operation must be converted manually: {:?}", op),
    }
}
