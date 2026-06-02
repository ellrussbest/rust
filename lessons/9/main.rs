pub mod backtrace;
pub mod error_propagation;
use std::error::Error;
use std::fs::File;
use std::io::ErrorKind;

fn main() -> Result<(), Box<dyn Error>> {
  // panic macro:
  // if a program fails in a way that's unrecoverable or in a way that
  // the error can't be handled gracefully, then we can use the panic macro
  // which will immediately quit the program and print out the error message
  // panic!("Crash and burn");

  // this environment variable would list up all the functions called
  // leading up to the erring code
  // RUST_BACKTRACE=1
  // backtrace::main();

  let f = File::open("hello.txt");

  let f = match f {
    Err(e) => match e.kind() {
      ErrorKind::NotFound => match File::create("hello.txt") {
        Ok(fc) => fc,
        Err(e) => panic!("problem creating the file: {:?}", e),
      },
      _ => panic!("Problem opening the file: {:?}", e),
    },
    Ok(f) => f,
  };

  // closure snippet
  let f = File::open("hello.txt").unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
      File::create("hello.txt").unwrap_or_else(|error| {
        panic!("Problem creating the file: {:?}", error);
      })
    } else {
      panic!("Problem opening the file {:?}", error);
    }
  });

  // unwrap would do the same thing as our match expression e.g. returns file or panics e.g.
  // an equivalent of the following
  //   let f = match f {
  //       Ok(file) => file,
  //       Err(e) => panic!("{:?}", e)
  //   };
  let f = File::open("hello.txt").unwrap();

  // pass a custom error message
  let f = File::open("hello.txt").expect("Failed to open hello.txt");

  let f = File::open("hello.txt")?;

  Ok(())
}
