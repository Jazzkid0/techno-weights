// Solve the following puzzle:
// You have 12 masses and a balance scale. 11 of the masses are the same weight, but one is different.
// You can only use the balance scale 3 times. How do you find the different weight?
//
// This tool lets you try out a method to solve the puzzle.
// It won't tell you whether you have found the consistent method though.

use rand::Rng;

#[derive(PartialEq, Clone, Debug)]
enum MassWeight {
    Different,
    Same,
}

#[derive(PartialEq, Clone, Ord, Eq, PartialOrd, Debug)]
enum MassInfo {
    Same,
    None,
}

#[derive(Clone, Debug)]
struct Mass {
    name: char,
    weight: MassWeight,
    info: MassInfo,
}

#[derive(PartialEq, Debug)]
enum Balance {
    Balanced,
    LeftHeavy,
    RightHeavy,
}

fn select_masses(masses: &Vec<Mass>) -> Vec<Mass> {
    let mut input = String::new();
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();

    let mut masses_out = Vec::new();
    for c in input.to_uppercase().chars() {
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

fn main() {
    let mut masses = Vec::new();
    for c in 'A'..='L' {
        masses.push(Mass {
            name: c,
            weight: MassWeight::Same,
            info: MassInfo::None,
        });
    }
    let index_of_different = rand::thread_rng().gen_range(0..12);
    masses[index_of_different as usize].weight = MassWeight::Different;

    let mut measurements = 3;

    while measurements > 0 {
        println!("\n-------------------\n");
        println!("Measurements left: {}", measurements);

        println!("Which masses would you like to put on the left side of the scale?");
        let left_side = select_masses(&masses);
        println!("Left side: {:?}", get_mass_names(&left_side));

        println!("Which masses would you like to put on the right side of the scale?");
        let right_side = select_masses(&masses);
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
    } else {
        println!("You didn't find the different mass.");
    }

    println!("The different mass was: {}", get_answer(&masses));

    // wait for any key to exit
    println!("\nPress Enter to exit.");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
