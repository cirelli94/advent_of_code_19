use super::intcode_computer as ic;
use crate::utils;

const EXPECTED_OUTPUT: usize = 19_690_720;
pub fn resolve() -> (usize, usize) {
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
                    return (i, j);
                }
            }
        }
    }

    panic!();
}

#[test]
fn test() {
    //let map: Vec<(Vec<usize>, &[usize])> = vec![
    //    (vec![1, 0, 0, 0, 99], &[2, 0, 0, 0, 99]),
    //    (vec![2, 3, 0, 3, 99], &[2, 3, 0, 6, 99]),
    //    (vec![2, 4, 4, 5, 99, 0], &[2, 4, 4, 5, 99, 9801]),
    //    (
    //        vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
    //        &[30, 1, 1, 4, 2, 5, 6, 0, 99],
    //    ),
    //];

    //for (mut x, y) in map {
    //    execute_program(&mut x);
    //    assert_eq!(x, y);
    //}
}
