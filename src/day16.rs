use self::OpCode::*;
use std::collections::HashSet;
use std::slice::Iter;

type Value = u32;
type Registers = [Value; 4];
type Input = [Value; 4];
type Sample = (Registers, Input, Registers);
type PossibleOps = [Vec<OpCode>; 16];
type MappedOps = [OpCode; 16];

#[derive(Debug)]
struct Computer {
  reg: Registers,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum OpCode {
  AddR, AddI,
  MulR, MulI,
  BanR, BanI,
  BorR, BorI,
  SetR, SetI,
  GtIR, GtRI, GtRR,
  EqIR, EqRI, EqRR,
}

impl Computer {
  fn addr(&mut self, a: usize, b: usize, c: usize) {
    self.reg[c] = self.reg[a] + self.reg[b];
  }
  fn addi(&mut self, a: usize, b: Value, c: usize) {
    self.reg[c] = self.reg[a] + b;
  }
  fn mulr(&mut self, a: usize, b: usize, c: usize) {
    self.reg[c] = self.reg[a] * self.reg[b];
  }
  fn muli(&mut self, a: usize, b: Value, c: usize) {
    self.reg[c] = self.reg[a] * b;
  }
  fn banr(&mut self, a: usize, b: usize, c: usize) {
    self.reg[c] = self.reg[a] & self.reg[b];
  }
  fn bani(&mut self, a: usize, b: Value, c: usize) {
    self.reg[c] = self.reg[a] & b;
  }
  fn borr(&mut self, a: usize, b: usize, c: usize) {
    self.reg[c] = self.reg[a] | self.reg[b];
  }
  fn bori(&mut self, a: usize, b: Value, c: usize) {
    self.reg[c] = self.reg[a] | b;
  }
  fn setr(&mut self, a: usize, c: usize) {
    self.reg[c] = self.reg[a];
  }
  fn seti(&mut self, a: Value, c: usize) {
    self.reg[c] = a;
  }
  fn gtir(&mut self, a: Value, b: usize, c: usize) {
    self.reg[c] = (a > self.reg[b]) as Value;
  }
  fn gtri(&mut self, a: usize, b: Value, c: usize) {
    self.reg[c] = (self.reg[a] > b) as Value;
  }
  fn gtrr(&mut self, a: usize, b: usize, c: usize) {
    self.reg[c] = (self.reg[a] > self.reg[b]) as Value;
  }
  fn eqir(&mut self, a: Value, b: usize, c: usize) {
    self.reg[c] = (a == self.reg[b]) as Value;
  }
  fn eqri(&mut self, a: usize, b: Value, c: usize) {
    self.reg[c] = (self.reg[a] == b) as Value;
  }
  fn eqrr(&mut self, a: usize, b: usize, c: usize) {
    self.reg[c] = (self.reg[a] == self.reg[b]) as Value;
  }

  fn execute(&mut self, op: OpCode, a: Value, b: Value, c: Value) {
    let (ar, br, cr) = (a as usize, b as usize, c as usize);
    match op {
      AddR => self.addr(ar, br, cr),
      AddI => self.addi(ar, b, cr),
      MulR => self.mulr(ar, br, cr),
      MulI => self.muli(ar, b, cr),
      BanR => self.banr(ar, br, cr),
      BanI => self.bani(ar, b, cr),
      BorR => self.borr(ar, br, cr),
      BorI => self.bori(ar, b, cr),
      SetR => self.setr(ar, cr),
      SetI => self.seti(a, cr),
      GtIR => self.gtir(a, br, cr),
      GtRI => self.gtri(ar, b, cr),
      GtRR => self.gtrr(ar, br, cr),
      EqIR => self.eqir(a, br, cr),
      EqRI => self.eqri(ar, b, cr),
      EqRR => self.eqrr(ar, br, cr),
    }
  }

  fn execute_all(&mut self, inputs: &[Input], mapping: MappedOps) {
    for &[idx, a, b, c] in inputs {
      self.execute(mapping[idx as usize], a, b, c);
    }
  }

