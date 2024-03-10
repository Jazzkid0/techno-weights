// Solve the following puzzle:
// You have 12 masses and a balance scale. 11 of the masses are the same weight, but one is different.
// You can only use the balance scale 3 times. How do you find the different weight?
//
// This tool lets you try out a method to solve the puzzle.
// It won't tell you whether you have found the consistent method though.

use rand::Rng;

#[derive(PartialEq, Clone)]
enum MassWeight {
    Different,
    Same,
}

#[derive(PartialEq, Clone, Ord, Eq, PartialOrd)]
enum MassInfo {
    Same,
    None,
}

#[derive(Clone)]
struct Mass {
    weight: MassWeight,
    info: MassInfo,
}

#[derive(PartialEq)]
enum Balance {
    Balanced,
    NotBalanced,
}

struct MassGroup<'a> {
    masses: Vec<&'a mut Mass>,
    balanced: Balance,
}

impl MassGroup<'_> {
    fn from_masses(masses: Vec<&mut Mass>) -> MassGroup {
        let mut balanced = Balance::Balanced;
        for mass in masses.as_slice().iter() {
            if mass.weight == MassWeight::Different {
                balanced = Balance::NotBalanced;
            }
        }
        MassGroup { masses, balanced }
    }
}

fn main() {
    let mut masses = Vec::new();
    let mut confirmed_masses = Vec::new();
    for _ in 0..=12 {
        masses.push(Mass {
            weight: MassWeight::Same,
            info: MassInfo::None,
        });
    }
    let index_of_different = rand::thread_rng().gen_range(0..=12);
    masses[index_of_different as usize].weight = MassWeight::Different;

    let mut guesses = 3;
    let mut different_mass_found = false;

    while guesses > 0 && !different_mass_found {
        println!("\n-------------------\n");
        let confirmed = confirmed_masses.len();
        println!("Guesses left: {}\n", guesses);
        println!("Masses identified as not different: {}", confirmed);

        let mut input = String::new();
        let mut group_size = 13;
        while group_size < 1 || group_size > 12 || group_size % 2 != 0 {
            println!("How many masses do you want to weigh?");
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            group_size = input.trim().parse().unwrap();
        }

        let mut of_which_confirmed = 12;
        while of_which_confirmed > confirmed {
            println!("How many of the masses do you want to be from those you have previously confirmed as not different?");
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            of_which_confirmed = input.trim().parse().unwrap();
        }

        // sort the masses vector so that masses with mass.info == MassInfo::Same are at the beginning
        masses.sort_by(|a, b| a.info.cmp(&b.info));

        let mut masses_to_weigh = Vec::new();
        let mut masses_left_out = Vec::new();
        let mut masses_to_add = group_size.clone();
        let mut confirmed_to_add = of_which_confirmed.clone();
        for mass in masses.iter_mut() {
            match mass.info {
                MassInfo::Same => {
                    if confirmed_to_add > 0 && masses_to_add > 0 {
                        masses_to_weigh.push(mass);
                        confirmed_to_add -= 1;
                        masses_to_add -= 1;
                    } else {
                        masses_left_out.push(mass);
                    }
                }
                MassInfo::None => {
                    if masses_to_add > 0 {
                        masses_to_weigh.push(mass);
                        masses_to_add -= 1;
                    } else {
                        masses_left_out.push(mass);
                    }
                }
            }
        }

        let mut group = MassGroup::from_masses(masses_to_weigh);

        match group.balanced {
            Balance::Balanced => {
                println!("The masses are balanced");
                for mass in group.masses.iter_mut() {
                    if mass.info != MassInfo::Same {
                        mass.info = MassInfo::Same;
                        confirmed_masses.push(mass.clone());
                    }
                }
            }
            Balance::NotBalanced => {
                println!("The masses are not balanced");
                for mass in masses_left_out {
                    if mass.info != MassInfo::Same {
                        mass.info = MassInfo::Same;
                        confirmed_masses.push(mass.clone());
                    }
                }
                if group_size == 2 && of_which_confirmed == 1 {
                    if confirmed_masses.len() < 11 {
                        println!("You got lucky! Not all masses were ruled out.");
                    }
                    different_mass_found = true;
                }
            }
        }

        if confirmed_masses.len() == 11 {
            different_mass_found = true;
        }

        guesses -= 1;
    }

    println!("\n-------------------\n");

    println!(
        "You successfully identified {} of the 12 masses as not different.",
        confirmed_masses.len()
    );

    if guesses > 0 {
        println!("You had {} guesses left over.", guesses);
    }

    if different_mass_found {
        println!("You found the different mass!");
    } else {
        println!("You did not find the different mass.");
    }

    // wait for any key to exit
    println!("\nPress Enter to exit.");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
