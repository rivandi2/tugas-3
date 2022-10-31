//Soal 2. Vector
fn check(v1: Vec<i32>) -> Vec<bool> {
    let mut v2 = Vec::new();
    
    for num in v1{
        v2.push(is_prime(num));
    }
    v2
}

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    for a in 2..n {
        if n % a == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let v1 = vec![8, 9, 10, 11, 12, 13, 19];
    let v2 = check(v1);
    for prim in v2{
        println!("{:?}", prim);
    }

}