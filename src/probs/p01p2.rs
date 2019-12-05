use crate::utils;

fn calc_fuel(mass: u32) -> u32 {
    let mut fuel = 0;

    if (mass as i32 / 3 - 2) > 0 {
        fuel = mass / 3 - 2;
        fuel += calc_fuel(fuel)
    }

    fuel
}

fn total_fuel(modules: &[u32]) -> u32 {
    modules
        .iter()
        .map(|module| calc_fuel(*module))
        .fold(0u32, |acc, fuel| acc + fuel)
}

fn load_modules(filename: &str) -> std::io::Result<Vec<u32>> {
    let numbers: Vec<u32> = utils::split_whitespace_file(filename)
        .iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    Ok(numbers)
}

pub fn resolve() -> u32 {
    let modules = load_modules("./data/01.txt").unwrap();

    total_fuel(&modules)
}
