use dialoguer::{Input, Select, theme::ColorfulTheme, console::Term};
use evaluate::evaluate_loop;
use truth_table_solver::solve_truth_table_loop;

mod evaluate;
mod truth_table_solver;

fn main() {
    let mode = Select::with_theme(&ColorfulTheme::default())
        .items(&vec!["Evaluator", "Truth Table Solver"])
        .default(0)
        .interact_on_opt(&Term::stderr())
        .unwrap();

    if let Some(mode) = mode {
        match mode {
            0 => evaluate_loop(),
            1 => solve_truth_table_loop(),
            _ => panic!("Invalid mode selected!"),
        }
    }
}
