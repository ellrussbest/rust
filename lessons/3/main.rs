fn main() {
  let x = 5;
  println!("The value of x is: {}", x);

  // Values are immutable by default.
  // x = 6;
  // println!("The value of x is: {}", x);

  // Mutability allows a value to be reassigned.
  let mut x = 5;
  println!("The value of x is: {}", x);
  x = 6;
  println!("The value of x is: {}", x);

  // Constants are immutable, require a type, and use constant expressions.
  const MY_CONST: u32 = 10;
  println!("My Const is {}", MY_CONST);

  // Data types
  {
    // Scalar types represent single values.
    // Integers (i8...i128, isize(arch/architecture), u8...u128, usize(arch/architecture))
    let a: i32 = 982_222; // Decimal
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

    // Compound types group multiple values.
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
    println!("No condition was true");
  }

  let condition = true;
  let num = if condition { 5 } else { 6 };
  println!("{}", num);

  // A loop can return a value through `break`.
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
