pub fn nth_ugly_number(n: i32) -> i32 {
    let n = n as usize;
    let mut ugly = vec![1; n];
    let mut i2 = 0;
    let mut i3 = 0;
    let mut i5 = 0;

    for i in 1..n {
        let next2 = ugly[i2] * 2;
        let next3 = ugly[i3] * 3;
        let next5 = ugly[i5] * 5;
        let next = next2.min(next3).min(next5);
        ugly[i] = next;

        if next == next2 {
            i2 += 1;
        }
        if next == next3 {
            i3 += 1;
        }
        if next == next5 {
            i5 += 1;
        }
    }

    ugly[n - 1]
}
