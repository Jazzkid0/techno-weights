// Solve the following puzzle:
// You have 12 masses and a balance scale. 11 of the masses are the same weight, but one is different.
// You can only use the balance scale 3 times. How do you find the different weight?
//
// This tool lets you try out a method to solve the puzzle.
// It won't tell you whether you have found the consistent method though.

use core::panic;

use rand::Rng;

#[derive(PartialEq, Clone, Debug)]
enum MassWeight {
    Different,
    Same,
}

#[derive(Clone, Debug)]
struct Mass {
    name: char,
    weight: MassWeight,
}

#[derive(PartialEq, Debug)]
enum Balance {
    Balanced,
    LeftHeavy,
    RightHeavy,
}

fn get_input() -> String {
    let mut input = String::new();
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

fn select_masses(masses: &Vec<Mass>, selection: String) -> Vec<Mass> {
    let mut masses_out = Vec::new();
    for c in selection.to_uppercase().chars() {
        if c.is_alphabetic() {
            if let Some(mass) = masses.iter().find(|m| m.name == c) {
                masses_out.push(mass.clone());
            }
        }
    }
    masses_out
}

fn get_mass_names(masses: &Vec<Mass>) -> String {
    let mut names = String::new();
    for mass in masses {
        names.push(mass.name);
    }
    names
}

fn weigh(left: &Vec<Mass>, right: &Vec<Mass>) -> Balance {
    let left_weight: i32 = left
        .iter()
        .map(|m| match m.weight {
            MassWeight::Different => 1,
            MassWeight::Same => 0,
        })
        .sum();
    let right_weight: i32 = right
        .iter()
        .map(|m| match m.weight {
            MassWeight::Different => 1,
            MassWeight::Same => 0,
        })
        .sum();
    if left_weight > right_weight {
        Balance::LeftHeavy
    } else if left_weight < right_weight {
        Balance::RightHeavy
    } else {
        Balance::Balanced
    }
}

fn get_answer(masses: &Vec<Mass>) -> String {
    let mut answer = String::new();
    for mass in masses {
        if mass.weight == MassWeight::Different {
            answer.push(mass.name);
        }
    }
    answer
}

fn guess(masses: &Vec<Mass>) -> bool {
    let mut input = String::new();
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_uppercase() == get_answer(masses)
}

fn setup_masses() -> Vec<Mass> {
    let mut masses = Vec::new();
    for c in 'A'..='L' {
        masses.push(Mass {
            name: c,
            weight: MassWeight::Same,
        });
    }
    let index_of_different = rand::thread_rng().gen_range(0..12);
    masses[index_of_different as usize].weight = MassWeight::Different;
    masses
}

#[derive(Debug)]
enum GameResult {
    Win,
    Lose,
}

fn manual_solve() -> GameResult {
    let masses = setup_masses();
    let mut measurements = 3;

    while measurements > 0 {
        println!("\n-------------------\n");
        println!("Measurements left: {}", measurements);

        println!("Which masses would you like to put on the left side of the scale?");
        let left_selection = get_input();
        let left_side = select_masses(&masses, left_selection);
        println!("Left side: {:?}", get_mass_names(&left_side));

        println!("Which masses would you like to put on the right side of the scale?");
        let right_selection = get_input();
        let right_side = select_masses(&masses, right_selection);
        println!("Right side: {:?}", get_mass_names(&right_side));

        let balance = weigh(&left_side, &right_side);

        println!("The balance is: {:?}", balance);
        measurements -= 1;
    }

    println!("\n-------------------\n");

    println!("You have no more measurements left.");
    println!("What do you think the different mass is?");

    if guess(&masses) {
        println!("You found the different mass!");
        println!("The different mass was: {}", get_answer(&masses));
        return GameResult::Win;
    } else {
        println!("You didn't find the different mass.");
        println!("The different mass was: {}", get_answer(&masses));
        return GameResult::Lose;
    }
}

fn ez_measure(masses: &Vec<Mass>, left: String, right: String, verbose: bool) -> Balance {
    let left_side = select_masses(&masses, left);
    let right_side = select_masses(&masses, right);
    let balance = weigh(&left_side, &right_side);

    if verbose {
        println!("Left side: {:?}", get_mass_names(&left_side));
        println!("Right side: {:?}", get_mass_names(&right_side));
        println!("The balance is: {:?}", balance);
        println!("\n-------------------\n");
    }
    balance
}

enum ResultComparison {
    Same,
    Opposite,
    Balanced,
}

fn compare_results(first: &Balance, second: &Balance) -> ResultComparison {
    if *second == Balance::Balanced {
        return ResultComparison::Balanced;
    } else if first == second {
        return ResultComparison::Same;
    } else {
        return ResultComparison::Opposite;
    }
}

fn auto_solve(verbose: bool) -> GameResult {
    let masses = setup_masses();

    let answer = get_answer(&masses);
    let final_result: char;

    let result_1 = ez_measure(&masses, "ABCD".to_string(), "EFGH".to_string(), verbose);

    match result_1 {
        Balance::Balanced => {
            let result_2 = ez_measure(&masses, "IJ".to_string(), "KA".to_string(), verbose);
            match result_2 {
                Balance::Balanced => final_result = 'L',
                _ => {
                    let result_3 = ez_measure(&masses, "JK".to_string(), "AB".to_string(), verbose);
                    let comparison = compare_results(&result_2, &result_3);
                    match comparison {
                        ResultComparison::Balanced => final_result = 'I',
                        ResultComparison::Same => final_result = 'J',
                        ResultComparison::Opposite => final_result = 'K',
                    }
                }
            }
        }
        _ => {
            let result_2 = ez_measure(&masses, "ABE".to_string(), "CDF".to_string(), verbose);
            match result_2 {
                Balance::Balanced => {
                    let result_3 = ez_measure(&masses, "G".to_string(), "I".to_string(), verbose);
                    match result_3 {
                        Balance::Balanced => final_result = 'H',
                        _ => final_result = 'G',
                    }
                }
                _ => {
                    let result_3 = ez_measure(&masses, "ED".to_string(), "FB".to_string(), verbose);
                    let comparisons = (
                        compare_results(&result_1, &result_2),
                        compare_results(&result_2, &result_3),
                    );
                    match comparisons {
                        (ResultComparison::Same, ResultComparison::Balanced) => final_result = 'A',
                        (ResultComparison::Same, ResultComparison::Same) => final_result = 'F',
                        (ResultComparison::Same, ResultComparison::Opposite) => final_result = 'B',
                        (ResultComparison::Opposite, ResultComparison::Balanced) => {
                            final_result = 'C'
                        }
                        (ResultComparison::Opposite, ResultComparison::Same) => final_result = 'E',
                        (ResultComparison::Opposite, ResultComparison::Opposite) => {
                            final_result = 'D'
                        }
                        _ => panic!("This should never happen!"),
                    }
                }
            }
        }
    }
    println!("Auto-solve result: {}", final_result);
    println!("The different mass was: {}", answer);

    if final_result == answer.chars().next().unwrap() {
        return GameResult::Win;
    } else {
        return GameResult::Lose;
    }
}

enum SolveMethod {
    Manual,
    Auto,
}

fn solve(method: SolveMethod, verbose: bool) -> GameResult {
    match method {
        SolveMethod::Manual => manual_solve(),
        SolveMethod::Auto => auto_solve(verbose),
    }
}

fn main() {
    let mut startover = true;
    while startover {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("12 Masses Puzzle");
        println!("-------------------\n");
        println!("Would you like to solve the puzzle manually or automatically?");
        println!("Type 'manual' or 'auto' and press Enter. (m or a works)");
        let method = get_input().trim().to_lowercase();
        if method.starts_with('m') {
            solve(SolveMethod::Manual, true);
        } else if method.starts_with('a') {
            let mut record: Vec<GameResult> = Vec::new();

            println!("How many times should the computer solve the puzzle?");
            let attempts = get_input().trim().parse::<i32>().unwrap();

            println!("Would you like to see the steps? (y/n)");
            let verbose = get_input().trim().to_lowercase();
            if verbose.starts_with('y') {
                for _ in 0..attempts {
                    record.push(solve(SolveMethod::Auto, true));
                }
            } else {
                for _ in 0..attempts {
                    record.push(solve(SolveMethod::Auto, false));
                }
            }
            println!("Results: {:?}", record);
        } else {
            println!("Invalid input.");
        }

        println!("\n\nWould you like to start over? (y/anything else)");
        let startover_input = get_input().trim().to_lowercase();
        if !startover_input.starts_with('y') {
            startover = false;
        }
    }
}
