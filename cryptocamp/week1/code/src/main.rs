mod exponent;

fn main() {
    println!("Hello, world!");
    let g = exponent::GroupElement {
        base: 5,
        value:3,
    };
    println!("the  exponent result is {:?} ", g.exponent(3));
}
