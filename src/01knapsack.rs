use std::error::Error;

use good_lp::{constraint, default_solver, variables, Solution, SolverModel};

fn main() -> Result<(), Box<dyn Error>> {
    let yen: Vec<i32> = vec![100, 130, 70, 40, 90, 30, 20, 20];
    let weight: Vec<i32> = vec![80, 70, 60, 50, 40, 30, 20, 10];
    variables!(vars:x[yen.len()] (binary));
    let solution = vars
        .maximise(
            x[0] * yen[0]
                + x[1] * yen[1]
                + x[2] * yen[2]
                + x[3] * yen[3]
                + x[4] * yen[4]
                + x[5] * yen[5]
                + x[6] * yen[6]
                + x[7] * yen[7],
        )
        .using(default_solver)
        .with(constraint!(
            x[0] * weight[0]
                + x[1] * weight[1]
                + x[2] * weight[2]
                + x[3] * weight[3]
                + x[4] * weight[4]
                + x[5] * weight[5]
                + x[6] * weight[6]
                + x[7] * weight[7]
                <= 100
        ))
        .solve()?;
    
    for i in 0..yen.len() {
        print!("x{} : {}\n", i, solution.value(x[i]));
    }
    println!(
        "Total yen: {}",
        solution.eval(
            solution.value(x[0]) as i32 * yen[0]
                + solution.value(x[1]) as i32 * yen[1]
                + solution.value(x[2]) as i32 * yen[2]
                + solution.value(x[3]) as i32 * yen[3]
                + solution.value(x[4]) as i32 * yen[4]
                + solution.value(x[5]) as i32 * yen[5]
                + solution.value(x[6]) as i32 * yen[6]
                + solution.value(x[7]) as i32 * yen[7]
        )
    );
    Ok(())
}
