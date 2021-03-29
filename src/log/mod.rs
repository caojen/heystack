#[macro_export]
macro_rules! log {
  ( $( $x:expr ),* ) => {
    {
      $(
        print!("{}", $x);
      )*
    }
  };
}

#[macro_export]
macro_rules! logln {
  ( $( $x:expr ),* ) => {
    {
      $(
        print!("{}", $x);
      )*
      println!("");
    }
  };
}