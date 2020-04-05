extern crate rand;

use rand::Rng;

const BAKUGO_MAX_BLASTS: i8 = 36;
const MIDORIYA_MAX_HEALTH: u8 = 3;
const TODOROKI_FREEZE_DECREMENT: u8 = 3;
const FOREST_WIDTH: u8 = 10;
const FOREST_HEIGHT: u8 = 10;

#[derive(Copy, Clone)]
struct Coordinate(u8, u8);

impl Coordinate {
    fn new(x: u8, y: u8) -> Coordinate {
        return Coordinate(x, y);
    }

    #[doc = "Generates a coordinate pair within 0 -> max_x, 0 -> max_y"]
    fn new_rand(max_x: u8, max_y: u8) -> Coordinate {
        return Coordinate::new(
            rand::thread_rng().gen_range(0, max_x - 1),
            rand::thread_rng().gen_range(0, max_y - 1),
        );
    }

    fn eq(&self, coord: &Coordinate) -> bool {
        return self.0 == coord.0 && self.1 == coord.1;
    }
}

#[test]
fn test_coordinate_equivalence() {
    let coord_a: Coordinate = Coordinate::new(3, 3);
    let coord_b: Coordinate = Coordinate::new(3, 3);
    let coord_c: Coordinate = Coordinate::new(4, 5);

    // coordinates should equal coorindates of the same place
    assert_eq!(true, coord_a.eq(&coord_b)); // coordinates should not equal coordinates at different places
    assert_ne!(true, coord_a.eq(&coord_c));
    // coordinates should equal themselves
    assert_eq!(true, coord_a.eq(&coord_a));
    // assert copies are equivalent
    assert_eq!(true, coord_a.eq(&coord_a.clone()));
}

#[test]
fn test_create_coordinate() {
    // It's kinda weird to test a function with randoms in it, but I can try to test ranges.
    let coordinate: Coordinate = Coordinate::new_rand(10, 10);
    assert!(coordinate.0 < 10);
    assert!(coordinate.1 < 10);
}

#[doc = "Generates a new location for midoriya to jump to"]
fn midoriya_jump(
    midoriya_current_location: Coordinate,
    todoroki_location: Coordinate,
) -> Coordinate {
    let mut midoriya_new_location: Coordinate = todoroki_location;

    while midoriya_new_location.eq(&todoroki_location)
        || midoriya_new_location.eq(&midoriya_current_location)
    {
        midoriya_new_location = Coordinate::new_rand(FOREST_WIDTH, FOREST_HEIGHT);
    }

    return midoriya_new_location;
}

#[test]
fn test_midoriya_jump() {
    let midoriya_location: Coordinate = Coordinate::new_rand(FOREST_WIDTH, FOREST_HEIGHT);
    let todoroki_location: Coordinate = Coordinate::new_rand(FOREST_WIDTH, FOREST_HEIGHT);
    let midoriya_location_new: Coordinate = midoriya_jump(midoriya_location, todoroki_location);

    // just make sure that the new location is not the same as the old one
    assert_eq!(false, midoriya_location_new.eq(&midoriya_location));
}

// come back to the lifetime explanation on the return type there
// This string should not live that long
fn determine_coord_label(
    coord: &Coordinate,
    todoroki_location: &Coordinate,
    midoriya_location: &Coordinate,
) -> String {
    if coord.eq(todoroki_location) {
        return "T".to_string();
    } else if coord.eq(midoriya_location) {
        return "M".to_string();
    }
    return "_".to_string();
}

#[test]
fn test_determine_coord_label() {
    let todoroki_location: Coordinate = Coordinate::new(1, 2);
    let midoriya_location: Coordinate = Coordinate::new(1, 3);

    assert_eq!(
        "T",
        determine_coord_label(
            &todoroki_location.clone(),
            &todoroki_location,
            &midoriya_location
        )
    );
    assert_eq!(
        "M",
        determine_coord_label(
            &midoriya_location.clone(),
            &todoroki_location,
            &midoriya_location
        )
    );
    assert_eq!(
        "_",
        determine_coord_label(
            &Coordinate::new(1, 1),
            &todoroki_location,
            &midoriya_location
        )
    );
}

fn print_forest(max_depth: u8, max_width: u8, todoroki_location: Coordinate, midoriya_location: Coordinate) {
    for y in 0..max_depth {
        for x in 0..max_width {
            print!("{} ", determine_coord_label(&Coordinate::new(x, y), &todoroki_location, &midoriya_location));
        }
        println!("");
    }
}

fn main() {
    let todoroki_location: Coordinate = Coordinate::new_rand(FOREST_WIDTH, FOREST_HEIGHT);
    let mut midoriya_current_location: Coordinate = todoroki_location.clone();

    while midoriya_current_location.eq(&todoroki_location) {
        midoriya_current_location = Coordinate::new_rand(FOREST_WIDTH, FOREST_HEIGHT);
    }

    println!("midoriya starts at {}, {}", midoriya_current_location.0, midoriya_current_location.1);

    let mut midoriya_current_health = MIDORIYA_MAX_HEALTH;
    let mut bakugo_blasts_left = BAKUGO_MAX_BLASTS;

    while midoriya_current_health > 0 && bakugo_blasts_left > 0 {
        let bakugo_attack_target: Coordinate = Coordinate::new_rand(FOREST_WIDTH, FOREST_HEIGHT);
        println!("");
        print_forest(FOREST_HEIGHT, FOREST_WIDTH, todoroki_location, midoriya_current_location);
        println!("Bakugo is attacking ({}, {})!", bakugo_attack_target.0, bakugo_attack_target.1);

        if bakugo_attack_target.eq(&midoriya_current_location) {
            midoriya_current_health -= 1;
            midoriya_current_location = midoriya_jump(midoriya_current_location, todoroki_location);
            println!("Bakugo hit Midoriya");
        } else if bakugo_attack_target.eq(&todoroki_location) {
            bakugo_blasts_left -= TODOROKI_FREEZE_DECREMENT as i8;
            println!("Bakugo hit Todoroki");
        } else {
            println!("Bakuko missed completely!");
        }

        bakugo_blasts_left -= 1;
        println!("Midoriya health: {}", midoriya_current_health);
        println!("Bakugo blasts left: {}", bakugo_blasts_left);
    }

    if midoriya_current_health == 0 {
        println!("Bakugo is the winner!");
    } else {
        println!("Midoriya is the winner!");
    }

}
