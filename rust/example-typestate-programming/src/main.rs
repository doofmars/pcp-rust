use crate::states::{Moving, Stopped};

mod states;

fn main() {
    let car_state: Stopped = Stopped::new();
    println!("Starting at {} m", car_state.get_distance());

    // transition to Moving
    let new_car_state: Moving = car_state.accelerate(
        5, // acceleration in m/s^2
        2 // how long to accelerate in seconds
    );
    println!("Accelerated to {} m/s", new_car_state.get_velocity());

    // transition back to Stopped (but with a new Distance)
    let final_car_state: Stopped = new_car_state.stop_after(
        10 // after how many seconds to stop moving
    );
    println!("Reached destination at {} m", final_car_state.get_distance());

    // to add more distance you can transition to Moving again

    /*
    // you can only start at 0, because the distance is private

    let invalid_state = Stopped {
        distance: 10,
    };
     */

    /*
    // you also can't start with a Moving state

    let invalid_state = Moving {
        initial_distance: 10,
        velocity: 5,
    };
     */
}

#[test]
fn test() {
    let final_destination = Stopped::new()
        .accelerate(5, 2)
        .stop_after(10)
        .get_distance();

    assert_eq!(
        final_destination,
        100
    )
}
