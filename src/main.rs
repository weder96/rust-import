mod hello;
mod my;

fn function() {
    println!("called `function()`");
}

fn main() {
    println!("Hello, world!");
    hello::hello_test();
    function();
    my::indirect_access();
}
