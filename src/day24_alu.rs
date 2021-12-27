use lazy_static::lazy_static;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MemoryState {
  w: i64,
  z: i64,
}

impl MemoryState {
  pub fn new(w: i64, z: i64) -> Self {
    Self {w, z}
  }

  pub fn z(&self) -> i64 {
    self.z
  }
}

// parsed manually 
fn validate_digit_0(memory: &MemoryState) -> MemoryState {
  let mut mem = memory.clone();

  // mul x 0
  let mut x: i64 = 0;
  // add x z
  x = mem.z;
  // mod x 26
  x %= 26;
  // div z 1
  mem.z /= 1;
  // add x 13
  x += 13;
  // eql x w
  x = if x == mem.w {1} else {0};
  // eql x 0
  x = if x == 0 {1} else {0};
  // mul y 0
  let mut y: i64 = 0;
  // add y 25
  y += 25;
  // mul y x
  y *= x;
  // add y 1
  y += 1;
  // mul z y
  mem.z *= y;
  // mul y 0
  y *= 0;
  // add y w
  y += mem.w;
  // add y 0
  y += 0;
  // mul y x
  y *= x;
  // add z y
  mem.z += y;

  mem
}

fn validate_digit_1(memory: &MemoryState) -> MemoryState {
  let mut mem = memory.clone();

  // mul x 0
  let mut x: i64 = 0;
  // add x z
  x += mem.z;
  // mod x 26
  x %= 26;
  // div z 1
  mem.z /= 1;
  // add x 11
  x += 11;
  // eql x w
  x = if x == mem.w {1} else {0};
  // eql x 0
  x = if x == 0 {1} else {0};
  // mul y 0
  let mut y: i64 = 0;
  // add y 25
  y += 25;
  // mul y x
  y *= x;
  // add y 1
  y += 1;
  // mul z y
  mem.z *= y;
  // mul y 0
  y *= 0;
  // add y w
  y += mem.w;
  // add y 3
  y += 3;
  // mul y x
  y *= x;
  // add z y
  mem.z += y;

  mem
}

fn validate_digit_2(memory: &MemoryState) -> MemoryState {
  let mut mem = memory.clone();

  // mul x 0
  let mut x: i64 = 0;
  // add x z
  x += mem.z;
  // mod x 26
  x %= 26;
  // div z 1
  mem.z /= 1;
  // add x 14
  x += 14;
  // eql x w
  x = if x == mem.w {1} else {0};
  // eql x 0
  x = if x == 0 {1} else {0};
  // mul y 0
  let mut y: i64 = 0;
  // add y 25
  y += 25;
  // mul y x
  y *= x;
  // add y 1
  y += 1;
  // mul z y
  mem.z *= y;
  // mul y 0
  y *= 0;
  // add y w
  y += mem.w;
  // add y 8
  y += 8;
  // mul y x
  y *= x;
  // add z y
  mem.z += y;

  mem
}

fn validate_digit_3(memory: &MemoryState) -> MemoryState {
  let mut mem = memory.clone();

  // mul x 0
  let mut x: i64 = 0;
  // add x z
  x += mem.z;
  // mod x 26
  x %= 26;
  // div z 26
  mem.z /= 26;
  // add x -5
  x -= 5;
  // eql x w
  x = if x == mem.w {1} else {0};
  // eql x 0
  x = if x == 0 {1} else {0};
  // mul y 0
  let mut y: i64 = 0;
  // add y 25
  y += 25;
  // mul y x
  y *= x;
  // add y 1
  y += 1;
  // mul z y
  mem.z *= y;
  // mul y 0
  y *= 0;
  // add y w
  y += mem.w;
  // add y 5
  y += 5;
  // mul y x
  y *= x;
  // add z y
  mem.z += y;

  mem
}

fn validate_digit_4(memory: &MemoryState) -> MemoryState {
  let mut mem = memory.clone();

  // mul x 0
  let mut x: i64 = 0;
  // add x z
  x += mem.z;
  // mod x 26
  x %= 26;
  // div z 1
  mem.z /= 1;
  // add x 14
  x += 14;
  // eql x w
  x = if x == mem.w {1} else {0};
  // eql x 0
  x = if x == 0 {1} else {0};
  // mul y 0
  let mut y: i64 = 0;
  // add y 25
  y += 25;
  // mul y x
  y *= x;
  // add y 1
  y += 1;
  // mul z y
  mem.z *= y;
  // mul y 0
  y *= 0;
  // add y w
  y += mem.w;
  // add y 13
  y += 13;
  // mul y x
  y *= x;
  // add z y
  mem.z += y;

  mem
}

