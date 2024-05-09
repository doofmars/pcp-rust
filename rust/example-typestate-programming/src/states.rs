// state stopped
pub struct Stopped {
    distance: u32
}
impl Stopped {
    pub fn new() -> Self {
        Self {
            distance: 0
        }
    }

    pub fn get_distance(&self) -> u32 {
        self.distance
    }

    pub fn accelerate(&self, acceleration: u32, seconds: u32) -> Moving {
        Moving {
            velocity: acceleration * seconds,
            initial_distance: self.distance
        }
    }
}


// state moving
pub struct Moving {
    initial_distance: u32,
    velocity: u32
}
impl Moving {
    pub fn get_velocity(&self) -> u32 {
        self.velocity
    }

    pub fn stop_after(&self, seconds: u32) -> Stopped {
        Stopped {
            distance: self.initial_distance + self.velocity * seconds
        }
    }
}
