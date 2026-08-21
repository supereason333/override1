mod drivetrain;

use glam::Vec2;
use vexide::math::Angle;
use vexide::prelude::*;
use vexide::smart::motor::BrakeMode;

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

impl Arm {}

impl Compete for Robot {
    async fn autonomous(&mut self) {
        println!("Autonomous!");
    }

    async fn driver(&mut self) {
        println!("Driver!");
        loop {
            let state = self.controller.state().unwrap_or_default();

            // Drivetrain movement
            let movement = Vec2 {
                // x and y is normalized Vec2 is not normalized.
                x: state.left_stick.x() as f32,
                y: state.left_stick.y() as f32,
            };
            let rotation = state.right_stick.x() as f32;

            self.drivetrain.trans(movement, rotation).unwrap();

            let lift_speed;
            if state.button_r2.is_pressed() {
                lift_speed = 20;
            } else {
                lift_speed = 80;
            }

            // Arm movement
            for motor in self.arm.lift.iter_mut() {
                if state.button_l1.is_pressed() {
                    motor.set_velocity(-lift_speed).unwrap();
                } else if state.button_l2.is_pressed() {
                    motor.set_velocity(lift_speed).unwrap();
                } else {
                    motor.set_velocity(0).unwrap();
                    motor.brake(BrakeMode::Hold).unwrap();
                }
            }

            if state.button_r1.is_now_pressed() {
                self.arm.claw.toggle().unwrap();
            }

            sleep(Controller::UPDATE_INTERVAL).await;
        }
    }
}

#[vexide::main]
async fn main(p: Peripherals) {
    let robot = Robot {
        controller: p.primary_controller,
        arm: Arm {
            lift: [
                Motor::new(p.port_8, Gearset::Red, Direction::Forward),
                Motor::new(p.port_11, Gearset::Red, Direction::Reverse),
            ],
            claw: AdiDigitalOut::new(p.adi_a),
        },
        drivetrain: Drivetrain {
            motors: [
                (
                    Motor::new(p.port_7, Gearset::Green, Direction::Forward),
                    Angle::from_degrees(45.0),
                ),
                (
                    Motor::new(p.port_10, Gearset::Green, Direction::Forward),
                    Angle::from_degrees(135.0),
                ),
                (
                    Motor::new(p.port_9, Gearset::Green, Direction::Forward),
                    Angle::from_degrees(225.0),
                ),
                (
                    Motor::new(p.port_20, Gearset::Green, Direction::Forward),
                    Angle::from_degrees(315.0),
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
