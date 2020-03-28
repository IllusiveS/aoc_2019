pub mod common;

use common::*;

#[test]
fn basic_test_01(){
    let mut masses = read_masses_from_file("01_easy_input.txt".to_string());
    let result = masses.iter().map(|mass| mass / 3 - 2).fold(0, |acc, mass| acc + mass);
    assert_eq!(result, 3161483);
}

#[test]
fn advanced_test_01() {
    let masses = read_masses_from_file("01_easy_input.txt".to_string());
    let mut req_fuel = 0;
    let mut current_masses = masses;
    loop {
        current_masses = current_masses.iter().map(|mass| mass / 3 - 2).filter(|&mass| mass > 0).collect();
        let calculated_fuel = current_masses.clone().iter().fold(0, |acc, &mass| acc + mass);

        if calculated_fuel <= 0 {
            break;
        }

        req_fuel += calculated_fuel;
    }
    assert_eq!(req_fuel, 4739374);
}