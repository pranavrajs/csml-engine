pub mod error_format;
pub mod interpreter;
pub mod parser;

use error_format::data::ErrorInfo;
use interpreter::{
    ast_interpreter::interpret_scope,
    csml_rules::check_valid_flow,
    data::Data,
    json_to_rust::{Context, Event},
    message::MessageData,
};
use parser::{ast::*, Parser};
use std::collections::HashMap;

pub fn parse_file(file: String) -> Result<Flow, ErrorInfo> {
    match Parser::parse_flow(file.as_bytes()) {
        Ok(flow) => {
            check_valid_flow(&flow)?;
            Ok(flow)
        }
        Err(e) => Err(e),
    }
}

pub fn is_trigger(flow: &Flow, string: &str) -> bool {
    let info = flow.flow_instructions.get(&InstructionType::StartFlow);

    if let Some(Expr::VecExpr(vec, ..)) = info {
        for elem in vec.iter() {
            match elem {
                Expr::LitExpr(Literal::StringLiteral { value, .. })
                    if value.to_lowercase() == string.to_lowercase() =>
                {
                    return true
                }
                _ => continue,
            }
        }
    }
    false
}

pub fn search_for<'a>(flow: &'a Flow, name: &str) -> Option<&'a Expr> {
    flow.flow_instructions
        .get(&InstructionType::NormalStep(name.to_owned()))
}

pub fn version() -> String {
    "CsmlV2".to_owned()
}

pub fn execute_step(flow: &Flow, name: &str, mut data: Data) -> Result<MessageData, ErrorInfo> {
    match search_for(flow, name) {
        Some(Expr::Block { arg: actions, .. }) => interpret_scope(actions, &mut data),
        _ => Err(ErrorInfo {
            interval: Interval { line: 0, column: 0 },
            message: format!("Error: step {} not found", name),
        }),
    }
}

pub fn interpret(
    ast: &Flow,
    step_name: &str,
    memory: &Context,
    event: &Option<Event>,
) -> MessageData {
    let data = Data {
        ast,
        memory,
        event,
        step_vars: HashMap::new(),
    };

    MessageData::error_to_message(execute_step(ast, step_name, data))
}
