#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u16)]
pub enum Address {
    /// 0: Operation control mode
    /// 1: Position mode
    /// 2: Velocity mode
    /// 3: Current mode
    RunMode(u8) = 0x7005,
    /// Current mode Iq command
    /// -90 ~ 90 A
    IqReference(f32) = 0x7006,
    /// Velocity mode velocity command
    /// -15 ~ 15 rad/s
    VelocityReference(f32) = 0x700A,
    /// Torque limitation
    /// 0 ~ 120 Nm
    LimitTorque(f32) = 0x700B,
    /// Kp of current
    /// Default value: 0.05
    CurrentKp(f32) = 0x7010,
    /// Ki of current
    /// Default value: 0.05
    CurrentKi(f32) = 0x7011,
    /// Current filtering coefficient
    /// 0 ~ 1.0, default value: 0.06
    CurrentFilterGain(f32) = 0x7014,
    /// Position mode angle command
    /// rad
    PositionCommand(f32) = 0x7016,
    /// Position mode velocity limit
    /// 0 ~ 15 rad/s
    LimitVelocity(f32) = 0x7017,
    /// Velocity position mode current limit
    /// 0 ~ 90 A
    LimitCurrent(f32) = 0x7018,
    /// Load end-ring mechanical angle
    /// rad
    MechanicalPosition(f32) = 0x7019,
    /// The Iq filter values
    /// -90 ~ 90 A
    IqF(f32) = 0x701A,
    /// Load end velocity
    /// -15 ~ 15 rad/s
    MechanicalVelocity(f32) = 0x701B,
    /// Busbar voltage
    /// V
    VBus(f32) = 0x701C,
    /// Position Kp
    /// Default value: 30
    PositionKp(f32) = 0x701E,
    /// Velocity Kp
    /// Default value: 5
    VelocityKp(f32) = 0x701F,
    /// Veclocity Ki
    /// Default value: 0.005
    VelocityKi(f32) = 0x7020,
    /// Velocity filter value
    /// Default value: 0.1
    VelocityFilterGain(f32) = 0x7021,
}

impl Address {
    pub fn as_bytes(&self) -> [u8; 4] {
        let mut buffer = [0; 4];
        match self {
            Address::RunMode(inner) => buffer[0] = *inner,
            Address::IqReference(inner) => {
                let bytes = inner.to_le_bytes();
                buffer[0..4].copy_from_slice(&bytes);
            }
            Address::VelocityReference(inner) => {
                let bytes = inner.to_le_bytes();
                buffer[0..4].copy_from_slice(&bytes);
            }
            Address::LimitTorque(inner) => {
                let bytes = inner.to_le_bytes();
                buffer[0..4].copy_from_slice(&bytes);
            }
            Address::CurrentKp(inner) => {
                let bytes = inner.to_le_bytes();
                buffer[0..4].copy_from_slice(&bytes);
            }
            Address::CurrentKi(inner) => {
                let bytes = inner.to_le_bytes();
                buffer[0..4].copy_from_slice(&bytes);
            }
            Address::CurrentFilterGain(inner) => {
                let bytes = inner.to_le_bytes();
                buffer[0..4].copy_from_slice(&bytes);
            }
            Address::PositionCommand(inner) => {
                let bytes = inner.to_le_bytes();
                buffer[0..4].copy_from_slice(&bytes);
            }
            Address::LimitVelocity(inner) => {
                let bytes = inner.to_le_bytes();
                buffer[0..4].copy_from_slice(&bytes);
            }
            Address::LimitCurrent(inner) => {
                let bytes = inner.to_le_bytes();
                buffer[0..4].copy_from_slice(&bytes);
            }
            Address::MechanicalPosition(inner) => {
                let bytes = inner.to_le_bytes();
                buffer[0..4].copy_from_slice(&bytes);
            }
            Address::IqF(inner) => {
                let bytes = inner.to_le_bytes();
                buffer[0..4].copy_from_slice(&bytes);
            }
            Address::MechanicalVelocity(inner) => {
                let bytes = inner.to_le_bytes();
                buffer[0..4].copy_from_slice(&bytes);
            }
            Address::VBus(inner) => {
                let bytes = inner.to_le_bytes();
                buffer[0..4].copy_from_slice(&bytes);
            }
            Address::PositionKp(inner) => {
                let bytes = inner.to_le_bytes();
                buffer[0..4].copy_from_slice(&bytes);
            }
            Address::VelocityKp(inner) => {
                let bytes = inner.to_le_bytes();
                buffer[0..4].copy_from_slice(&bytes);
            }
            Address::VelocityKi(inner) => {
                let bytes = inner.to_le_bytes();
                buffer[0..4].copy_from_slice(&bytes);
            }
            Address::VelocityFilterGain(inner) => {
                let bytes = inner.to_le_bytes();
                buffer[0..4].copy_from_slice(&bytes);
            }
        };

        buffer
    }
}

// ????
impl Into<u16> for Address {
    fn into(self) -> u16 {
        match self {
            Address::RunMode(_) => 0x7005,
            Address::IqReference(_) => 0x7006,
            Address::VelocityReference(_) => 0x700A,
            Address::LimitTorque(_) => 0x700B,
            Address::CurrentKp(_) => 0x7010,
            Address::CurrentKi(_) => 0x7011,
            Address::CurrentFilterGain(_) => 0x7014,
            Address::PositionCommand(_) => 0x7016,
            Address::LimitVelocity(_) => 0x7017,
            Address::LimitCurrent(_) => 0x7018,
            Address::MechanicalPosition(_) => 0x7019,
            Address::IqF(_) => 0x701A,
            Address::MechanicalVelocity(_) => 0x701B,
            Address::VBus(_) => 0x701C,
            Address::PositionKp(_) => 0x701E,
            Address::VelocityKp(_) => 0x701F,
            Address::VelocityKi(_) => 0x7020,
            Address::VelocityFilterGain(_) => 0x7021,
        }
    }
}
