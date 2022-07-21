fn main() {
    let sen = "I am a student.";
    let t:String = sen.split(" ").map(|s|s.chars().rev().collect())
        .collect::<Vec<String>>().join(" ")
        .chars().rev().collect();
}