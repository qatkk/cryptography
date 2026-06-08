mod exponent;

fn main() {
    println!("Hello, world!");
    let element = exponent::GroupElement {
        p: 7,
        g:3,
    };
    println!("the  exponent result is {:?} ", element.mod_inv());
}
