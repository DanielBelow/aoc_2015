use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display, FromStr};
use std::ops::{AddAssign, DivAssign, MulAssign};

#[derive(Default, Copy, Clone, Debug)]
struct Cpu {
    reg_a: u32,
    reg_b: u32,
}

#[derive(FromStr, Display, Copy, Clone, Debug)]
pub enum Instruction {
    #[display("hlf {0}")]
    Half(char),

    #[display("tpl {0}")]
    Triple(char),

    #[display("inc {0}")]
    Inc(char),

    #[display("jmp {0}")]
    Jump(i32),

    #[display("jie {0}, {1}")]
    JumpIfEven(char, i32),

    #[display("jio {0}, {1}")]
    JumpIfOne(char, i32),
}

#[aoc_generator(day23)]
pub fn generate(inp: &str) -> Vec<Instruction> {
    inp.lines().filter_map(|it| it.parse().ok()).collect()
}

fn set_pc(pc: &mut usize, offset: i32) -> bool {
    if offset < 0 {
        match pc.checked_sub(offset.unsigned_abs() as usize) {
            None => return false,
            Some(npc) => *pc = npc,
        };
    } else {
        *pc += offset.unsigned_abs() as usize;
    }

    true
}

fn run_program(cpu: &mut Cpu, insts: &[Instruction]) {
    let mut pc = 0;
    loop {
        if pc >= insts.len() {
            break;
        }

        let cur_inst = insts[pc];
        match cur_inst {
            Instruction::Half(reg) => {
                if reg == 'a' {
                    cpu.reg_a.div_assign(&2);
                } else {
                    cpu.reg_b.div_assign(&2);
                }

                pc += 1;
            }
            Instruction::Triple(reg) => {
                if reg == 'a' {
                    cpu.reg_a.mul_assign(&3);
                } else {
                    cpu.reg_b.mul_assign(&3);
                }

                pc += 1;
            }
            Instruction::Inc(reg) => {
                if reg == 'a' {
                    cpu.reg_a.add_assign(&1);
                } else {
                    cpu.reg_b.add_assign(&1);
                }

                pc += 1;
            }
            Instruction::Jump(offset) => {
                if !set_pc(&mut pc, offset) {
                    break;
                }
            }
            Instruction::JumpIfEven(reg, offset) => {
                let reg = if reg == 'a' { cpu.reg_a } else { cpu.reg_b };

                if reg % 2 == 0 {
                    if !set_pc(&mut pc, offset) {
                        break;
                    }
                } else {
                    pc += 1;
                }
            }
            Instruction::JumpIfOne(reg, offset) => {
                let reg = if reg == 'a' { cpu.reg_a } else { cpu.reg_b };

                if reg == 1 {
                    if !set_pc(&mut pc, offset) {
                        break;
                    }
                } else {
                    pc += 1;
                }
            }
        };
    }
}

#[aoc(day23, part1)]
pub fn part1(insts: &[Instruction]) -> u32 {
    let mut cpu = Cpu::default();
    run_program(&mut cpu, insts);
    cpu.reg_b
}

#[aoc(day23, part2)]
pub fn part2(insts: &[Instruction]) -> u32 {
    let mut cpu = Cpu { reg_a: 1, reg_b: 0 };
    run_program(&mut cpu, insts);
    cpu.reg_b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let inp = "inc a\n\
                          jio a, +2\n\
                          tpl a\n\
                          inc a";
        let gen = generate(inp);

        let mut cpu = Cpu::default();
        run_program(&mut cpu, &gen);
        assert_eq!(cpu.reg_a, 2);
    }
}
