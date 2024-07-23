use std::io;

use super::custom_player::Player;

//Define a generic function t get user input 
fn collect_input<T: std::str::FromStr>(prompt: &str) -> T{
    loop{
        println!("{}", prompt);
        let mut buffer = String::new();

        io::stdin().read_line(&mut buffer).expect("Failled to read input");

        match buffer.trim().parse() {
            Ok(value) => return value,
            Err(_) => continue,
        }
    }
}

fn collect_player() -> Vec<Player>{
    let mut players=Vec::new();
    let mut num_players = 0;

    loop {
        num_players = collect_input::<u32>("How many players (>= 2)?");
        if num_players < 2{
            println!("Input no.! Please try again!");
            continue;
        }else{
            break;
        }
    }
    for i in 1..=num_players{
        let name = collect_input::<String>(format!("Player {} name:", i).as_str());
        players.push(Player { name, score: 0 });
    }

    players
}