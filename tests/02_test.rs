pub mod common;

use common::*;

pub struct Computer {
    opcodes : Vec<i32>
}

impl Computer {
    pub fn set_opcode(&mut self, index : usize, new_value : i32) {
        self.opcodes[index] = new_value;
    }

    pub fn process(&mut self) {
        let mut curr_index = 0;

        loop {
            let op_value = self.opcodes[curr_index];
            match op_value {
                1 => self.add(curr_index),
                2 => self.multiply(curr_index),
                99=> break,
                _ => ()
            }
            curr_index += 4;
        }
    }

    fn add(&mut self, starting_index : usize) {
        let first_index = self.opcodes[starting_index + 1] as usize;
        let first_val = self.opcodes[first_index];

        let second_index = self.opcodes[starting_index + 2] as usize;
        let second_val = self.opcodes[second_index];

        let target_index = self.opcodes[starting_index + 3] as usize;

        self.opcodes[target_index as usize] = first_val + second_val;
    }

    fn multiply(&mut self, starting_index : usize) {
        let first_index = self.opcodes[starting_index + 1] as usize;
        let first_val = self.opcodes[first_index];

        let second_index = self.opcodes[starting_index + 2] as usize;
        let second_val = self.opcodes[second_index];

        let target_index = self.opcodes[starting_index + 3] as usize;

        self.opcodes[target_index as usize] = first_val * second_val;
    }
}

#[test]
fn basic_test_02() {
    let opcodes = read_02_input();
    let mut computer : Computer = Computer{ opcodes };

    computer.set_opcode(1, 12);
    computer.set_opcode(2, 2);

    computer.process();

    assert_eq!(computer.opcodes[0], 3765464);
}

#[test]
fn advanced_test_02() {
    let opcodes = read_02_input();

    for noun in 0..99 {
        for verb in 0..99 {
            let mut copy = opcodes.clone();
            let mut computer : Computer = Computer{ opcodes : copy };
            computer.set_opcode(1, noun);
            computer.set_opcode(2, verb);

            computer.process();
            if computer.opcodes[0] == 19690720 {
                println!("{}", noun * 100 + verb);
            }
        }
    }
}