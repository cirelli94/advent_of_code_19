use super::intcode_computer as ic;
use crate::utils;

const EXPECTED_OUTPUT: usize = 19_690_720;
pub fn resolve() -> usize {
    let vec = utils::split_char_file("./data/02.txt", ',');
    let memory = utils::convert_strings_to_usize(vec);

    for i in 0..99 {
        for j in 0..99 {
            let mut mem = memory.clone();
            mem[1] = i;
            mem[2] = j;
            let mut computer = ic::IntcodeComputer::new(mem);
            let result = computer.execute_program();

            if let Some(result) = result {
                if result == EXPECTED_OUTPUT {
                    return 100 * i + j;
                }
            }
        }
    }

    panic!();
}
