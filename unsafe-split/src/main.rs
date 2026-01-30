use unsafe_split::split_at_mut;

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let (a, b) = split_at_mut(&mut v, 3);

    println!("First slice:  {:?}", a);
    println!("Second slice: {:?}", b);
}

