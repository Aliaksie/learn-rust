mod challenges;

fn main() {
    let input = vec![2, 1, 1];
    let answer = challenges::find::unique(input);

    println!("unique items -> {:?}", answer)
}
