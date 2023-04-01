use rand::Rng;
use std::fmt;

/*
#[macro_use]
extern crate clap;
use clap::{App, Arg};
*/

/// The Representation of the hide and seek game
/// #[derive(Debug)]
struct GameState {    
    map: Vec<Vec<GridOccupant>>,
    row: usize,
    col: usize,
    sister_coord: Coord,
    your_coord: Coord,
    blast: i8,
    allow_hit: u8,
    current_hit: u8
}

#[derive(Eq, PartialEq, Debug)]
struct Coord {
    x: u32,
    y: u32,
}

/// What can occupy a space in the grid.
#[derive(Clone)]
enum GridOccupant {
    PlayerYou,
    PlayerYourSister,
    Tree,
}

impl fmt::Display for GridOccupant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GridOccupant::PlayerYou => write!(f, "Y "),
            GridOccupant::PlayerYourSister => write!(f, "S "),
            GridOccupant::Tree => write!(f, "_ "),
        }
    }
}

impl GameState {
    // QUIZ: How can you generate cargo docs and view their website
    /// Initializes the GameState
    pub fn new(row : usize, col : usize, blast: i8, allow_hit: u8) -> Self {
        GameState { 
            map: vec![vec![GridOccupant::Tree; row]; col],
            row,
            col,
            sister_coord: Coord {x : rand::thread_rng().gen_range(0..row as u32), y : rand::thread_rng().gen_range(0..col as u32)},
            your_coord: Coord{ x : 0, y : 0},
            blast,
            allow_hit,
            current_hit : 0,
        }
    }

    /// Generates a coordinate on the map
    fn generate_random_coordinate(&self) -> Coord {
        Coord {
            x : rand::thread_rng().gen_range(0..self.row as u32),
            y : rand::thread_rng().gen_range(0..self.col as u32),
        }
    }

    fn set_sister_coord(&mut self) {
        self.map[self.sister_coord.x as usize][self.sister_coord.y as usize] = GridOccupant::PlayerYourSister;
    }

    fn you_move(&mut self) {
        let mut your_coord = self.generate_random_coordinate();
        while your_coord == self.sister_coord{
            your_coord = self.generate_random_coordinate();
        }
        self.set_your_coord(your_coord);
    }

    fn set_your_coord(&mut self, your_coord: Coord) {
        self.map[self.your_coord.x as usize][self.your_coord.y as usize] = GridOccupant::Tree;
        self.your_coord = your_coord;
        self.map[self.your_coord.x as usize][self.your_coord.y as usize] = GridOccupant::PlayerYou;
    }

    fn attack(&mut self) {
        let guess_coord = self.generate_random_coordinate();
        println!("Your brother is attcking {:?}", guess_coord);
        if guess_coord == self.your_coord {
            println!("Your brother hit you!");
            self.current_hit += 1;
        } else if guess_coord == self.sister_coord {
            println!("Your brother hit your sister!");
        } else {
            println!("Your brother missed completely!");
        }
        println!("You are hit {:?}, your brother left blast {:?}", self.current_hit, self.blast);
    }

    fn dump_map(&mut self) {
        for row in self.map.iter(){
            for col in row.iter() {
                print!("{} ", col);
            }
            println!("");
        }
        
        /*    self.map.iter().for_each(|it| {
                println!("{:?}", it);
            }
        
        )*/
    }
}

fn main() {
/*
    let matches = App::new("My program")
        .arg(Arg:: with_name("row").required(true))
        .arg(Arg::with_name("col").required(true))
        .get_matches();

    let row = value_t!(matches, "row", i32).unwrap();
    let col = value_t!(matches, "col", i32).unwrap();
 */

    let mut game = GameState::new(3, 3, 5, 3);
    game.set_sister_coord();
    
    while game.blast >= 0 {
        game.you_move();

        game.dump_map();
        game.attack();
        if game.current_hit == game.allow_hit {
            break;
        }
        game.blast -= 1;
    }
    
    if game.current_hit == game.allow_hit {
        println!("Your brother win!");
    } else {
        println!("You win!");
    }
}

#[cfg(test)]
mod test {
    use super::*;
    /// Tests that a grid is valid. IE it contains trees, you, and your sister.
    #[test]
    fn test_valid_grid() {}
}
