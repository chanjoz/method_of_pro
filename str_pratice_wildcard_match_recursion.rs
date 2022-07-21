use std::io;
// // use regex::Regex;

fn wildcard_match(s: &str, p: &str) -> bool {
    if p.is_empty() {
        return s.is_empty();
    }
    if s.is_empty(){
        if &p[0..] == "*" {
            return true;
        } else {
            return false;
        }
    }
    if &p[0..1] == "*" {
        wildcard_match(s, &p[1..]) || wildcard_match(&s[1..], p)
    } else {
        (p[0..1] == s[0..1] || &p[0..1] == "?") && wildcard_match(&s[1..], &p[1..])
    }
}
fn main() {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string)
    .expect("failed to read standard input!");
    let si = &input_string[0..input_string.len()-1];
    let pat = "bea*msn?ow";

    assert!(wildcard_match(si, pat));
    // let re = Regex::new(r"beam\w*sn\w?ow").unwrap();
    // assert!(re.is_match(&input_string));
}