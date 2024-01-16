use crate::day16::{Computer, Value};
use crate::day19::Program;
use std::collections::HashSet;

fn check_finish(program: &Program, init: Value) -> bool {
  let mut comp = Computer::new(6);
  let mut ip: Value = 0;
  let mut visited = HashSet::<Vec<Value>>::new();
  comp.reg[0] = init;
  loop {
    if !visited.insert(comp.reg.clone()) { return false; }
    if (ip as usize) >= program.size() { return true; }
    ip = program.step(&mut comp, ip);
  }
}

pub fn run(content: &str) {
  let program = Program::parse(content);
  println!("{}", check_finish(&program, 0));
}
