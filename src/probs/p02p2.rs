use crate::utils;

fn add(x: usize, y: usize) -> usize {
    x + y
}
fn multiplies(x: usize, y: usize) -> usize {
    x * y
}

fn calc<F>(vec: &mut [usize], opcode_position: usize, operation: F)
where
    F: Fn(usize, usize) -> usize,
{
    let pos_x = vec[opcode_position + 1];
    let pos_y = vec[opcode_position + 2];
    let pos_result = vec[opcode_position + 3];
    vec[pos_result] = operation(vec[pos_x], vec[pos_y]);
}

pub fn execute_program(vec: &mut [usize]) {
    let mut pos = 0;
    while vec[pos] != 99 {
        match vec[pos] {
            1 => calc(vec, pos, add),
            2 => calc(vec, pos, multiplies),
            99 => break,
            _ => panic!("Should never happen!"),
        }

        pos += 4;
    }
}

pub fn resolve() -> usize {
    let vec = utils::split_char_file("./data/02.txt", ',');
    let mut vec = utils::convert_strings_to_usize(vec);

    execute_program(&mut vec);
    vec[0]
}

#[test]
fn test() {
    let map: Vec<(Vec<usize>, &[usize])> = vec![
        (vec![1, 0, 0, 0, 99], &[2, 0, 0, 0, 99]),
        (vec![2, 3, 0, 3, 99], &[2, 3, 0, 6, 99]),
        (vec![2, 4, 4, 5, 99, 0], &[2, 4, 4, 5, 99, 9801]),
        (
            vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
            &[30, 1, 1, 4, 2, 5, 6, 0, 99],
        ),
    ];

    for (mut x, y) in map {
        execute_program(&mut x);
        assert_eq!(x, y);
    }
}
