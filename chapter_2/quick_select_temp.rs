fn median3(v: &mut [usize], right: usize) -> usize {
    let center = right/2;
    if v[0] > v[center] {
        v.swap(0, center);
    }
    if v[0] > v[right] {
        v.swap(0, right);
    }
    if v[center] > v[right] {
        v.swap(center, right);
    }
    v.swap(center, right-1);
    v[right-1]
}

fn quick_select(v: &mut [usize]) {
    let l = v.len();
    let pivot = median3(v, l-1);
    let mut i = 0 as usize;
    let mut j = l-2;
    while i < j {
        while v[i] <= pivot && i < j {
            i += 1;
        }
        v[j] = v[i];
        while v[j] > pivot && i < j {
            j -= 1;
        }
        v[i] = v[j];
    }
    v[i] = pivot;
}
fn main() {
    let mut s =  vec![3, 99, 11, 7, 108, 22, 5];
    quick_select(&mut s);
    dbg!(s);
}

