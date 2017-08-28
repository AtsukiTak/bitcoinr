use bytes::{BytesMut, BufMut, BigEndian};

use sha2::{Sha256, Digest};

use super::{SIZE_OF_HEADER, Message, Command, EMPTY_STRING_CHECKSUM};
use super::get_addr;
use error::*;


pub fn encode_message(msg: Message, dst: &mut BytesMut) -> Result<()> {

    // Get each command name and payload bytes.
    let (command_name, payload) = match msg.command {
        Command::GetAddr => get_addr::command_name_and_payload(),
    };

    let payload_size = payload.len();

    // Write message header.
    dst.reserve(SIZE_OF_HEADER);
    dst.put(msg.network_type.start_string().as_ref());
    dst.put(command_name.as_ref());
    dst.put_u32::<BigEndian>(payload_size as u32);
    if payload_size == 0 {
        dst.put(EMPTY_STRING_CHECKSUM.as_ref());
    } else {
        // Calculate Sha256 hash
        let hashed_once = Sha256::digest(payload.as_ref());
        let hashed_twice = Sha256::digest(&hashed_once.as_ref());
        dst.put(hashed_twice[0..4].as_ref());
    }

    // Write payload.
    dst.reserve(payload_size);
    dst.put(payload);

    Ok(())
}
