use glam::Vec2;
use vexide::math::Angle;
use vexide::prelude::*;
use vexide::smart::PortError;

pub struct Drivetrain {
    pub motors: [(Motor, Angle); 4], // Motor and rotation for the holonomic drive
}

impl Drivetrain {
    // Translate because i cant name it move and translate is too
    // long to type out for my lazy ahh so i shortened it
    pub fn trans(&mut self, direction: Vec2, rotation: f32) -> Result<(), PortError> {
        // let direction = direction * Vec2 { x: -1.0, y: 1.0 };
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
