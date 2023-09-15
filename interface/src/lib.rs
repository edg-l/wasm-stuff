#[derive(Debug, Clone)]
#[repr(C, packed)]
pub struct Game {
    pub data: [u8; 32],
}
