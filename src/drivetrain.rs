use std::f64::consts::PI;

use glam::{Mat2, Vec2};
use vexide::math::Angle;
use vexide::prelude::*;
use vexide::smart::PortError;

pub struct Drivetrain {
    pub motors: [(Motor, Angle); 4],
}

pub struct Odometry<const N: usize> {
    encoders: [(RotationSensor, f64, Angle, Angle); N],
    encoder_last_rotations: [Angle; N],
    imu: InertialSensor,
    wheel_diameter: f64, // inches
    position: Vec2,
    basis: Mat2,
    last_heading: Angle,
}

impl Drivetrain {
    // Translate because i cant name it move
    pub fn trans(&mut self, direction: Vec2, rotation: f32) -> Result<(), PortError> {
        let speed = direction.length();
        let dir = direction.normalize_or_zero();

        let mut outputs = [0f32; 4];
        let mut max_mag = 12f32;

        for (i, (_, angle)) in self.motors.iter_mut().enumerate() {
            let translation = dir.x * angle.cos() as f32 - dir.y * angle.sin() as f32;
            outputs[i] = translation * speed * 12.0 + rotation * 12.0;
            max_mag = max_mag.max(outputs[i].abs());
        }

        for (i, (motor, _)) in self.motors.iter_mut().enumerate() {
            motor.set_voltage(outputs[i] as f64 * (12.0 / max_mag) as f64)?;
        }
        Ok(())
    }
}

impl<const N: usize> Odometry<N> {
    pub fn new(
        &mut self,
        encoders: [(RotationSensor, f64, Angle, Angle); N],
        imu: InertialSensor,
        wheel_diameter: f64,
        starting_position: Vec2,
        starting_angle: Angle,
    ) -> Self {
        let mut positions = [Angle::default(); N];
        for i in 0..N {
            positions[i] = encoders[i].0.position().unwrap();
        }
        Self {
            encoders: encoders,
            imu: imu,
            wheel_diameter: wheel_diameter,
            position: starting_position,
            basis: Mat2::from_angle(starting_angle.as_radians() as f32),
            last_heading: starting_angle,
            encoder_last_rotations: positions,
        }
    }
    pub fn update(&mut self) {
        let heading = self.imu.heading().unwrap();
        let imu_change = heading - self.last_heading;
        self.last_heading = heading;

        // calculate new basis
        self.basis = Mat2::from_angle(heading.as_radians() as f32);

        let mut sum_vf = Vec2::ZERO;
        for i in 0..N {
            let (encoder, length, alpha, theta) = &mut self.encoders[i];

            let rotation = encoder.position().unwrap();
            let rotation_change = rotation - self.encoder_last_rotations[i];
            self.encoder_last_rotations[i] = rotation;

            let travel = rotation_change.as_radians() * self.wheel_diameter; // Inches

            let travelr = imu_change.as_radians()
                * length.clone()
                * (theta.as_radians() - alpha.as_radians() - PI / 2f64).cos();

            let forward_vec = Vec2 {
                x: theta.sin() as f32,
                y: theta.cos() as f32,
            };

            let travel_real = forward_vec * (travel - travelr) as f32;
            sum_vf += travel_real;
        }

        self.position += self.basis.inverse() * sum_vf;
    }
}
