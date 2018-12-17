extern crate core;

use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

const PLAYERS: usize = 9;
const TURNS: usize = 25;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
struct Node {
    points: usize,
    right: Link,
    left: Link,
}

fn main() -> Result<(), Box<Error>> {
    let mut current = Rc::new(RefCell::new(Node {
        points: 0,
        right: None,
        left: None,
    }));

    let mut scores = [0; PLAYERS];

    (*current).borrow_mut().right = Some(current.clone());
    (*current).borrow_mut().left = Some(current.clone());

    for i in 1..TURNS + 1 {
        if i % 23 != 0 {
            let moving = (*current).borrow().right.as_ref().unwrap().clone();
            current = moving;
            let next = (*current).borrow().right.as_ref().unwrap().clone();

            let mut new = Rc::new(RefCell::new(Node {
                points: i,
                right: Some(next.clone()),
                left: Some(current.clone()),
            }));

            (*current).borrow_mut().right = Some(new.clone());
            (*next).borrow_mut().left = Some(new.clone());

            current = new;
        } else {
            let player = i % PLAYERS;
            scores[player] += i;

            for _ in 0..6 {
                let moving = (*current).borrow().left.as_ref().unwrap().clone();
                current = moving;
            }

            let to_remove = (*current).borrow().left.as_ref().unwrap().clone();
            let new_left = (*to_remove).borrow().left.as_ref().unwrap().clone();

            scores[player] += (*to_remove).borrow().points;

            (*new_left).borrow_mut().right = Some(current.clone());
            (*current).borrow_mut().left = Some(new_left.clone());
        }
    }

    let mut high_score = 0;

    for score in scores.iter() {
        if *score > high_score {
            high_score = *score;
        }
    }

    println!("{}", high_score);

    Ok(())
}
