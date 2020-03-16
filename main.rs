use bit_vec::BitVec;
use lazy_static::lazy_static;

const N: usize = 1000;

lazy_static! {
    // FIRST_SQUARE[x] = minimal i such that (i*i - x) > 0.
    static ref FIRST_SQUARE: [i16; N+1] = {
        let mut first_square = [0; N+1];
        let mut s: i16 = 2;
        for x in 1..N+1 {
            if s * s - (x as i16) < 1 {
                s += 1;
            }
            first_square[x] = s;
        }
        first_square
    };

    // Allows to quickly check if a number is a perfect square.
    static ref IS_SQUARE: BitVec = {
        let mut is_square = BitVec::from_elem(2*N, false);
        let mut x = 2;
        loop {
            if x*x > 2*N-1 {
                break;
            }
            is_square.set(x*x, true);
            x += 1;
        }
        is_square
    };
}

fn main() {
    let mut result: [i16; N + 1] = [0; N + 1];
    let mut used_numbers = BitVec::from_elem(N + 1, false);
    for n in 25..=N {
        solve(n, &mut result, &mut used_numbers);
        println!("{} {} {:?}\t", n, check(n, &result), &result[1..=n]);
        let (a, b) = find_place(n+1, &result);
        let extracted = insert((n+1) as i16, n, &mut result, a, b);
        println!("{:?} {}", &extracted, check(n+1, &result));
    }
}

fn solve(n: usize, result: &mut [i16], used_numbers: &mut BitVec) -> bool {
    result[1] = 1;
    used_numbers.clear();
    used_numbers.set(1, true);
    let mut i = 1;
    let mut x = result[i] as usize;
    let mut p = 0; // x before backtracking
    loop {
        let mut s = FIRST_SQUARE[x] as usize;
        'inner: loop {
            let y = s * s - x;
            if y > n {
                // backtrack
                if i == 0 {
                    return false;
                }
                used_numbers.set(x, false);
                i -= 1;
                p = x;
                x = result[i] as usize;
                break 'inner;
            } else if y > p && !used_numbers.get(y).unwrap() {
                used_numbers.set(y, true);
                i += 1;
                result[i] = y as i16;
                if i == n {
                    return true;
                }
                p = 0;
                x = y;
                break 'inner;
            }
            s += 1;
        }
    }
}

fn check(n: usize, xs: &[i16]) -> bool {
    let mut used_numbers = BitVec::from_elem(n + 1, false);
    used_numbers.set(0, true);
    for i in 1..n {
        let s = (xs[i] + xs[i + 1]) as usize;
        if !is_square(s) {
            println!("{} {}", xs[i], xs[i+1]);
            return false;
        }
        used_numbers.set(xs[i] as usize, true);
    }
    used_numbers.set(xs[n] as usize, true);
    used_numbers.all()
}

fn find_place(n: usize, xs: &[i16]) -> (usize, usize) {
    if is_square(xs[1] as usize + n) {
        return (0, 1)
    }
    if is_square(xs[n-1] as usize + n) {
        return (n-1, n)
    }

    let mut start = 0;
    let mut end = 0;
    let mut prev = 0;

    for i in 1..n-1 {
        let a = xs[i] as usize;
        let b = xs[i+1] as usize;
        if is_square(a + n) {
            if is_square(b + n) {
                return (i, i+1)
            }
            else if prev == 0 {
                start = i;
            } else if end == 0 {
                end = i;
            } else {
                if end - start > i - prev {
                    start = prev;
                    end = i;
                }
            }
            prev = i;
        }
    }
    (start, end)
}

fn insert(x: i16, n: usize, xs: &mut [i16], a: usize, b: usize) -> Vec<i16> {
    let res = Vec::from(&xs[a+1..b]);
    if a+1 == b {
        let mut tmp1 = x;
        if n >= b {
            for i in 0..=(n-b) {
                let tmp2 = xs[b + i];
                xs[b + i] = tmp1;
                tmp1 = tmp2;
            }
        }
        xs[n+1] = tmp1;
    } else {
        xs[a+1] = x;
        for i in 0..=(n-b) {
            xs[a+i+2] = xs[b+i];
        }
    }
    res
}

fn is_square(n: usize) -> bool {
    n < N && IS_SQUARE.get(n).unwrap()
}
