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

// handle enum via pattern matching
fn route(ip_kind: IpAddrKind) {
  match ip_kind {
    IpAddrKind::V4 => println!("Version V4"),
    IpAddrKind::V6 => println!("Version V6"),
  }
}

// match with data extraction
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
    None => None,
    Some(i) => Some(i + 1),
  }
}

fn main() {
  // simple enums
  let v4 = IpAddrKind::V4;
  let v6 = IpAddrKind::V6;

  // struct using enum + string
  let localhost_struct = IpAddr {
    address: String::from("127.0.0.1"),
    kind: v4,
  };

  // enum with data instead of struct
  let localhost_enum = IpAddrWithStorage {
    address: IpAddrKindWithStorage::V4(127, 0, 0, 1),
  };

  // message enum usage
  let msg = Message::Quit;
  msg.some_fn();

  // Option examples
  let some_number = Some(5);
  let some_string = Some("a string");
  let absent_number: Option<i32> = None;

  // combining Option with values
  let x: i8 = 5;
  let y: Option<i8> = Some(5);

  let sum_match = match y {
    Some(val) => val + x,
    None => x,
  };

  let sum_unwrap = x + y.unwrap_or(0);

  // function usage
  route(v6);

  value_in_cents(Coin::Quarter(UsState::Alaska));

  // Option transformation
  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);

  // match vs if let
  let some_val = Some(3);

  match some_val {
    Some(3) => println!("three"),
    _ => (),
  }

  if let Some(3) = some_val {
    println!("three");
  }
}
