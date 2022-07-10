use std::fmt;

struct Chunk {
  name: String,
  lines: Vec<u32>,
  code: Vec<Instruction>,
}

impl Chunk {
  fn new(name: String) -> Self {
    Chunk { name, lines: vec![], code: vec![] }
  }

  fn write(&mut self, line: u32, ins: Instruction) {
    self.lines.push(line);
    self.code.push(ins);
  }
}

#[derive(Debug)]
enum Value {
  Number(f64),
}

enum Instruction {
  Return,
  Constant(Value),
}

impl fmt::Debug for Chunk {
  fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
    println!("== {} ==", self.name);

    let mut prev_line = u32::MAX;
    for (index, (instruction, &line)) in self.code.iter().zip(&self.lines).enumerate() {
      if prev_line == line {
        println!("{index:0>5} |      | {instruction:?}");
      } else {
        prev_line = line;
        println!("{index:0>5} | {line:>4} | {instruction:?}");
      }
    }

    Ok(())
  }
}

impl fmt::Debug for Instruction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
        Self::Return => write!(f, "Return"),
        Self::Constant(value) => write!(f, "Constant({:?})", value),
    }
  }
}

fn main() {
  let mut c = Chunk::new("luiz test".into());

  c.write(1, Instruction::Constant(Value::Number(1.0)));
  c.write(1, Instruction::Constant(Value::Number(2.0)));
  c.write(2, Instruction::Return);

  println!("{c:?}");
}
