/**
 * Life time Elision Rules
 * 1. Each parameter that is a reference gets its own lifetime parameter
 * 2. If there is exactly one input lifetime parameter, that lifetime
 *    is assigned to all output lifetitime parameters
 * 3. If there are multiple input lifetime parameters,
 *    but one of them is &self or &mut self the lifetime of the self
 *    is assinged to all ouput lifetime parameters
 */

mod combined;

struct ImportantExcerpt<'a> {
  part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
  fn return_part(&self, announcement: &str) -> &str {
    println!("Attention please: {}", announcement);
    self.part
  }
}

fn main() {
  // dangling references

  // let r;                  // ---------+-- 'a
  //                         //          |
  // {                       //          |
  //   let x = 5;            // -+-- 'b  |
  //   r = &x;               //  |       |
  // }                       // -+       |
  //                         //          |
  // println!("r: {}", r);   // ----------+

  // Correct referencing
  // let x = 5;                 // ----------+-- 'b
  //                            //           |
  // let r = &x;                // --+-- 'a  |
  //                            //   |       |
  // println!("r: {r}");        //   |       |
  //                            // --+       |
  //                            // ----------+

  let string1 = String::from("abcd");
  let string2 = String::from("xyz");

  let res = longest(string1.as_str(), string2.as_str());
  println!("The longest string is {}", res);

  // also correct
  {
    let string2 = String::from("xyz");
    let res = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", res);
  }

  // INCORRECT
  // let res;

  // {
  //   let string2 = String::from("xyz");
  //   res = longest(string1.as_str(), string2.as_str());
  //   // legal
  //   println!("{}", res)
  // }

  // // illegal
  // println!("{}", res)

  let novel = String::from("Call me Ishmael. Some years ago...");
  let first_sentence = novel.split('.').next().expect("Could not find");
  let i = ImportantExcerpt {
    part: first_sentence,
  };

  // let res;
  // {
  //   let i = ImportantExcerpt {
  //     part: first_sentence,
  //   };

  //   res = i.return_part("announcement");
  //   // allowed
  //   println!("{}", res);
  // }
  // // illegal
  // println!("{}", res);

  // Static lifetimes can live for as long as the duration of the program
  // all string literals have a static lifetime because string literals are stored in the program's
  // binary
  let s: &'static str = "I have a static lifetime.";
}

// &i32         // a reference
// &'a i32      // a reference with an explicit lifetime
// &'a mut i32  // a mutable reference with an explicit lifetime
// borrow checker wouldn't know how to handle the lifetimes of x and y

// Generic lifetime annotations dont actually change the lifetime
// they just create relationships between the lifetimes of multiple
// references e.g. from the below example, there's a relationship between
// x, y, and the return value e.g. the lifetime of the return reference,
// would be the same as the smallest lifetime of the arguments so if x has a smaller
// lifetime than y, then that would be the return
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() { x } else { y }
}
