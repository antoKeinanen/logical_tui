use std::{iter::zip, collections::HashMap};

use dialoguer::Input;
use logical_solver::{permutate, solve_truth_table, parse_expression, ast::Expr};
use spinner::SpinnerBuilder;

fn print_table(vars: Vec<String>, states: Vec<HashMap<String, bool>>, results: Vec<bool>, expr: Expr) {
    
    vars.iter().for_each(|var| print!("{}\t|", var));
    print!("{:?}\n", expr);

    for (state, result) in zip(states, results) {
        state.values().for_each(|state| print!("{}\t|", state));
        print!("{}\n", result);
    }
}

pub fn solve_truth_table_loop() {
    loop {
        let vars: String = Input::new()
            .with_prompt("Enter variables")
            .interact_text()
            .unwrap();

        let vars:Vec<String> = vars.split(", ").map(|var| var.to_string()).collect();

        let expr: String = Input::new()
            .with_prompt("Enter expression")
            .interact_text()
            .unwrap();


        let sp = SpinnerBuilder::new("Generating permutations...".into()).start();
        let states = permutate(vars.clone());

        sp.message("Parsing expression...".into());
        let expr = parse_expression(expr.as_str());

        match expr {
            Ok(expr) => {
                sp.message("Evaluating all permutations...".into());
                let results = solve_truth_table(expr.clone(), states.clone());
                sp.close();
                
                print_table(vars, states, results, *expr);
            },
            Err(err) => {
                sp.close();
                println!("{}", err);
            },
        }

    }
}
