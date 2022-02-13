fn main() {
  let a = "0.0.0.0";
  let port: i32 = 8080;
  let b = port.to_string();

  let v = std::env::var("PORT").expect("PORT is not set.");

  println!("Hello {}", v);

  let c = a.to_owned() + ":" + &v;
  println!("Result {}", c);
}
