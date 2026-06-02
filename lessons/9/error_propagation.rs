// often times, when you have a function whose implementation
// calls something that could fail, you want to return that error back
// to the caller instead of handling it within the function
// this gives more control to the caller who can decided what to do with the error
// this is what we call error propagation

use std::{
  fs::File,
  io::{self, Read},
};

pub fn read_username_from_file() -> Result<String, io::Error> {
  // if the file doesn't exist, our function will end early and return the error
  // with the ? syntax equivalent of the below
  // let mut f = match f {
  //     Ok(f) => f,
  //     Err(e) => return Err(e)
  // };

  // the question mark operator is only used in a function that returns a Result or an Option
  let mut s = String::new();
  File::open("hello.txt")?.read_to_string(&mut s)?;

  Ok(s)
}
