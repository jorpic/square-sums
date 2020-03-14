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
        println!("{} {} {:?}\t", n, check(n, &result), &result[1..15]);
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
        if !IS_SQUARE.get(s).unwrap() {
            return false;
        }
        used_numbers.set(xs[i] as usize, true);
    }
    used_numbers.set(xs[n] as usize, true);
    used_numbers.all()
}
