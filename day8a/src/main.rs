use std::error::Error;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), Box<Error>> {
    let stdin = io::stdin();

    let tree : Vec<u32> = stdin.lock().split(b' ').map(|c| String::from_utf8(c.unwrap()).unwrap().parse().unwrap()).collect();

    let mut idx = 0;
    let mut sum = 0;

    walk_tree(&tree, &mut idx, &mut sum);

    println!("{}", sum);

    Ok(())
}

fn walk_tree(tree: &Vec<u32>, idx: &mut usize, sum: &mut u32) {
    let children = tree[*idx];
    *idx += 1;
    let metas = tree[*idx];
    *idx += 1;
    for _ in 0..children {
        walk_tree(tree, idx, sum);
    }

    for _ in 0..metas {
        *sum += tree[*idx];
        *idx += 1;
    }
}
