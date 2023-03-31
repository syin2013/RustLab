use rand::thread_rng;

/// The Representation of the hide and seek game
struct GameState {
    // TODO: make this 2d
    map: Vec<GridOccupant>,
}

impl GameState {
    // QUIZ: How can you generate cargo docs and view their website
    /// Initializes the GameState
    pub fn new() -> Self {
        GameState { map: vec![] }
    }

    /// Generates a coordinate on the map
    fn generate_random_coordinate() -> Coord {
        // This wont compile! What do I do!
        todo!()
    }
}

/// What can occupy a space in the grid.
enum GridOccupant {
    Player,
    Tree,
}


fn main() {

}

#[cfg(test)]
mod test {
    use super::*;
    /// Tests that a grid is valid. IE it contains trees, you, and your sister.
    #[test]
    fn test_valid_grid() {}
}
