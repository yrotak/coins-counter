use std::fs::OpenOptions;
use std::io::prelude::*;
fn main() {
    const VALUES: [i32; 8] = [1, 2, 5, 10, 20, 50, 100, 200];
    const BANK_STACK_SIZE: [i32; 8] = [50, 50, 50, 40, 40, 40, 25, 25];

    let cents: Vec<i32> = vec![115, 103, 80, 95, 53, 0, 0, 0];

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("results.csv")
        .unwrap();

    if let Err(e) = writeln!(
            file,
            "Pièce,Quantitée,Valeur,Nombre de rouleaux,Valeur des rouleaux,Quantitée non conditionnable en rouleaux,Quantitée pour faire un autre rouleaux",
        ) {
            eprintln!("Couldn't write to file: {}", e);
        }

    let mut total_price: f64 = 0.0;
    let mut total_stacks: i32 = 0;
    let mut total_stacks_price: f64 = 0.0;

    for (i, n) in cents.iter().enumerate() {
        let stacks = num_integer::div_rem::<i32>(*n, BANK_STACK_SIZE[i]);

        let value: f64 = (VALUES[i] * n) as f64;
        let stacks_value: f64 = (stacks.0 * BANK_STACK_SIZE[i] * VALUES[i]) as f64;

        total_price += value;
        total_stacks += stacks.0;
        total_stacks_price += stacks_value;

        if let Err(e) = writeln!(
            file,
            "{} €,{},{} €,{},{} €,{},{}",
            (VALUES[i] as f64) / 100.0,
            *n,
            value / 100.0,
            stacks.0,
            stacks_value / 100.0,
            stacks.1,
            BANK_STACK_SIZE[i] - stacks.1
        ) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
    if let Err(e) = writeln!(
        file,
        "Total,{},{} €,{},{} €,,",
        cents.iter().sum::<i32>(),
        total_price / 100.0,
        total_stacks,
        total_stacks_price / 100.0,
    ) {
        eprintln!("Couldn't write to file: {}", e);
    }
}
