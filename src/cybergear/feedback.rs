use crate::cybergear::*;
use crate::*;

#[derive(Debug, Clone, Copy)]
pub enum FeedbackType {
    Normal = CMD_ID_FEEDBACK as isize,
    ParameterResult = CMD_ID_READ_PARAMETER as isize,
    Failure = CMD_ID_FAILURE_FEEDBACK as isize,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Feedback {
    is_not_calibrated: bool,
    is_hall_encoder_error: bool,
    is_magnet_encoder_error: bool,
    is_overheating: bool,
    is_overcurrent: bool,
    is_undervoltage: bool,
    mode: MotorModeFeedback,

    pub(crate) angle: f32,
    pub(crate) velocity: f32,
    pub(crate) torque: f32,
    pub(crate) temperature: f32,
}

impl Feedback {
    pub fn from_can_buffer(&mut self, buffer: &CanBuffer) {
        match Self::check_id(buffer.extended_can_id) {
            Some(FeedbackType::Normal) => {
                self.parse_id_normal(buffer.extended_can_id);
                self.parse_data_buffer_normal(&buffer.buffer);
            }
            Some(FeedbackType::ParameterResult) => {
                self.parse_data_buffer_parameter(&buffer.buffer);
            }
            Some(FeedbackType::Failure) => {
                self.parse_data_buffer_failure(&buffer.buffer);
            }
            _ => {}
        }
    }

    fn check_id(id: u32) -> Option<FeedbackType> {
        match id >> 24 {
            CMD_ID_FEEDBACK => Some(FeedbackType::Normal),
            CMD_ID_READ_PARAMETER => Some(FeedbackType::ParameterResult),
            _ => None,
        }
    }

    fn parse_id_normal(&mut self, id: u32) {
        self.is_not_calibrated = (id >> 21) & 1 == 1;
        self.is_hall_encoder_error = (id >> 20) & 1 == 1;
        self.is_magnet_encoder_error = (id >> 19) & 1 == 1;
        self.is_overheating = (id >> 18) & 1 == 1;
        self.is_overcurrent = (id >> 17) & 1 == 1;
        self.is_undervoltage = (id >> 16) & 1 == 1;

        self.mode = match (id >> 22) & 0xFF {
            0 => MotorModeFeedback::Reset,
            1 => MotorModeFeedback::Calibration,
            2 => MotorModeFeedback::Motor,
            _ => MotorModeFeedback::Reset,
        };
    }

    fn parse_data_buffer_normal(&mut self, buffer: &[u8; 8]) {
        self.angle = u16::from_le_bytes(buffer[0..2].try_into().unwrap()) as f32 * 2.0 * MAX_ANGLE
            / u16::MAX as f32
            - MAX_ANGLE;
        self.velocity =
            u16::from_le_bytes(buffer[2..4].try_into().unwrap()) as f32 * 2.0 * MAX_VELOCITY
                / u16::MAX as f32
                - MAX_VELOCITY;
        self.torque =
            u16::from_le_bytes(buffer[4..6].try_into().unwrap()) as f32 * 2.0 * MAX_TORQUE
                / u16::MAX as f32
                - MAX_TORQUE;
        self.temperature = u16::from_le_bytes(buffer[6..8].try_into().unwrap()) as f32 * 10.0;
    }

    fn parse_data_buffer_parameter(&mut self, _buffer: &[u8; 8]) {
        todo!()
    }

    fn parse_data_buffer_failure(&mut self, _buffer: &[u8; 8]) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CAN_ID: u32 = 0x02807F00;
    const TEST_CAN_DATA: [u8; 8] = [0x75, 0x8A, 0x54, 0xC6, 0x61, 0xFD, 0x01, 0x3F];

    const EXPECTED_ANGLE: f32 = 1.026824;
    const EXPECTED_VELOCITY: f32 = 16.483864;

    #[test]
    fn test_feedback() {
        let mut feedback = Feedback::default();
        let can_buffer = CanBuffer {
            extended_can_id: TEST_CAN_ID,
            buffer: TEST_CAN_DATA,
        };

        feedback.from_can_buffer(&can_buffer);

        assert_eq!(feedback.is_not_calibrated, false);
        assert_eq!(feedback.is_hall_encoder_error, false);
        assert_eq!(feedback.is_magnet_encoder_error, false);
        assert_eq!(feedback.is_overheating, false);
        assert_eq!(feedback.is_overcurrent, false);
        assert_eq!(feedback.is_undervoltage, false);
        assert_eq!(feedback.mode, MotorModeFeedback::Reset);
        assert_eq!(feedback.angle, EXPECTED_ANGLE);
        assert_eq!(feedback.velocity, EXPECTED_VELOCITY);
        assert_eq!(feedback.torque, 1.0);
    }
}
