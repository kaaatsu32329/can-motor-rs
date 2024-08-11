use crate::cybergear::*;
use crate::*;

const CAN_ID_CYBERGEAR_DEFAULT: u8 = 0x7F;

const CMD_ID_NOTHING: u32 = 0;
const CMD_ID_RUN_TORQUE: u32 = 1;
pub(crate) const CMD_ID_FEEDBACK: u32 = 2;
const CMD_ID_TORQUE_ENABLE: u32 = 3;
const CMD_ID_TORQUE_DISABLE: u32 = 4;
const CMD_ID_SET_MECHANICAL_ZERO: u32 = 6;
const CMD_ID_CHANGE_CAN_ID: u32 = 7;
pub(crate) const CMD_ID_READ_PARAMETER: u32 = 17;
const CMD_ID_WRITE_PARAMETER: u32 = 18;
pub(crate) const CMD_ID_FAILURE_FEEDBACK: u32 = 21;
const CMD_ID_CHANGE_BAUDRATE: u32 = 22;

/// radians
pub(crate) const MAX_ANGLE: f32 = 4.0 * core::f32::consts::PI;
/// radians per second
pub(crate) const MAX_VELOCITY: f32 = 30.0;
/// Nm
pub(crate) const MAX_TORQUE: f32 = 12.0;

pub const MAX_P_GAIN: f32 = 500.0;
pub const MAX_D_GAIN: f32 = 5.0;

#[derive(Debug)]
pub struct CyberGear {
    target_id: u8,
    feedback: Feedback,
}

impl CyberGear {
    pub fn new(target_id: u8) -> Self {
        Self {
            target_id,
            feedback: Feedback::default(),
        }
    }

    /// CMD_ID: 0
    pub fn send_nothing(&self) -> CanBuffer {
        let extended_can_id = CMD_ID_NOTHING << 24 | self.target_id as u32;

        CanBuffer {
            extended_can_id,
            buffer: [0; 8],
        }
    }

    /// CMD_ID: 3
    pub fn enable_torque(&self) -> CanBuffer {
        let extended_id = CMD_ID_TORQUE_ENABLE << 24 | self.target_id as u32;

        CanBuffer {
            extended_can_id: extended_id,
            buffer: [0; 8],
        }
    }

    /// CMD_ID: 4
    pub fn disable_torque(&self) -> CanBuffer {
        let extended_id = CMD_ID_TORQUE_DISABLE << 24 | self.target_id as u32;

        CanBuffer {
            extended_can_id: extended_id,
            buffer: [0; 8],
        }
    }

    /// CMD_ID: 1
    pub fn run_torque(&self, torque: f32) -> CanBuffer {
        let torque_clamped = torque.clamp(-MAX_TORQUE, MAX_TORQUE);
        let torque_bytes =
            ((torque_clamped + MAX_TORQUE) / (2.0 * MAX_TORQUE) * 0x10000 as f32) as u16;

        let extended_can_id =
            CMD_ID_RUN_TORQUE << 24 | (torque_bytes as u32) << 8 | self.target_id as u32;

        CanBuffer {
            extended_can_id,
            buffer: [0; 8],
        }
    }

