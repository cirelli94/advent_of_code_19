use super::utils;

fn module_fuel(mass: u32) -> u32 {
    mass / 3 - 2
}

fn total_fuel(modules: &[u32]) -> u32 {
    modules
        .iter()
        .map(|module| module_fuel(*module))
        .fold(0u32, |acc, fuel| acc + fuel)
}

fn load_modules(filename: &str) -> std::io::Result<Vec<u32>> {
    let numbers: Vec<u32> = utils::split_whitespace_file(filename)
        .iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    Ok(numbers)
}

pub fn resolve_problem() -> u32 {
    let modules = load_modules("./data/01.txt").unwrap();
    let fuel = total_fuel(&modules);

    fuel
}
