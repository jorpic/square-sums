use lazy_static::lazy_static;
use bit_vec::BitVec;

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
}

fn main() {

    let mut result: [i16; N + 1] = [0; N + 1];
    let mut used_numbers = BitVec::from_elem(N+1, false);
    for n in 25..N {
        solve(n, &mut result, &mut used_numbers);
        println!("{} {:?}\t", n, &result[1..15]);
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
