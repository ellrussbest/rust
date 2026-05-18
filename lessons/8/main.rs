use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
  // collection grow on the heap
  let mut v: Vec<i32> = Vec::new();
  v.push(1);
  v.push(2);
  v.push(3);

  let v2 = vec![1, 2, 3, 4, 5];

  let third = &v2[2];
  println!("The third element is {}", third);

  match v2.get(20) {
    Some(twentieth) => println!("The twentieth element is {}", twentieth),
    None => println!("There is no twentieth element"),
  };

  let mut v = vec![1, 2, 3, 4, 5];
  let third = &v[2];

  // illegal
  // already borrowed as immutable
  // cannot have mutable and immutable at same scope
  // third might have changed... ergo corruption
  //   v.push(6);
  println!("The third element is {}", third);

  // iteration
  let mut v = vec![1, 2, 3, 4, 5];

  for i in &v {
    println!("{}", i);
  }

  // mutable
  for i in &mut v {
    *i *= 2;
  }

  // storing enum variants
  enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
  }

  let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
  ];

  match &row[1] {
    SpreadsheetCell::Int(i) => println!("{}", i),
    _ => println!("Not an integer"),
  }

  // Strings
  // Strings are stored as a collection of UTF-8 encoded bytes
  let s1 = String::new();
  let s2 = "Initial contents";
  let s3 = s2.to_string();
  let s4 = String::from("initial contents");
  let s5 = s1 + &s2;

  // illegal...
  // ownership was moved from s1 to s5
  //   println!("{}", s1);

  let s6 = format!("{}{}", s3, s4);

  let hello = String::from("नमस्ते");

  // Bytes
  // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
  for b in "नमस्ते".bytes() {
    println!("{}", b);
  }

  // Scalar values
  // ['न', 'म', 'स', '्', 'त', 'े']
  for c in "नमस्ते".chars() {
    println!("{}", c);
  }

  // Grapheme clusters
  // ["न", "म", "स्", "ते"]
  for g in "नमस्ते".graphemes(true) {
    println!("{}", g);
  }

  // Hashmaps
  let blue = String::from("Blue");
  let yellow = String::from("yellow");

  let mut scores = HashMap::new();
  scores.insert(blue, 10);
  scores.insert(yellow, 50);

  let team_name = String::from("Blue");
  let score = scores.get(&team_name);

  let res = match score {
    Some(val) => *val,
    None => 0,
  };
  println!("{}", res);

  for (key, value) in &scores {
    println!("{}: {}", key, value);
  }

  // overrides
  scores.insert(String::from("blue"), 10);
  scores.insert(String::from("blue"), 20);

  // doesn't override... creates entry or does nothing!
  scores.entry(String::from("yellow")).or_insert(30);
  scores.entry(String::from("yellow")).or_insert(40);

  let text = "hello world wonderful world";
  let mut map = HashMap::new();

  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }

  println!("{:?}", map);
}
