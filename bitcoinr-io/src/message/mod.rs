mod decoder;
mod encoder;
mod get_addr;

pub use self::decoder::decode_message;
pub use self::encoder::encode_message;

use net::NetworkType;


pub const SIZE_OF_HEADER: usize = 24;


/// `Message` represents a message which contains `network_type` and `command` field.
pub struct Message {
    network_type: NetworkType,
    command: Command,
}


pub enum Command {
    GetAddr,
    // Addr(Address),
}


pub(self) struct Header {
    pub start_string: [u8; 4],
    pub command_name: [u8; 12],
    pub payload_size: u32,
    pub checksum: [u8; 4],
}
