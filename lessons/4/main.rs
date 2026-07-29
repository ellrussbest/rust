fn main() {
  // Ownership rules:
  // 1. Every value has one owner.
  // 2. Ownership can move to another binding or function.
  // 3. A value is dropped when its owner leaves scope.

  {
    // s is not valid here, it's not yet declared
    let s = "hello"; // s is valid from this point forward
    println!("{}", s); // perform operation with s
  } // this scope is now over, and s is no longer valid

  {
    let x = 5;
    let y = x; // copy
    println!("Copy X: {}", y);

    let s1 = String::from("hello");
    let s2 = s1; // move
    println!("Move s1 {}", s2);
    // println!("Move s1 {}", s1); // illegal

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("Clone s1 {}", s2);
    println!("Clone s1 {}", s1); // clone
  }

  {
    // 1.
    let s = String::from("hello");
    fn takes_ownership(s: String) {
      println!("Move {}", s)
    }
    takes_ownership(s);
    // println!("Move s {}", s); // illegal

    // 2.
    let a: i32 = 10;
    fn makes_copy(a: i32) {
      println!("Copy {}", a);
    }
    makes_copy(a);
    println!("Copy a {}", a); // copy

    // 3.
    fn gives_ownership() -> String {
      String::from("hello")
    }
    let s = gives_ownership();
    println!("Give {}", s);

    // 4.
    let s1 = String::from("hello");
    fn takes_and_gives_back(s: String) -> String {
      s
    }
    let s2 = takes_and_gives_back(s1);
    println!(
      "Takes and gives back to someone who might want to use it: {}",
      s2
    );

    // 5.
    let s1 = String::from("hello");
    let mut s2 = String::from("hello");
    fn borrow(s1: &str, s2: &mut String) {
      // s.push_str("oops"); // illegal
      s2.push_str(" world");
      println!("strlen {}", s1.len())
    }
    borrow(&s1, &mut s2);
    println!("S1 & s2 was just borrowed {} {}", s1, s2);

    // Only one mutable reference may be active at a time.
    // this helps in preventing data races at compile time e.g. a data-race occurs
    // when two pointers point to the same variable and one pointer is used to write to the same data
    // and there's no mechanism to synchronize data access between them
    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s; // illegal

    r1.push(' ');

    {
      // different scope
      let r2 = &mut s;
      r2.push_str("world");
    }

    println!("{}", s);

    // Borrowing rules
    // 1. At any given time, you can have either one mutable reference or any number of immutable references.
    // 2. References must always be valid.

    // 6.

    // A mutable reference cannot overlap active immutable references.
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s; // illegal
    println!("{} {}", r1, r2); // with the above illegal, data could be corrupted... we could be reading from stale, or changed data!

    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2);
    let r3 = &mut s; // allowed
    println!("{}", r3);

    // 7.
    // fn dangle() -> &String {
    //   let s = String::from("hello");
    //   &s
    // }

    // String slices
    let s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..];
    let hello_world = &s[..];
    let f_word = first_word(&s);
    // s.clear(); // illegal
    println!("The first word is: {}", f_word);

    fn first_word(s: &str) -> &str {
      let bytes = s.as_bytes();

      for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
          return &s[..i];
        }
      }

      s
    }

    // Collection slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[0..2];
    println!("String slices: {hello}, {world}, {hello_world}");
    println!("Collection slice: {slice:?}");
  }
}
