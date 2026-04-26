struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.height * self.width
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

impl Rectangle {
  fn square(size: u32) -> Rectangle {
    Rectangle {
      width: size,
      height: size,
    }
  }
}

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

fn main() {
  let mut user1 = User {
    active: true,
    email: String::from("name1@email.com"),
    sign_in_count: 1,
    username: String::from("John One"),
  };

  let name = user1.username;
  user1.username = String::from("Jane One");

  let user2 =
    create_user(String::from("name2@email.com"), String::from("John Two"));

  let user3 = User {
    email: String::from("name3@email.com"),
    username: String::from("John Three"),
    ..user2 // Move operation
  };

  // tuple struct
  struct Color(i32, i32, i32);
  struct Point(i32, i32, i32);

  let rect = Rectangle {
    height: 10,
    width: 20,
  };

  let rect2 = Rectangle {
    height: 5,
    width: 10,
  };
  let rect3 = Rectangle {
    height: 40,
    width: 50,
  };
  let sq = Rectangle::square(10);

  // {} → uses the Display trait (user-friendly output)
  // {:?} → debug output (single line)
  // {:#?} → pretty debug (multi-line, nicely indented)
  println!("The area of the rectangle is {} sq pixels.", rect.area());
  println!("{:#?}", rect);

  println!("rect can hold rect2: {}", rect.can_hold(&rect2));
  println!("rect can hold rect3: {}", rect.can_hold(&rect3));
  println!("Square: {:#?}", sq);
}

fn create_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 1,
  }
}
