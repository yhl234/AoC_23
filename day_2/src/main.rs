use std::fs::read_to_string;
fn main() {
    let file_path = "./src/input.txt";
    let input = read_to_string(file_path).unwrap();
    let bag = Game::new(vec!["12 red", "13 green", "14 blue"]);
    let records: Vec<(u32, Vec<Game>)> = input
        .lines()
        .map(|l| {
            let records: Vec<&str> = l.split(": ").collect();
            let id = records[0].trim_start_matches("Game ").parse().unwrap();
            let games: Vec<Game> = records[1]
                .split(&[',', ';'][..])
                .filter(|p| !p.is_empty())
                .map(|g| Game::new(vec![g]))
                .collect();

            (id, games)
        })
        .collect();
    // let ids = records.iter().fold(0, |mut acc, curr| {
    //     let (id, games) = curr;
    //     let imposable = games.iter().find(|game| !bag.passable(game));
    //     if imposable.is_none() {
    //         acc += id;
    //     }
    //     acc
    // });
    let ids: u32 = records
        .iter()
        .filter_map(|(id, games)| games.iter().find(|game| !bag.passable(game)).map(|_| *id))
        .sum();
    println!("step 1: {:?}", ids);

    let power = records.iter().fold(0, |mut acc, curr| {
        let (_, games) = curr;
        let minimum = games.iter().fold(
            Game {
                red: 0,
                blue: 0,
                green: 0,
            },
            |mut acc_game: Game, current_game| {
                acc_game = acc_game.minimum(current_game);
                acc_game
            },
        );
        acc += minimum.blue * minimum.green * minimum.red;
        acc
    });

    println!("step 2: {:?}", power);
}

#[derive(Debug)]
struct Game {
    red: u32,
    blue: u32,
    green: u32,
}

impl Game {
    fn new(records: Vec<&str>) -> Game {
        let mut game = Game {
            red: 0,
            blue: 0,
            green: 0,
        };
        for i in records {
            let record: Vec<&str> = i.trim().split(' ').collect();
            let count = record[0];
            let color = record[1];
            match color {
                "red" => game.red += count.parse::<u32>().unwrap(),
                "blue" => game.blue += count.parse::<u32>().unwrap(),
                "green" => game.green += count.parse::<u32>().unwrap(),
                _ => continue,
            }
        }
        game
    }
    fn passable(&self, game: &Game) -> bool {
        self.blue >= game.blue && self.red >= game.red && self.green >= game.green
    }
    fn minimum(&self, game: &Game) -> Game {
        Game {
            red: std::cmp::max(self.red, game.red),
            blue: std::cmp::max(self.blue, game.blue),
            green: std::cmp::max(self.green, game.green),
        }
    }
}
