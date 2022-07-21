fn is_palindrome(s: String) -> bool {
    let l = s.len();
    for i in 0..l/2 {
        if s[i..(i+1)] != s[(l-(i+1))..(l-i)] {
            return false;
        } else {}
    }
    true
}

fn main() {
    println!("{}", is_palindrome(String::from("useresu")));
}