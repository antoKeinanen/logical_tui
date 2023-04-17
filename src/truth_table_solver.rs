use dialoguer::Input;
use logical_solver::{permutate, solve_truth_table, parse_expression};
use spinner::SpinnerBuilder;

pub fn solve_truth_table_loop() {
    loop {
        let vars: String = Input::new()
            .with_prompt("Enter variables")
            .interact_text()
            .unwrap();

        let vars = vars.split(", ").map(|var| var.to_string()).collect();

        let expr: String = Input::new()
            .with_prompt("Enter expression")
            .interact_text()
            .unwrap();


        let sp = SpinnerBuilder::new("Generating permutations...".into()).start();
        let states = permutate(vars);

        sp.message("Parsing expression...".into());
        let expr = parse_expression(expr.as_str());

        match expr {
            Ok(expr) => {
                sp.message("Evaluating all permutations...".into());
                let results = solve_truth_table(expr, states);
                sp.close();

                println!("-> {:?}", results);
            },
            Err(err) => {
                sp.close();
                println!("{}", err);
            },
        }

    }
}
