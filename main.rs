use bit_vec::BitVec;

const N: usize = 1000;

fn main() {
    let mut first_square: [i16; N + 1] = [0; N + 1];
    let mut s: i16 = 2;
    for x in 1..N+1 {
        if s * s - (x as i16) < 1 {
            s += 1;
        }
        first_square[x] = s;
    }

    let mut result: [i16; N + 1] = [0; N + 1];
    let mut used_numbers = BitVec::from_elem(N+1, false);
    for n in 2..N {
        let a = result[1..n-1].to_vec();
        let ok = solve(n, &first_square, &mut result, &mut used_numbers);
        let b = result[1..n].to_vec();
        compare(a, b);
        // println!("{} {} {} {}", ok, n, result[1], result[n]);
    }
}

fn compare(a: Vec<i16>, b: Vec<i16>) {
    print!("{}\t", b.len());
    for i in 0..a.len() {
        if a[i] == b[i] {
            print!("+");
        } else if b[i] as usize == b.len() {
            print!("O");
        } else if a[i] == b[i+1] {
            print!("-");
        } else {
            print!(" ");
        }
    }
    print!("!\n");
}

fn solve(n: usize, first_square: &[i16], result: &mut [i16], used_numbers: &mut BitVec) -> bool {
    result[1] = 1;
    used_numbers.clear();
    used_numbers.set(1, true);
    let mut i = 1;
    let mut x = result[i] as usize;
    let mut p = 0; // x before backtracking
    'outer: loop {
        let mut s = first_square[x] as usize;
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
