mod challenges;

fn main() {
    let mut hand = challenges::black_jack::Hand::new();
    let mut dealer = challenges::black_jack::Dealer::new();
    let mut desk  = dealer.get_desk();
    
    // ..
    hand.add(desk.pop().unwrap());
    hand.add(desk.pop().unwrap());
    hand.add(desk.pop().unwrap());


    let msg = if hand.is_loosing() { "Lose" } else { "Won" }; 
    println!("My card {:?}", hand.get_card());
    println!("You are {}!", msg);
}
