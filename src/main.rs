mod drivetrain;

use vexide::prelude::*;

use crate::drivetrain::Drivetrain;

struct Arm {
    lift: [Motor; 2],
    claw: AdiDigitalOut,
}

struct Robot {
    controller: Controller,
    arm: Arm,
    drivetrain: Drivetrain,
}

impl Compete for Robot {
    async fn autonomous(&mut self) {
        println!("Autonomous!");
    }

    async fn driver(&mut self) {
        println!("Driver!");
    }
}

#[vexide::main]
async fn main(p: Peripherals) {
    let robot = Robot {
        controller: p.primary_controller,
        arm: Arm {
            lift: [
                Motor::new(p.port_1, Gearset::Red, Direction::Forward),
                Motor::new(p.port_2, Gearset::Red, Direction::Reverse),
            ],
            claw: AdiDigitalOut::new(p.adi_a),
        },
        drivetrain: Drivetrain {
            motors: [
                (
                    Motor::new(p.port_3, Gearset::Green, Direction::Forward),
                    0.0,
                ),
                (
                    Motor::new(p.port_4, Gearset::Green, Direction::Forward),
                    0.0,
                ),
                (
                    Motor::new(p.port_5, Gearset::Green, Direction::Forward),
                    0.0,
                ),
                (
                    Motor::new(p.port_6, Gearset::Green, Direction::Forward),
                    0.0,
                ),
            ],
        },
    };

    robot.compete().await;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_adds_two() {
        assert_eq!(2 + 2, 4);
    }
}
