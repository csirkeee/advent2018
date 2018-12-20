const PATTERN: usize = 51589;
const PATTERN_LEN: usize = 5;

fn main() {
    let mut recipes = vec![3, 7];

    let mut idx_a = 0;
    let mut idx_b = 1;

    let pos;

    'tick: loop {
        let val = recipes[idx_a] + recipes[idx_b];

        if val >= 10 {
            recipes.push(1);
        }
        recipes.push(val % 10);

        let len = recipes.len();

        for which in 0..2 {
            if len > which + PATTERN_LEN {
                let mut end = 0;
                for i in len - which - PATTERN_LEN..len - which {
                    end = end * 10 + recipes[i];
                }
                if end == PATTERN {
                    pos = len - which - PATTERN_LEN;
                    break 'tick;
                }
            }
        }

        idx_a = (idx_a + 1 + recipes[idx_a]) % len;
        idx_b = (idx_b + 1 + recipes[idx_b]) % len;
    }

    println!("{}", pos);
}
