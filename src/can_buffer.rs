use embedded_can::{ExtendedId, Frame};
#[cfg(feature = "socketcan")]
use socketcan::{CanFrame, Id};

#[derive(Debug, Clone, Copy)]
pub struct CanBuffer {
    pub(crate) extended_can_id: u32,
    pub(crate) buffer: [u8; 8],
}

impl CanBuffer {
    #[cfg(feature = "socketcan")]
    pub fn as_socketcan_can_frame(&self) -> Option<CanFrame> {
        match ExtendedId::new(self.extended_can_id) {
            Some(id) => CanFrame::new(id, &self.buffer),
            None => None,
        }
    }

    #[cfg(feature = "socketcan")]
    pub fn from_socketcan_can_frame(frame: CanFrame) -> Option<Self> {
        if let Id::Extended(extended_id) = frame.id() {
            let id = extended_id.as_raw();
            let buffer = frame.data().try_into().unwrap();
            return Some(Self {
                extended_can_id: id,
                buffer,
            });
        }
        None
    }
}
