#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Reg {
  NoRegister,
  R0,
  R1,
  R2,
  R3,
  R4,
  R5,
  R6,
  R7,
  R8,
  R9,
  R10,
  R11,
  W0,
  W1,
  W2,
  W3,
  W4,
  W5,
  W6,
  W7,
  W8,
  W9,
  W10,
  W11,
  NUM_TARGET_REGS,
  UNKNOWN(u64),
}

impl From<u64> for Reg {
    fn from(value: u64) -> Self {
        match value {
          0 => Reg::NoRegister,
          1 => Reg::R0,
          2 => Reg::R1,
          3 => Reg::R2,
          4 => Reg::R3,
          5 => Reg::R4,
          6 => Reg::R5,
          7 => Reg::R6,
          8 => Reg::R7,
          9 => Reg::R8,
          10 => Reg::R9,
          11 => Reg::R10,
          12 => Reg::R11,
          13 => Reg::W0,
          14 => Reg::W1,
          15 => Reg::W2,
          16 => Reg::W3,
          17 => Reg::W4,
          18 => Reg::W5,
          19 => Reg::W6,
          20 => Reg::W7,
          21 => Reg::W8,
          22 => Reg::W9,
          23 => Reg::W10,
          24 => Reg::W11,
          25 => Reg::NUM_TARGET_REGS,
          _ => Reg::UNKNOWN(value),
        }
    }
}