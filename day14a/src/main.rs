const BAD_RECIPES: usize = 846021;

fn main() {
    let mut recipes = vec![3, 7];

    let mut idx_a = 0;
    let mut idx_b = 1;

    loop {
        let val = recipes[idx_a] + recipes[idx_b];

        if val >= 10 {
            recipes.push(1);
        }
        recipes.push(val % 10);

        let len = recipes.len();

        if len > BAD_RECIPES + 10 {
            break;
        }

        idx_a = (idx_a + 1 + recipes[idx_a]) % len;
        idx_b = (idx_b + 1 + recipes[idx_b]) % len;
    }

    for i in BAD_RECIPES..BAD_RECIPES + 10 {
        print!("{}", recipes[i]);
    }

    println!();
}
