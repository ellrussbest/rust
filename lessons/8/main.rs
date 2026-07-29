#![allow(dead_code)] // Every enum variant is retained for demonstration.
#![allow(clippy::useless_vec)] // These examples intentionally teach vectors.
#![allow(clippy::vec_init_then_push)] // The first example demonstrates `Vec::push`.

use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
  // Vectors store a growable sequence of values on the heap.
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

  #[allow(unused_mut)]
  // `mut` is required by the commented-out `push` example below.
  let mut v = vec![1, 2, 3, 4, 5];
  let third = &v[2];

  // This would not compile because `third` immutably borrows `v`.
  // Mutating `v` could reallocate its storage and invalidate that reference.
  //   v.push(6);
  println!("The third element is {}", third);

  // iteration
  let mut v = vec![1, 2, 3, 4, 5];

  for i in &v {
    println!("{}", i);
  }

  // Mutate each element through a mutable reference.
  for i in &mut v {
    *i *= 2;
  }

  // An enum lets a vector store values with different shapes.
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
  let _s5 = s1 + s2;

  // illegal...
  // ownership was moved from s1 to s5
  //   println!("{}", s1);

  let _s6 = format!("{}{}", s3, s4);

  let _hello = String::from("नमस्ते");

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

  // Hash maps associate keys with values.
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

  // Inserting an existing key replaces its value.
  scores.insert(String::from("blue"), 10);
  scores.insert(String::from("blue"), 20);

  // `entry().or_insert()` inserts only when the key is absent.
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
