use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn calc_fuel_req() -> i32 {
    println!("hello");

    let file = File::open("input/one.txt").expect("failed to read file");
    let reader = BufReader::new(file);

    let mut lines = vec![];
    for line in reader.lines() {
        lines.push(line.expect("failed to read line"))
    }

    println!("{:?}", lines);

    let fuel:i32 = lines
        .iter()
        .map(|line|line.parse::<i32>().expect("failed to convert to int"))
        .map(|mass|get_total_fuel_from_mass(mass))
        .sum();

//    let fuel = get_total_fuel_from_mass(fuel.clone());

    println!("{:?}", fuel);
    fuel
}

fn get_total_fuel_from_mass(fuel: i32) -> i32 {
//    let init = fuel.clone();
    let mut last_fuel = fuel;
    let mut fuel = 0;
    loop {
        let fuel_for_fuel = calc_module_fuel(last_fuel);
        println!("fuel: {}, last fuel: {}, fuel for that fuel: {}", fuel, last_fuel, fuel_for_fuel);
        if fuel_for_fuel < 1 {
            break;
        }
        fuel += fuel_for_fuel;
        last_fuel = fuel_for_fuel;
    }
    fuel
}

fn calc_module_fuel(mass: i32) -> i32 {
    (mass/3) -2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(get_total_fuel_from_mass(14), 2);
        assert_eq!(get_total_fuel_from_mass(1969), 966);
        assert_eq!(get_total_fuel_from_mass(100756), 50346);
        assert_eq!(calc_fuel_req(), 5183653);
    }

}
