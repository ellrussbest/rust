#[derive(Debug)]
struct Point<T> {
  x: T,
  y: T,
}

impl<T> Point<T> {
  fn new(x: T, y: T) -> Self {
    Self { x, y }
  }

  fn x(&self) -> &T {
    &self.x
  }
}

// impl block for only types of float64
impl Point<f64> {
  fn y(&self) -> f64 {
    self.y
  }
}

enum Option<T> {
  Some(T),
  None,
}

enum Result<T, E> {
  Ok(T),
  Err(E),
}

fn main() {
  let number_list = vec![34, 50, 25, 100, 65];
  let largest = get_largest(number_list);
  println!("The largest number is {}", largest);

  let char_list = vec!['y', 'm', 'a', 'q'];
  let largest = get_largest(char_list);
  println!("The largest char is {}", largest);

  let p1 = Point { x: 5, y: 10 };
  println!("The point is {:?}", p1);
  //   println!("{}", p1.y()); // illegal, method y not available for p1

  let p1 = Point { x: 5.5, y: 0.0 };
  println!("{}", p1.y());
}

// add trait to our type saying that our type T should be Ordeable & Copyable
fn get_largest<T: PartialOrd + Copy>(lst: Vec<T>) -> T {
  let mut largest = lst[0];

  for el in lst {
    if el > largest {
      largest = el;
    }
  }

  return largest;
}
