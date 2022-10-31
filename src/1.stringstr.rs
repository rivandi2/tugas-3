//Soal 1. String dan &str
fn reverse(str: &str) -> String {
    str.chars().rev().collect()
}

fn main() {
    let before: &str = "rust";
    let after = reverse(&before);

    println!("{:?}", after);
}