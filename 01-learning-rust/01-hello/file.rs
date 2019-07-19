fn main() {
    println!("Hello world!");

    println!("{}, {}", "Hello", "foo");

    println!("{1}, {0}", "Hello", "foo");

    println!("{greeting}, {name}", greeting="Hola", name="foo");

    println!("{:?}", [1,2,3]);
    println!("{:#?}", [1,2,3]);

    let x = format!("{}, {}!", "Hello", "foo");
    println!("{}", x);
}
