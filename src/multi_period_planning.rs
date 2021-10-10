use std::error::Error;

use good_lp::{constraint, default_solver, variables, Solution, SolverModel};

fn main() -> Result<(), Box<dyn Error>> {
    variables! {
        vars:
            0 <= xi1;
            0 <= xii1;
            0 <= xi2;
            0 <= xii2;
            0 <= xi3;
            0 <= xii3;
            0 <= yi1;
            0 <= yii1;
            0 <= yi2;
            0 <= yii2;
    }
    let solution = vars
        .minimise(
            75 * xi1
                + 50 * xii1
                + 8 * yi1
                + 7 * yii1
                + 75 * xi2
                + 50 * xii2
                + 8 * yi2
                + 7 * yii2
                + 75 * xi3
                + 50 * xii3,
        )
        .using(default_solver)
        .with(constraint!(2 * xi1 + 7 * xii1 <= 920))
        .with(constraint!(5 * xi1 + 3 * xii1 <= 790))
        .with(constraint!(2 * xi2 + 7 * xii2 <= 750))
        .with(constraint!(5 * xi2 + 3 * xii2 <= 600))
        .with(constraint!(2 * xi3 + 7 * xii3 <= 500))
        .with(constraint!(5 * xi3 + 3 * xii3 <= 480))
        .with(constraint!(xi1 - yi1 == 30))
        .with(constraint!(xii1 - yii1 == 20))
        .with(constraint!(xi2 + yi1 - yi2 == 60))
        .with(constraint!(xii2 + yii1 - yii2 == 50))
        .with(constraint!(xi3 + yi2 == 80))
        .with(constraint!(xii3 + yii2 == 90))
        .solve()?;

    println!("製品Iの1ヶ月目の生産量: {}", solution.value(xi1));
    println!("製品IIの1ヶ月目の生産量: {}", solution.value(xii1));
    println!("製品Iの2ヶ月目の生産量: {}", solution.value(xi2));
    println!("製品IIの2ヶ月目の生産量: {}", solution.value(xii2));
    println!("製品Iの3ヶ月目の生産量: {}", solution.value(xii3));
    println!("製品IIの3ヶ月目の生産量: {}", solution.value(xii3));
    println!("製品Iの1ヶ月目の在庫量: {}", solution.value(yi1));
    println!("製品IIの1ヶ月目の在庫量: {}", solution.value(yii1));
    println!("製品Iの2ヶ月目の在庫量: {}", solution.value(yi2));
    println!("製品IIの2ヶ月目の在庫量: {}", solution.value(yii2));
    Ok(())
}
