// state stopped
pub struct Stopped {
    // private field to disallow external manipulation
    distance: u32,
}
impl Stopped {
    // associated function to create an instance of Stopped
    pub fn new() -> Self {
        Self {
            // the distance always starts at 0
            distance: 0,
        }
    }

    // public getter for the distance
    pub fn get_distance(&self) -> u32 {
        self.distance
    }

    // accelerate to transition to the state "Moving"
    pub fn accelerate(&self, acceleration: u32, seconds: u32) -> Moving {
        Moving {
            velocity: acceleration * seconds,
            initial_distance: self.distance,
        }
    }
}


// state moving
pub struct Moving {
    // private fields
    initial_distance: u32,
    velocity: u32,
}

impl Moving {
    // no new() function, an instance can only be created by transitioning from the state Stopped

    // public getter for the velocity
    pub fn get_velocity(&self) -> u32 {
        self.velocity
    }

    // stop_after x seconds to transition back to Stopped
    pub fn stop_after(&self, seconds: u32) -> Stopped {
        Stopped {
            distance: self.initial_distance + self.velocity * seconds,
        }
    }
}
