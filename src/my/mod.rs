// Similarly `mod inaccessible` and `mod nested` will locate the `nested.rs`
// modules
mod inside;

pub fn function() {
  println!("called `my::function()`");
}

fn private_function() {
  println!("called `my::private_function()`");
}

pub fn indirect_access() {
  print!("called `my::indirect_access()`, that\n> ");
  private_function();
}
