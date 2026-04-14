fn main() {
  let x = 5;
  println!("The value of x is: {}", x);

  // cannot re-assign: values immutable by default
  // x = 6;
  // println!("The value of x is: {}", x);

  // MUTATION & SHADOWING:
  // make values mutable by:
  let mut x = 5;
  println!("The value of x is: {}", x);
  // can re-assign: values marked as mutable
  x = 6;
  println!("The value of x is: {}", x);

  // CONST
  // constant values: Can never be modified
  // consts should always be type-annotated
  // can never be set to any computed values at runtime
  const MY_CONST: u32 = 10;
  println!("My Const is {}", MY_CONST);

  // DATATYPES
  {
    // SCALAR: Single Values
    // Integers (i8...i128, isize(arch/architecture), u8...u128, usize(arch/architecture))
    let a: i32 = 98_2222; // Decimal
    let b: i32 = 0xff; // Hex
    let c: i32 = 0o77; // Octal
    let d: i32 = 0b1111_0000; // Binary
    let e: u8 = b'A'; // Byte (u8 only)

    println!("{}, {}, {}, {}, {}", a, b, c, d, e);

    // Floating-point numbers (f32, f64 -> (default: double point precision))
    let a: f64 = 2.0; // f64
    let b: f32 = 3.0; // f32
    println!("{}, {}", a, b);

    // Booleans (true/false)
    // Character (unicode)
    let a = 'Z';
    let b = 'z';
    let emoji = '😀';
    println!("{}, {}, {}", a, b, emoji);

    // COMPOUND: Group of values
    // 1. TUPLE: fixed size array of related data of different types
    let tup = ("John Doe", 50);

    // Destructure tuple
    let (name, age) = tup;
    println!("{}, {}", name, age);

    // Access through Dot notation
    let name = tup.0;
    let age = tup.1;
    println!("{}, {}", name, age);

    // 2. ARRAYS: fixed length
    let err_codes = [200, 404, 5000];
    let byte = [0; 8]; // create an array with 8 values all preset with 0
    println!("{}, {}", err_codes[0], byte[0]);
  }

  // Control flow
  let num = 5;

  // a condition must explicity be a boolean!
  if num < 10 {
    println!("first condition was true");
  } else if num < 22 {
    println!("second condition was true");
  } else {
    println!("condition was flase");
  }

  let condition = true;
  let num = if condition { 5 } else { 6 };
  println!("{}", num);

  // Loop:
  let mut count = 0;
  let res = loop {
    if count == 10 {
      break count;
    } else {
      count += 1;
    }

    println!("[LOOP]: again!")
  };
  println!("[LOOP Returns]: {}", res);

  let mut num = 3;

  while num != 0 {
    println!("[WHILE]: {}!", num);

    num -= 1;
  }

  let list = [10, 20, 30, 40, 50];

  for el in list.iter() {
    println!("[FOR]: Collection {}", el);
  }

  for num in 1..=10 {
    println!("[FOR]: Range: {}", num);
  }
}
