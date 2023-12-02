use std::fs;

fn main() {
    let input =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_lines = input.lines();

    println!("Files Lines {}", input_lines.clone().count());

    let mut sum = 0;

    for line in input_lines {
        let lin_game = string_to_game(line).unwrap_or(Game {
            id: 0,
            sets: vec![],
        });
        let lin_game_t = string_to_game(line).unwrap_or(Game {
            id: 0,
            sets: vec![],
        });

        let mut impossible = false;

        for set in lin_game.sets {
            if set.blue > 14 || set.green > 13 || set.red > 12 {
                impossible = true;
            }
        }

        if !impossible {
            sum = sum + lin_game.id;
        }

        println!(
            "Line '{}'\n'{:?}'\nimpossible:{}\n",
            line, lin_game_t, impossible
        );
    }

    println!("Sum of valid: {}", sum);
}

#[derive(Debug)]
pub struct Game {
    id: i32,
    sets: Vec<Sets>,
}
#[derive(Debug)]
pub struct Sets {
    red: i32,
    green: i32,
    blue: i32,
}

fn string_to_game(text: &str) -> Option<Game> {
    let parts: Vec<&str> = text.split(": ").collect();

    if parts.len() == 2 {
        let id: i32 = parts[0]
            .trim()
            .strip_prefix("Game ")
            .and_then(|s| s.parse().ok())
            .unwrap_or_default();

        let sets = parts[1].trim();
        let sets_part = sets.split(';');

        let mut sets: Vec<Sets> = vec![];

        for part in sets_part {
            let color_parts: std::str::Split<'_, &str> = part.trim().split(", ");
            let set=strings_to_set(color_parts);

            sets.push(set);
        }

        Some(Game {
            id,
            sets,
        })
    }else {
        None
    }

}

fn strings_to_set(set_str:std::str::Split<'_, &str>) -> Sets {
    let mut set=Sets{
        blue:0,
        green:0,
        red:0
    };
    for part in set_str {
        let parsed:Vec<&str>=part.trim().split(' ').collect();
        if parsed.len()==2{
            let number:i32=parsed[0].trim().parse().unwrap_or_default();  
            match parsed[1].trim() {
                "blue"=>set.blue=number,
                "red"=>set.red=number,
                "green"=>set.green=number,
                _=>{}
            }
        }
    }
    set
}
