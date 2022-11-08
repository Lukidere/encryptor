use crate::lib::key::encrypt;
pub mod lib;
fn main() {
    let z:String = encrypt(1000);
    println!("{z}");
}
