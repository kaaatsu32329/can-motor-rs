#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum MotorModeFeedback {
    #[default]
    Reset,
    Calibration,
    Motor,
}
