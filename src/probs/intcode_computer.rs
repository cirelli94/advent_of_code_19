#[derive(Default)]
pub struct IntcodeComputer {
    pub memory: Vec<usize>,
    instruction_pointer: usize,
}

impl IntcodeComputer {
    pub fn new(mem: Vec<usize>) -> IntcodeComputer {
        IntcodeComputer {
            memory: mem,
            instruction_pointer: 0,
        }
    }

    pub fn execute_program(&mut self) -> Option<usize> {
        while self.memory[self.instruction_pointer] != 99 {
            match self.memory[self.instruction_pointer] {
                1 => self.do_operation(|x: usize, y: usize| x + y),
                2 => self.do_operation(|x: usize, y: usize| x * y),
                99 => break,
                _ => return None,
            }

            self.instruction_pointer += 4;
        }
        Some(self.memory[0])
    }
    fn do_operation<F>(&mut self, operation: F)
    where
        F: Fn(usize, usize) -> usize,
    {
        let pos_x = self.memory[self.instruction_pointer + 1];
        let pos_y = self.memory[self.instruction_pointer + 2];
        let pos_result = self.memory[self.instruction_pointer + 3];
        self.memory[pos_result] = operation(self.memory[pos_x], self.memory[pos_y]);
    }
}
