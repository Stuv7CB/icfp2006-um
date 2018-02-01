use plate::{Plate};
use operator::{Operator};
use std::io::{Read, Write};
use std;

#[derive(Debug)]
pub struct Machine {
    registers: [Plate;8],
    memory: Vec<Vec<Plate>>,
    pointer_finger : u32,
    free_addr: Vec<u32>,
}

impl Machine {
    pub fn new(program: Vec<Plate>) -> Machine {
        let registers = [
            Plate::from(0);8];
        Machine {
            registers,
            memory: vec![program],
            pointer_finger: 0,
            free_addr: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            let operand = self.memory[0]
                .get(self.pointer_finger as usize)
                .unwrap_or_else(||panic!("Pointer finger points out of program"))
                .get_operator_data();
            match operand {
                Operator::Halt => break,
                Operator::Orthography(ort) => self.reg_set(ort.a, Plate::from(ort.data)),
                Operator::ConditionalMove(data) => {
                    if *(self.reg_get(data.c)) != 0 {
                        self.reg_mov(data.b, data.a);
                    }
                }
                Operator::ArrayIndex(data) => {
                    let index = self.reg_get(data.b).unwrap();
                    let offset = self.reg_get(data.c).unwrap();
                    let plate = *self.get_data(index, offset);
                    self.reg_set(data.a, plate);
                }
                Operator::ArrayAmendment(data) => {
                    let value = self.registers[data.c as usize];
                    let index = self.registers[data.a as usize].unwrap();
                    let offset = self.registers[data.b as usize].unwrap();
                    self.set_data(index, offset, value);
                }
                Operator::Addition(data) => {
                    self.registers[data.a as usize] = &self.registers[data.b as usize] + &self.registers[data.c as usize];
                }
                Operator::Multiplication(data) => {
                    self.registers[data.a as usize] = &self.registers[data.b as usize] * &self.registers[data.c as usize];
                }
                Operator::Division(data) => {
                    self.registers[data.a as usize] = &self.registers[data.b as usize] / &self.registers[data.c as usize];
                }
                Operator::NotAnd(data) => {
                    self.registers[data.a as usize] = !&(&self.registers[data.b as usize] & &self.registers[data.c as usize]);
                }
                Operator::Allocation(data) => {
                    let index = self.free_addr.pop().unwrap_or_else(||{
                        self.memory.len() as u32
                    });
                    let size = self.reg_get(data.c).unwrap() as usize;
                    self.reg_set(data.b, Plate::from(index));
                    if index == self.memory.len() as u32 {
                        self.memory.push(vec![Plate::from(0);size]);
                    }
                    else {
                        self.memory[index as usize] = vec![Plate::from(0);size];
                    }
                }
                Operator::Abandonment(data) => {
                    let index= *self.reg_get(data.c);
                    self.free_addr.push(index.unwrap());
                }
                Operator::Output(data) => {
                    print!("{}", (self.registers[data.c as usize].unwrap() as u8) as char);
                    std::io::stdout().flush().unwrap();
                }
                Operator::Input(data) => {
                    self.registers[data.c as usize] = Plate::from(std::io::stdin()
                        .bytes()
                        .next()
                        .and_then(|result| result.ok())
                        .map(|byte| {
                            u32::from(byte)
                        }).unwrap());
                }
                Operator::LoadProgram(data) => {
                    self.pointer_finger = self.reg_get(data.c).unwrap();
                    let data_to_load_index = *self.reg_get(data.b);
                    if data_to_load_index != 0 {
                        self.memory[0] = self.memory[data_to_load_index.unwrap() as usize].clone();
                    }
                    continue;
                }
            };
            self.pointer_finger += 1;
        }
    }

    fn set_data(&mut self, index: u32, offset: u32, value: Plate) {
        self.memory[index as usize][offset as usize] = value;
    }

    fn reg_set(&mut self, index: u32, value: Plate) {
        self.registers[index as usize] = value;
    }

    fn reg_get(&mut self, index: u32) -> &Plate {
        &self.registers[index as usize]
    }

    fn reg_mov(&mut self, source: u32, dest: u32) {
        let data = *self.reg_get(source);
        self.reg_set(dest, data);
    }

    fn get_data(&self, index: u32, offset: u32) -> &Plate {
        &self.memory[index as usize][offset as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::Machine;
    use ::plate::Plate;

    #[test]
    fn machine_halt() {
        let halt_program = vec![Plate::from(0x70000000)];
        let mut machine = Machine::new(halt_program);
        machine.run();
        assert_eq!(machine.pointer_finger, 0);
    }
}