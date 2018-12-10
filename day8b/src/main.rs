use std::error::Error;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), Box<Error>> {
    let stdin = io::stdin();

    let tree : Vec<u32> = stdin.lock().split(b' ').map(|c| String::from_utf8(c.unwrap()).unwrap().parse().unwrap()).collect();

    let mut idx = 0;

    println!("{}", node_value(&tree, &mut idx));

    Ok(())
}

fn node_value(tree: &Vec<u32>, idx: &mut usize) -> u32 {
    let children = tree[*idx];
    *idx += 1;
    let metas = tree[*idx];
    *idx += 1;

    if children > 0 {
        let mut child_values = Vec::new();

        for _ in 0..children {
            child_values.push(node_value(tree, idx));
        }

        let mut sum = 0;

        for _ in 0..metas {
            let child = tree[*idx];
            *idx += 1;

            if child > 0 && child <= children {
                sum += child_values[child as usize - 1];
            }
        }

        return sum;
    } else {
        let mut sum = 0;

        for _ in 0..metas {
            sum += tree[*idx];
            *idx += 1;
        }
        return sum;
    }
}
