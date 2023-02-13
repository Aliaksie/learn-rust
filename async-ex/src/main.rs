use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!");
}

#[derive(Debug, Clone, Copy)]
struct Song { }

impl Song {
    fn new() -> Song {
        Song{}
    }
}

async fn learn_and_sing() -> Song {
    let song = learn_song().await;
    sing_song(song).await;

    song
}

async fn learn_song() -> Song { 
    Song::new()
 }

async fn sing_song(song: Song) { /* ... */ }

async fn dance() { /* ... */ }

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    futures::join!(f1, f2);
}


fn main() {
    block_on(async_main());
}