#[macro_export]
macro_rules! log {
  ( $( $x:expr ),* ) => {
    {
      $(
        print!("{}", $x);
      )*
      println!("");
    }
  };
}