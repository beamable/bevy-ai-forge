use bincode::config;
use serde::{de::DeserializeOwned, Serialize};

pub(crate) fn serialize<T>(value: &T) -> Result<Vec<u8>, bincode::error::EncodeError>
where
    T: Serialize + ?Sized,
{
    bincode::serde::encode_to_vec(value, config::legacy())
}

pub(crate) fn deserialize<T>(bytes: &[u8]) -> Result<T, bincode::error::DecodeError>
where
    T: DeserializeOwned,
{
    bincode::serde::decode_from_slice(bytes, config::legacy()).map(|(value, _bytes_read)| value)
}

#[cfg(test)]
mod tests {
    use super::{deserialize, serialize};
    use crate::NetworkPacket;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct FixtureMessage {
        number: u32,
        text: String,
    }

    const MESSAGE_BYTES: &[u8] = &[
        42, 0, 0, 0, // number
        4, 0, 0, 0, 0, 0, 0, 0, // text length
        b't', b'e', b's', b't',
    ];

    const PACKET_BYTES: &[u8] = &[
        7, 0, 0, 0, 0, 0, 0, 0, // kind length
        b'f', b'i', b'x', b't', b'u', b'r', b'e', 16, 0, 0, 0, 0, 0, 0, 0, // data length
        42, 0, 0, 0, // number
        4, 0, 0, 0, 0, 0, 0, 0, // text length
        b't', b'e', b's', b't',
    ];

    #[test]
    fn legacy_message_encoding_matches_bincode_1() {
        let message = FixtureMessage {
            number: 42,
            text: "test".to_owned(),
        };

        assert_eq!(serialize(&message).unwrap(), MESSAGE_BYTES);
        assert_eq!(
            deserialize::<FixtureMessage>(MESSAGE_BYTES).unwrap(),
            message
        );
    }

    #[test]
    fn legacy_packet_encoding_matches_bincode_1() {
        let packet = NetworkPacket {
            kind: "fixture".to_owned(),
            data: MESSAGE_BYTES.to_vec(),
        };

        assert_eq!(serialize(&packet).unwrap(), PACKET_BYTES);

        let decoded: NetworkPacket = deserialize(PACKET_BYTES).unwrap();
        assert_eq!(decoded.kind, packet.kind);
        assert_eq!(decoded.data, packet.data);
    }

    #[test]
    fn legacy_decoding_accepts_trailing_bytes() {
        let mut bytes = MESSAGE_BYTES.to_vec();
        bytes.extend_from_slice(&[0xaa, 0xbb]);

        let decoded: FixtureMessage = deserialize(&bytes).unwrap();
        assert_eq!(
            decoded,
            FixtureMessage {
                number: 42,
                text: "test".to_owned(),
            }
        );
    }
}
