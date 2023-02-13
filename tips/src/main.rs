use std::io::Error;

trait Karaoke {
    fn pick_song_i_know(&self, songs_i_know: Vec<String>, song_list: Vec<String>)
        -> Option<String>;

    fn sing(&self, song: String) -> Result<(), Error>;
}

struct Bubbles {}

impl Karaoke for Bubbles {
    fn pick_song_i_know(
        &self,
        songs_i_know: Vec<String>,
        song_list: Vec<String>,
    ) -> Option<String> {
        let mut picked_song = None;

        for song in song_list {
            if songs_i_know.contains(&song) {
                picked_song = Some(song);
                break;
            }
        }
        picked_song
    }

    fn sing(&self, song: String) -> Result<(), Error> {
        todo!();
    }
}

fn even_or_odd(x: i32) -> String {
    match x % 2 {
        0 => "even".to_string(),
        1 | -1 => "odd".to_string(),
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone)]
struct Invitation {
    invitee: String,
    attending: bool,
    message: Option<String>,
}

impl Invitation {
    fn new(invitee: String, attending: bool, message: Option<String>) -> Invitation {
        Invitation {
            invitee,
            attending,
            message,
        }
    }
}

fn commercials(hour: u32) -> String {
    match hour {
        0..=7 => "Classic video bundle commercials".to_string(),
        8 | 12 | 18 => "Food commercials".to_string(),
        9..=11 | 13..=17 => "Clothing commercials".to_string(),
        19..=24 => "Season ticket commercials".to_string(),
        _ => "NOT A VALID HOUR".to_string(),
    }
}

fn main() {
    let name = env!("NAME");
    let greeting  = option_env!("GREETING").expect("Missing GREETING");
    println!("{}", format!("{greeting}, {name}!"));
}