    /// CMD_ID: 1
    pub fn run_torque_with_param(
        &self,
        torque: f32,
        target_angle: f32,
        target_velocity: f32,
        p_gain: f32,
        d_gain: f32,
    ) -> CanBuffer {
        let torque_clamped = torque.clamp(-MAX_TORQUE, MAX_TORQUE);
        let torque_bytes =
            ((torque_clamped + MAX_TORQUE) / (2.0 * MAX_TORQUE) * 0x10000 as f32) as u16;

        let extended_can_id =
            CMD_ID_RUN_TORQUE << 24 | (torque_bytes as u32) << 8 | self.target_id as u32;

        let mut buffer = [0; 8];
        let target_angle_clamped = target_angle.clamp(-MAX_ANGLE, MAX_ANGLE);
        let target_angle_bytes =
            ((target_angle_clamped + MAX_ANGLE) / (2.0 * MAX_ANGLE) * 0x10000 as f32) as u16;
        let target_velocity_clamped = target_velocity.clamp(-MAX_VELOCITY, MAX_VELOCITY);
        let target_velocity_bytes = ((target_velocity_clamped + MAX_VELOCITY)
            / (2.0 * MAX_VELOCITY)
            * 0x10000 as f32) as u16;
        let p_gain_clamped = p_gain.clamp(0.0, MAX_P_GAIN);
        let p_gain_bytes = (p_gain_clamped / MAX_P_GAIN * 0x10000 as f32) as u16;
        let d_gain_clamped = d_gain.clamp(0.0, MAX_D_GAIN);
        let d_gain_bytes = (d_gain_clamped / MAX_D_GAIN * 0x10000 as f32) as u16;

        buffer[0] = (target_angle_bytes >> 8) as u8;
        buffer[1] = target_angle_bytes as u8;
        buffer[2] = (target_velocity_bytes >> 8) as u8;
        buffer[3] = target_velocity_bytes as u8;
        buffer[4] = (p_gain_bytes >> 8) as u8;
        buffer[5] = p_gain_bytes as u8;
        buffer[6] = (d_gain_bytes >> 8) as u8;
        buffer[7] = d_gain_bytes as u8;

        CanBuffer {
            extended_can_id,
            buffer,
        }
    }

    /// CMD_ID: 6
    pub fn set_mechanical_zero(&self) -> CanBuffer {
        let extended_can_id = CMD_ID_SET_MECHANICAL_ZERO << 24 | self.target_id as u32;
        let mut buffer = [0; 8];
        buffer[0] = 1;

        CanBuffer {
            extended_can_id,
            buffer,
        }
    }

    /// CMD_ID: 7
    pub fn change_can_id(&self, new_id: u8) -> CanBuffer {
        let extended_can_id =
            CMD_ID_CHANGE_CAN_ID << 24 | (new_id as u32) << 16 | self.target_id as u32;

        CanBuffer {
            extended_can_id,
            buffer: [0; 8],
        }
    }

    /// CMD_ID: 17
    pub fn read_parameter(&self, address: u16) -> CanBuffer {
        let extended_can_id = CMD_ID_READ_PARAMETER << 24 | (self.target_id as u32) << 8;

        let mut buffer = [0; 8];
        buffer[0] = (address >> 8) as u8;
        buffer[1] = address as u8;

        CanBuffer {
            extended_can_id,
            buffer,
        }
    }

    /// CMD_ID: 18
    pub fn write_parameter(&self, address: u16, _value: u16) -> CanBuffer {
        let _extended_can_id = CMD_ID_WRITE_PARAMETER << 24 | (self.target_id as u32) << 8;

        let mut buffer = [0; 8];
        buffer[0] = (address >> 8) as u8;
        buffer[1] = address as u8;
        todo!()
    }

    /// CMD_ID: 22
    pub fn change_baudrate(&self, baudrate: Baudrate) -> CanBuffer {
        let extended_can_id = CMD_ID_CHANGE_BAUDRATE << 24 | (self.target_id as u32) << 8;

        let mut buffer = [0; 8];
        buffer[0] = baudrate as u8;

        CanBuffer {
            extended_can_id,
            buffer,
        }
    }

    pub fn receive(&mut self, buffer: &CanBuffer) {
        self.feedback.from_can_buffer(buffer);
    }

    pub fn feedback(&self) -> &Feedback {
        &self.feedback
    }
}

impl Default for CyberGear {
    fn default() -> Self {
        Self {
            target_id: CAN_ID_CYBERGEAR_DEFAULT,
            feedback: Feedback::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_TARGET_TORQUE: f32 = 0.0;
    const EXPECTED_CAN_ID: u32 = 0x0180007F;

    #[test]
    fn test_run_torque() {
        let cyber_gear = CyberGear::new(CAN_ID_CYBERGEAR_DEFAULT);
        let can_buffer = cyber_gear.run_torque(TEST_TARGET_TORQUE);

        assert_eq!(can_buffer.extended_can_id, EXPECTED_CAN_ID);
    }
}
