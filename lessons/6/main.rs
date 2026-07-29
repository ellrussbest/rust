#![allow(dead_code)] // Some declarations exist solely to demonstrate the lesson concepts.
#![allow(clippy::manual_map)] // `plus_one` demonstrates exhaustive `Option` matching.
#![allow(clippy::single_match)] // The final example intentionally compares `match` with `if let`.
#![allow(clippy::unnecessary_literal_unwrap)] // Demonstrates `Option::unwrap_or`.

#[derive(Debug)]
enum IpAddrKind {
  V4,
  V6,
}

enum IpAddrKindWithStorage {
  V4(u8, u8, u8, u8),
  V6(String),
}

enum Message {
  Quit,                    // no data
  Move { x: i32, y: i32 }, // struct-like
  Write(String),           // tuple-like
  ChangeColor(i32, i32, i32),
}

impl Message {
  fn some_fn(&self) {
    println!("Let's get rusty!");
  }
}

struct IpAddr {
  kind: IpAddrKind,
  address: String,
}

#[derive(Debug)]
enum UsState {
  Alabama,
  Alaska,
  Arizona,
  Arkansas,
  California,
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState), // carries extra data
}

struct IpAddrWithStorage {
  address: IpAddrKindWithStorage,
}

// Handle every enum variant with exhaustive pattern matching.
fn route(ip_kind: IpAddrKind) {
  match ip_kind {
    IpAddrKind::V4 => println!("Version V4"),
    IpAddrKind::V6 => println!("Version V6"),
  }
}

// Extract data stored inside an enum variant.
fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
      println!("State quarter from {:?}!", state);
      25
    }
  }
}

// Option handling
fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    Some(value) => Some(value + 1),
    None => None,
  }
}

fn main() {
  // simple enums
  let v4 = IpAddrKind::V4;
  let v6 = IpAddrKind::V6;

  // struct using enum + string
  let _localhost_struct = IpAddr {
    address: String::from("127.0.0.1"),
    kind: v4,
  };

  // enum with data instead of struct
  let _localhost_enum = IpAddrWithStorage {
    address: IpAddrKindWithStorage::V4(127, 0, 0, 1),
  };

  // message enum usage
  let msg = Message::Quit;
  msg.some_fn();

  // Option examples
  let _some_number = Some(5);
  let _some_string = Some("a string");
  let _absent_number: Option<i32> = None;

  // combining Option with values
  let x: i8 = 5;
  let y: Option<i8> = Some(x);

  let _sum_match = match y {
    Some(val) => val + x,
    None => x,
  };

  let _sum_unwrap = x + y.unwrap_or(0);

  // function usage
  route(v6);

  value_in_cents(Coin::Quarter(UsState::Alaska));

  // Option transformation
  let five = Some(5);
  let _six = plus_one(five);
  let _none = plus_one(None);

  // match vs if let
  let some_val = Some(3);

  match some_val {
    Some(3) => println!("three from match"),
    _ => (),
  }

  if let Some(3) = some_val {
    println!("three");
  }
}