  fn parse_sample<'a>(text: &'a str) -> Sample {
    let lines = text.lines().collect::<Vec<_>>();
    let clean = |s: &'a str| s.split_once('[').unwrap().1.trim_end_matches(']');
    let parse = |s: &str, d: &str| -> Vec<Value> {
      s.split(d).map(|x| x.parse::<Value>().unwrap()).collect()
    };
    let before: Registers = parse(clean(lines[0]), ", ").try_into().unwrap();
    let input: Input = parse(lines[1], " ").try_into().unwrap();
    let after: Registers = parse(clean(lines[2]), ", ").try_into().unwrap();
    (before, input, after)
  }

  fn parse_samples(text: &str) -> Vec<Sample> {
    text.split("\n\n").map(|s| Self::parse_sample(s)).collect()
  }

  fn parse_inputs(text: &str) -> Vec<Input> {
    text.lines().map(|s| {
      s.split(' ').map(|x| x.parse::<Value>().unwrap()).collect::<Vec<_>>()
        .try_into().unwrap()
    }).collect()
  }

  fn parse(text: &str) -> (Vec<Sample>, Vec<Input>) {
    let (s1, s2) = text.split_once("\n\n\n\n").unwrap();
    (Self::parse_samples(s1), Self::parse_inputs(s2))
  }

  fn find_mapping(options: PossibleOps) -> Option<MappedOps> {
    let mut defined = HashSet::<OpCode>::new();
    let mut valid = Vec::<(usize, usize)>::new();
    for (k, v) in options.iter().enumerate() {
      match v.len() {
        0 => return None,
        1 => if !defined.insert(v[0]) { return None; },
        n => valid.push((n, k)),
      }
    }
    if defined.len() == options.len() {
      let result = options.into_iter().map(|a| a[0]).collect::<Vec<_>>();
      return result.try_into().ok();
    }
    let idx = valid.into_iter().min().unwrap().1;
    for op in &options[idx] {
      defined.insert(*op);
      let mut copy = options.clone();
      for line in copy.iter_mut() {
        if line.len() == 1 { continue; }
        line.retain(|x| !defined.contains(x));
      }
      copy[idx] = vec![*op];
      let res = Self::find_mapping(copy);
      if res.is_some() { return res; }
      defined.remove(op);
    }
    None
  }
}

impl OpCode {
  fn iterator() -> Iter<'static, OpCode> {
    static OPCODES: [OpCode; 16] = [
      AddR, AddI, MulR, MulI, BanR, BanI, BorR, BorI,
      SetR, SetI, GtIR, GtRI, GtRR, EqIR, EqRI, EqRR,
    ];
    OPCODES.iter()
  }
}

fn get_matching_opcodes((before, input, after): Sample) -> Vec<OpCode> {
  OpCode::iterator().filter(|&&op| {
    let mut comp = Computer { reg: before };
    comp.execute(op, input[1], input[2], input[3]);
    comp.reg == after
  }).copied().collect()
}

fn deduce_opcode_mapping(samples: &[Sample]) -> MappedOps {
  let init = OpCode::iterator().copied().collect::<Vec<_>>();
  let mut options: PossibleOps = vec![init; 16].try_into().unwrap();
  for sample in samples {
    let idx = sample.1[0] as usize;
    let matching = HashSet::<OpCode>::from_iter(
      get_matching_opcodes(*sample).into_iter());
    options[idx].retain(|op| matching.contains(op));
  }
  Computer::find_mapping(options).unwrap()
}

fn calculate(samples: &[Sample], inputs: &[Input]) -> Registers {
  let mut comp = Computer { reg: [0, 0, 0, 0] };
  comp.execute_all(inputs, deduce_opcode_mapping(samples));
  comp.reg
}

pub fn run(content: &str) {
  let (samples, inputs) = Computer::parse(content);
  let res1 = samples.iter()
    .filter(|&&s| get_matching_opcodes(s).len() >= 3).count();
  let res2 = calculate(&samples, &inputs)[0];
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  const TEST: &str = "\
Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]";

  #[test]
  fn small() {
    let sample = super::Computer::parse_sample(TEST);
    let opcodes = super::get_matching_opcodes(sample);
    assert_eq!(opcodes, [super::AddI, super::MulR, super::SetI]);
  }
}
