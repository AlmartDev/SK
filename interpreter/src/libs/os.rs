use crate::evaluator::env::Environment;
use crate::evaluator::eval::Evaluator;
use crate::parser::lexer::TokenSpan;
use crate::core::error::Error;
use crate::core::value::Value;

use std::env::consts::OS;
use std::process::Command;

pub fn register(env: &mut Environment) {
    env.define("name".into(), Value::NativeFn(name));
    env.define("command".into(), Value::NativeFn(command));
    env.define("clear".into(), Value::NativeFn(clear));
}

fn err(token: TokenSpan, msg: String) -> Error {
    Error {
        token,
        message: msg
    }
}

pub fn name(_args: Vec<Value>, _span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    Ok(Value::String(OS.to_string()))
}

pub fn command(args: Vec<Value>, span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    if args.is_empty() {
        return Err(err(span, "command() requires at least one argument".to_string()));
    }

    let cmd_str = match &args[0] {
        Value::String(s) => s,
        _ => return Err(err(span, "command() expects a string argument".to_string())),
    };

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", cmd_str])
            .output()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(cmd_str)
            .output()
    };

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            
            if output.status.success() {
                Ok(Value::String(stdout))
            } else {
                Err(err(span, format!("Command failed: {}", stderr)))
            }
        }
        Err(e) => Err(err(span, format!("Failed to execute command: {}", e))),
    }
}

pub fn clear(_args: Vec<Value>, span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    let status = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "cls"]).status()
    } else {
        Command::new("clear").status()
    };

    match status {
        Ok(status) => {
            if status.success() {
                Ok(Value::None)
            } else {
                Err(err(span, "Failed to clear the console".to_string()))
            }
        }
        Err(e) => Err(err(span, e.to_string())),
    }
}