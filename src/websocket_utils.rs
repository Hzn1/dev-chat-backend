pub enum Opcode {
    Utf8Text = 0x1,
    Binary = 0x2,
    CloseRequest = 0x8,
    Ping = 0x9,
    Pong = 0xA,
}

pub enum PayloadLen {
    Exact(u64),
    NextTwoBytes,
    NextEightBytes,
}

pub struct WsMessageHeader {
    pub fin: bool,
    pub opcode: Opcode,
    pub is_masked: bool,
    pub payload_len: PayloadLen,
    pub masking_key: Option<[u8; 4]>,
}

/* pub fn extract_ws_message_frame_headers(message_buffer: &[u8]) -> WsMessageHeader {
    WsMessageHeader {
        fin: (),
        opcode: (),
        is_masked: (),
        payload_len: (),
        masking_key: (),
    }
} */
