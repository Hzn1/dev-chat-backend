use base64::prelude::*;
use sha1::{Digest, Sha1};

const STANDARD_UUID_OF_WEB_SOCKET_PROTOCOL: &'static str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

pub const HANDSHAKE_HTTP_REQUEST_ITEMS_LIST: [&'static str; 5] = [
    "GET /ws HTTP/1.1",
    "Upgrade: websocket",
    "Connection: Upgrade",
    "Sec-WebSocket-Key:",
    "Sec-WebSocket-Version: 13",
];
const HANDSHAKE_HTTP_RESPONSE_BAD_REQUEST: &str = "HTTP/1.1 400 Bad Request\r\n\
    Connection: close\r\n\
    Content-Type: text/plain\r\n\
    Content-Length: 25\r\n\
    \r\n\
    Invalid Handshake Request";

pub fn verify_if_is_a_handshake_request(message_received: &String) -> bool {
    for http_item in HANDSHAKE_HTTP_REQUEST_ITEMS_LIST {
        if !message_received.contains(http_item) {
            return false;
        }
    }
    true
}

fn extract_websocket_key_from_handshake_request(message_received: String) -> Option<String> {
    let splited_message: Vec<&str> = message_received.split("\r\n").collect();
    for item in splited_message {
        if item.contains("Sec-WebSocket-Key: ") {
            let values: Vec<&str> = item.split(" ").collect();
            return Some(values.get(1).unwrap().to_string());
        }
    }
    None
}

pub fn make_handshake_http_response(message_received: String) -> String {
    let ws_key = extract_websocket_key_from_handshake_request(message_received);

    match ws_key {
        Some(key) => {
            let key_with_ws_uuid = format!("{}{}", key, STANDARD_UUID_OF_WEB_SOCKET_PROTOCOL);
            let hashed = Sha1::digest(key_with_ws_uuid.into_bytes());
            let base64_reponse = BASE64_STANDARD.encode(hashed);

            return format!(
                "HTTP/1.1 101 Switching Protocols\r\n\
                Upgrade: websocket\r\n\
                Connection: Upgrade\r\n\
                Sec-WebSocket-Accept: {}\r\n\r\n",
                base64_reponse
            );
        }
        None => return HANDSHAKE_HTTP_RESPONSE_BAD_REQUEST.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_correct_handshake_request() {
        let http_request = String::from(
            "GET /ws HTTP/1.1\r\n\
            Host: example.com\r\n\
            Upgrade: websocket\r\n\
            Connection: Upgrade\r\n\
            Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n\
            Sec-WebSocket-Version: 13\r\n\
            Origin: http://your-frontend-domain.com\r\n\
            User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64)...\r\n\
            Accept-Encoding: gzip, deflate\r\n\
            Accept-Language: en-US,en;q=0.9\r\n\r\n",
        );

        assert_eq!(verify_if_is_a_handshake_request(&http_request), true);
    }

    #[test]
    fn test_verify_incorrect_handshake_request() {
        let http_request = String::from(
            "POST /ws HTTP/1.1\r\n\
            Host: example.com\r\n\
            Upgrade: websocket\r\n\
            Connection: Upgrade\r\n\
            Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n\
            Sec-WebSocket-Version: 13\r\n\
            Origin: http://your-frontend-domain.com\r\n\
            User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64)...\r\n\
            Accept-Encoding: gzip, deflate\r\n\
            Accept-Language: en-US,en;q=0.9\r\n\r\n",
        );

        assert_eq!(verify_if_is_a_handshake_request(&http_request), false);
    }

    #[test]
    fn test_extracting_a_valid_websocket_key() {
        let http_request = String::from(
            "GET /ws HTTP/1.1\r\n\
            Host: example.com\r\n\
            Upgrade: websocket\r\n\
            Connection: Upgrade\r\n\
            Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n\
            Sec-WebSocket-Version: 13\r\n\
            Origin: http://your-frontend-domain.com\r\n\
            User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64)...\r\n\
            Accept-Encoding: gzip, deflate\r\n\
            Accept-Language: en-US,en;q=0.9\r\n\r\n",
        );

        assert_eq!(
            extract_websocket_key_from_handshake_request(http_request),
            Some(String::from("dGhlIHNhbXBsZSBub25jZQ=="))
        );
    }

    #[test]
    fn test_extracting_with_a_empty_websocket_key() {
        let http_request = String::from(
            "GET /ws HTTP/1.1\r\n\
            Host: example.com\r\n\
            Upgrade: websocket\r\n\
            Connection: Upgrade\r\n\
            Sec-WebSocket-Version: 13\r\n\
            Origin: http://your-frontend-domain.com\r\n\
            User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64)...\r\n\
            Accept-Encoding: gzip, deflate\r\n\
            Accept-Language: en-US,en;q=0.9\r\n\r\n",
        );

        assert_eq!(
            extract_websocket_key_from_handshake_request(http_request),
            None
        );
    }

    #[test]
    fn test_make_handshake_http_reponse_with_correct_request() {
        let http_request = String::from(
            "GET /ws HTTP/1.1\r\n\
            Host: example.com\r\n\
            Upgrade: websocket\r\n\
            Connection: Upgrade\r\n\
            Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n\
            Sec-WebSocket-Version: 13\r\n\
            Origin: http://your-frontend-domain.com\r\n\
            User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64)...\r\n\
            Accept-Encoding: gzip, deflate\r\n\
            Accept-Language: en-US,en;q=0.9\r\n\r\n",
        );

        let expected_response = String::from(
            "HTTP/1.1 101 Switching Protocols\r\n\
            Upgrade: websocket\r\n\
            Connection: Upgrade\r\n\
            Sec-WebSocket-Accept: s3pPLMBiTxaQ9kYGzzhZRbK+xOo=\r\n\r\n",
        );

        assert_eq!(
            make_handshake_http_response(http_request),
            expected_response
        );
    }

    #[test]
    fn test_make_handshake_http_reponse_with_incorrect_request() {
        let http_request = String::from(
            "GET /ws HTTP/1.1\r\n\
            Host: example.com\r\n\
            Upgrade: websocket\r\n\
            Connection: Upgrade\r\n\
            Sec-WebSocket-Version: 13\r\n\
            Origin: http://your-frontend-domain.com\r\n\
            User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64)...\r\n\
            Accept-Encoding: gzip, deflate\r\n\
            Accept-Language: en-US,en;q=0.9\r\n\r\n",
        );

        assert_eq!(
            make_handshake_http_response(http_request),
            HANDSHAKE_HTTP_RESPONSE_BAD_REQUEST
        );
    }
}