fn validate_digit_5(memory: &MemoryState) -> MemoryState {
  let mut mem = memory.clone();

  // mul x 0
  let mut x: i64 = 0;
  // add x z
  x += mem.z;
  // mod x 26
  x %= 26;
  // div z 1
  mem.z /= 1;
  // add x 10
  x += 10;
  // eql x w
  x = if x == mem.w {1} else {0};
  // eql x 0
  x = if x == 0 {1} else {0};
  // mul y 0
  let mut y: i64 = 0;
  // add y 25
  y += 25;
  // mul y x
  y *= x;
  // add y 1
  y += 1;
  // mul z y
  mem.z *= y;
  // mul y 0
  y *= 0;
  // add y w
  y += mem.w;
  // add y 9
  y += 9;
  // mul y x
  y *= x;
  // add z y
  mem.z += y;

  mem
}

fn validate_digit_6(memory: &MemoryState) -> MemoryState {
  let mut mem = memory.clone();

  // mul x 0
  let mut x: i64 = 0;
  // add x z
  x += mem.z;
  // mod x 26
  x %= 26;
  // div z 1
  mem.z /= 1;
  // add x 12
  x += 12;
  // eql x w
  x = if x == mem.w {1} else {0};
  // eql x 0
  x = if x == 0 {1} else {0};
  // mul y 0
  let mut y: i64 = 0;
  // add y 25
  y += 25;
  // mul y x
  y *= x;
  // add y 1
  y += 1;
  // mul z y
  mem.z *= y;
  // mul y 0
  y *= 0;
  // add y w
  y += mem.w;
  // add y 6
  y += 6;
  // mul y x
  y *= x;
  // add z y
  mem.z += y;

  mem
}

fn validate_digit_7(memory: &MemoryState) -> MemoryState {
  let mut mem = memory.clone();

  // mul x 0
  let mut x: i64 = 0;
  // add x z
  x += mem.z;
  // mod x 26
  x %= 26;
  // div z 26
  mem.z /= 26;
  // add x -14
  x -= 14;
  // eql x w
  x = if x == mem.w {1} else {0};
  // eql x 0
  x = if x == 0 {1} else {0};
  // mul y 0
  let mut y: i64 = 0;
  // add y 25
  y += 25;
  // mul y x
  y *= x;
  // add y 1
  y += 1;
  // mul z y
  mem.z *= y;
  // mul y 0
  y *= 0;
  // add y w
  y += mem.w;
  // add y 1
  y += 1;
  // mul y x
  y *= x;
  // add z y
  mem.z += y;

  mem
}

fn validate_digit_8(memory: &MemoryState) -> MemoryState {
  let mut mem = memory.clone();

  // mul x 0
  let mut x: i64 = 0;
  // add x z
  x += mem.z;
  // mod x 26
  x %= 26;
  // div z 26
  mem.z /= 26;
  // add x -8
  x -= 8;
  // eql x w
  x = if x == mem.w {1} else {0};
  // eql x 0
  x = if x == 0 {1} else {0};
  // mul y 0
  let mut y: i64 = 0;
  // add y 25
  y += 25;
  // mul y x
  y *= x;
  // add y 1
  y += 1;
  // mul z y
  mem.z *= y;
  // mul y 0
  y *= 0;
  // add y w
  y += mem.w;
  // add y 1
  y += 1;
  // mul y x
  y *= x;
  // add z y
  mem.z += y;

  mem
}

fn validate_digit_9(memory: &MemoryState) -> MemoryState {
  let mut mem = memory.clone();

  // mul x 0
  let mut x: i64 = 0;
  // add x z
  x += mem.z;
  // mod x 26
  x %= 26;
  // div z 1
  mem.z /= 1;
  // add x 13
  x += 13;
  // eql x w
  x = if x == mem.w {1} else {0};
  // eql x 0
  x = if x == 0 {1} else {0};
  // mul y 0
  let mut y: i64 = 0;
  // add y 25
  y += 25;
  // mul y x
  y *= x;
  // add y 1
  y += 1;
  // mul z y
  mem.z *= y;
  // mul y 0
  y *= 0;
  // add y w
  y += mem.w;
  // add y 2
  y += 2;
  // mul y x
  y *= x;
  // add z y
  mem.z += y;

  mem
}

