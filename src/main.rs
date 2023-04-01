mod data;
use data::Data;
fn main() {
    let mut d = Data::new();
    d.add_term("Test".to_string());
    dbg!(d);
}
