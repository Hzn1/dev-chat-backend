use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

mod http_utils;
mod models;
mod storage;
mod websocket_utils;

enum ConnectionState {
    HttpHandshake,
    WebSocketActive,
}

fn handle_client(mut stream: TcpStream) {
    let mut state = ConnectionState::HttpHandshake;
    let mut buffer = [0; 512]; // Buffer to read coming data
    let mut incoming_bytes: Vec<u8> = Vec::new();
    //let mut incoming_bytes_count = 0;

    // Loop to keep listening for messages from the client
    loop {
        match state {
            ConnectionState::HttpHandshake => {
                match stream.read(&mut buffer) {
                    Ok(0) => {
                        println!("Client diconnected");
                        break;
                    }
                    Ok(size) => {
                        // println!("Buffer: {:#?}", String::from_utf8_lossy(&buffer).into_owned());
                        for byte in buffer[..size].iter() {
                            incoming_bytes.push(*byte);
                        }

                        let headers_end = incoming_bytes
                            .windows(4)
                            .position(|window| window == b"\r\n\r\n");

                        match headers_end {
                            Some(pos) => {
                                let message_received =
                                    String::from_utf8_lossy(&incoming_bytes[..pos]).into_owned();
                                // println!("{}", message_received);
                                //incoming_bytes_count += size;
                                // println!("Received {} bytes", incoming_bytes_count);
                                // println!("Received message: {}", message_received);

                                match http_utils::verify_if_is_a_handshake_request(
                                    &message_received,
                                ) {
                                    true => {
                                        let http_response =
                                            http_utils::make_handshake_http_response(
                                                message_received,
                                            )
                                            .into_bytes();
                                        let r: &[u8] = &http_response[..];
                                        state = ConnectionState::WebSocketActive;

                                        _ = stream.write_all(r);
                                        continue;
                                    }
                                    false => {
                                        continue;
                                    }
                                }
                            }
                            None => {
                                //incoming_bytes_count += size;
                                continue;
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error on read the stream: {}", e);
                        break;
                    }
                }
            }
            ConnectionState::WebSocketActive => {
                let mut header = [0u8; 2];
                _ = stream.read_exact(&mut header);

                let fin = (header[0] & 0b1000_0000) != 0;
                let opcode = header[0] & 0b0000_1111;
                let mask = (header[1] & 0b1000_0000) != 0;
                let payload_len_indicator = header[1] & 0b0111_1111;

                if opcode == 0x8 {
                    println!("Connection with the client closed");
                    break;
                }

                let real_payload_len = match payload_len_indicator {
                    0..=125 => payload_len_indicator as u64,
                    126 => {
                        let mut len_bytes = [0u8; 2];
                        _ = stream.read_exact(&mut len_bytes);
                        u16::from_be_bytes(len_bytes) as u64
                    }
                    127 => {
                        let mut len_bytes = [0u8; 8];
                        _ = stream.read_exact(&mut len_bytes);
                        u64::from_be_bytes(len_bytes)
                    }
                    _ => unreachable!(),
                };

                let mask_key = if mask {
                    let mut key = [0u8; 4];
                    _ = stream.read_exact(&mut key);
                    Some(key)
                } else {
                    None
                };

                let mut payload = vec![0u8; real_payload_len as usize];
                _ = stream.read_exact(&mut payload);

                // Decode payload running XOR with MaskKey
                if let Some(key) = mask_key {
                    for (i, byte) in payload.iter_mut().enumerate() {
                        *byte ^= key[i % 4];
                    }
                }

                let message = String::from_utf8(payload).unwrap();
                println!("Message reaceived from client: {}", message);

                if !fin {
                    println!(
                        "There are more chunks of data incoming from the same message, but for now it's not implemented..."
                    );
                }
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    // Listening on localhost on port 7878
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server running on port 7878...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection accepted!");
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Fail in connection: {}", e);
            }
        }
    }

    Ok(())
}
