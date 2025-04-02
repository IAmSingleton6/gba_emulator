mod arm_decoder;
mod thumb_decoder;

pub use arm_decoder::decode_arm;
pub use thumb_decoder::decode_thumb;