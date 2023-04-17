use std::collections::HashMap;

use dialoguer::Input;
use logical_solver::{parse_expression, evaluate};
use spinner::SpinnerBuilder;

pub fn evaluate_loop() {
    loop {
        let input: String = Input::new()
           .with_prompt("Enter the expression to evaluate")
           .interact_text().unwrap();
        
        let sp = SpinnerBuilder::new("Parsing expression...".into()).start();
        let expr = parse_expression(input.as_str());
        match expr {
            Ok(expr) => {
                let state = HashMap::new();
                sp.message("Evaluating expression...".into());
                let result = evaluate(expr, state);
                sp.close();
                println!("-> {}", result);
            },
            Err(err) => {
                sp.close();
                println!("\n{}", err);
            },
        }
    }
}
