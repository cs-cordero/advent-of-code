use std::cmp::max;
use std::ops::RangeInclusive;

// From input:
// target area: x=185..221, y=-122..-74
static TARGET_X_RANGE: RangeInclusive<isize> = 185..=221;
static TARGET_Y_RANGE: RangeInclusive<isize> = -122..=-74;

fn main() {
    let x_range = find_x_range();

    let answer1 = {
        let mut highest_y_ever = 0;
        for x in x_range.clone() {
            if let Some(highest_y) = find_highest_y(x) {
                if highest_y > highest_y_ever {
                    highest_y_ever = highest_y;
                }
            }
        }
        highest_y_ever
    };

    let answer2 = {
        let mut count = 0;
        for x in 0..=*TARGET_X_RANGE.end() {
            for y in *TARGET_Y_RANGE.start()..500 {
                if launch_probe(x, y).is_some() {
                    count += 1;
                }
            }
        }
        count
    };

    println!("Part 1: {:?}", answer1);
    println!("Part 2: {:?}", answer2);
}

/// Determines the range of initial x velocities where the probe would stop moving
/// in the x direction and the x position is in the correct range.
/// This is not the same as the range of ALL initial x's that can be in the range.
fn find_x_range() -> RangeInclusive<isize> {
    fn find_finished_x_position(initial_velocity: isize) -> isize {
        initial_velocity.signum() * ((initial_velocity.abs() * (initial_velocity.abs() + 1)) / 2)
    }

    // Find the lower bound
    let lo_bound = {
        let mut velocity = 0;
        loop {
            if TARGET_X_RANGE.contains(&find_finished_x_position(velocity)) {
                break;
            }
            velocity += 1;
        }
        velocity
    };

    // Find the upper bound
    let hi_bound_non_inclusive = {
        let mut velocity = lo_bound;
        loop {
            if !TARGET_X_RANGE.contains(&find_finished_x_position(velocity)) {
                break;
            }
            velocity += 1;
        }
        velocity
    };

    lo_bound..=(hi_bound_non_inclusive - 1)
}

/// For a given initial x velocity, finds the highest y position reached along
/// the probe's trajectory while still landing in the target at some point later
/// in the probe path.
///
/// Tests a hardcoded y range of 0..1000.
/// If it never hits the target, then returns None.
fn find_highest_y(initial_x_velocity: isize) -> Option<isize> {
    (0..1000)
        .filter_map(|initial_y_velocity| launch_probe(initial_x_velocity, initial_y_velocity))
        .max()
}

/// Simulate probe trajectory
///
/// Returns Some(highest_y_position_ever_reached) if the probe ever touches the target.
/// Returns None if it never hits the target.
fn launch_probe(mut velocity_x: isize, mut velocity_y: isize) -> Option<isize> {
    let mut position_x = 0;
    let mut position_y = 0;
    let mut highest_y = 0;
    loop {
        let will_never_reach_x_target = (&position_x < TARGET_X_RANGE.start() && velocity_x <= 0)
            || (&position_x > TARGET_X_RANGE.end() && velocity_x >= 0);
        let will_never_reach_y_target = &position_y < TARGET_Y_RANGE.start();

        if will_never_reach_x_target || will_never_reach_y_target {
            return None;
        } else if TARGET_X_RANGE.contains(&position_x) && TARGET_Y_RANGE.contains(&position_y) {
            return Some(highest_y);
        }

        position_x += velocity_x;
        position_y += velocity_y;
        highest_y = max(highest_y, position_y);

        velocity_x = velocity_x.signum() * (velocity_x.abs() - 1);
        velocity_y -= 1;
    }
}
