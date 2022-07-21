fn main() {
    let s = "abc efg   hij";
    let v = s.split(' ')
    .filter(|w| { *w != ""} )
    .map(|w| { w.chars().rev().collect::<String>()})
    // .inspect(|w|{ println!("{}", w); })
    .collect::<Vec<_>>();
    let rs = v.join( " ");
    println!("{}", rs);
}