fn validate_digit_10(memory: &MemoryState) -> MemoryState {
  let mut mem = memory.clone();

  // mul x 0
  let mut x: i64 = 0;
  // add x z
  x += mem.z;
  // mod x 26
  x %= 26;
  // div z 26
  mem.z /= 26;
  // add x 0
  x += 0;
  // eql x w
  x = if x == mem.w {1} else {0};
  // eql x 0
  x = if x == 0 {1} else {0};
  // mul y 0
  let mut y: i64 = 0;
  // add y 25
  y += 25;
  // mul y x
  y *= x;
  // add y 1
  y += 1;
  // mul z y
  mem.z *= y;
  // mul y 0
  y *= 0;
  // add y w
  y += mem.w;
  // add y 7
  y += 7;
  // mul y x
  y *= x;
  // add z y
  mem.z += y;

  mem
}

fn validate_digit_11(memory: &MemoryState) -> MemoryState {
  let mut mem = memory.clone();

  // mul x 0
  let mut x: i64 = 0;
  // add x z
  x += mem.z;
  // mod x 26
  x %= 26;
  // div z 26
  mem.z /= 26;
  // add x -5
  x -= 5;
  // eql x w
  x = if x == mem.w {1} else {0};
  // eql x 0
  x = if x == 0 {1} else {0};
  // mul y 0
  let mut y: i64 = 0;
  // add y 25
  y += 25;
  // mul y x
  y *= x;
  // add y 1
  y += 1;
  // mul z y
  mem.z *= y;
  // mul y 0
  y *= 0;
  // add y w
  y += mem.w;
  // add y 5
  y += 5;
  // mul y x
  y *= x;
  // add z y
  mem.z += y;

  mem
}

fn validate_digit_12(memory: &MemoryState) -> MemoryState {
  let mut mem = memory.clone();

  // mul x 0
  let mut x: i64 = 0;
  // add x z
  x += mem.z;
  // mod x 26
  x %= 26;
  // div z 26
  mem.z /= 26;
  // add x -9
  x -= 9;
  // eql x w
  x = if x == mem.w {1} else {0};
  // eql x 0
  x = if x == 0 {1} else {0};
  // mul y 0
  let mut y: i64 = 0;
  // add y 25
  y += 25;
  // mul y x
  y *= x;
  // add y 1
  y += 1;
  // mul z y
  mem.z *= y;
  // mul y 0
  y *= 0;
  // add y w
  y += mem.w;
  // add y 8
  y += 8;
  // mul y x
  y *= x;
  // add z y
  mem.z += y;

  mem
}

fn validate_digit_13(memory: &MemoryState) -> MemoryState {
  let mut mem = memory.clone();

  // mul x 0
  let mut x: i64 = 0;
  // add x z
  x += mem.z;
  // mod x 26
  x %= 26;
  // div z 26
  mem.z /= 26;
  // add x -1
  x -= 1;
  // eql x w
  x = if x == mem.w {1} else {0};
  // eql x 0
  x = if x == 0 {1} else {0};
  // mul y 0
  let mut y: i64 = 0;
  // add y 25
  y += 25;
  // mul y x
  y *= x;
  // add y 1
  y += 1;
  // mul z y
  mem.z *= y;
  // mul y 0
  y *= 0;
  // add y w
  y += mem.w;
  // add y 15
  y += 15;
  // mul y x
  y *= x;
  // add z y
  mem.z += y;

  mem
}

lazy_static! {
  pub static ref VALIDATORS: Vec<fn(&MemoryState) -> MemoryState> = vec![
    validate_digit_0,
    validate_digit_1, 
    validate_digit_2, 
    validate_digit_3, 
    validate_digit_4, 
    validate_digit_5, 
    validate_digit_6, 
    validate_digit_7, 
    validate_digit_8, 
    validate_digit_9, 
    validate_digit_10,
    validate_digit_11,
    validate_digit_12,
    validate_digit_13, 
  ];
}