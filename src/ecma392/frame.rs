// Frame types
// Frame control field format: b15 ... b0
//  b0..b1: Protocol version 00
//  b2: secure 0 .. 1
//  b3..b4: ACK policy 00 [0 .. 3]
//  b5..b7: Frame type 000
//  b8..b11: Frame Subtype/Delivery ID 0000
//  b12: Retry: 1 or reserved
//  b13..b15: Reserved 000

// 7.1.2.1.2 Secure
const FRAME_CRTL_SECURE: u16 = 0x0004;
const FRAME_CRTL_NOT_SECURE: u16 = 0x0000;

// 7.1.2.1.3 ACK Policy
const FRAME_CTRL_ACK_POLICY_NO_ACK: u16 = 0x0000;
const FRAME_CTRL_ACK_POLICY_IMM_ACK: u16 = 0x0008;
const FRAME_CTRL_ACK_POLICY_B_ACK: u16 = 0x0010;
const FRAME_CTRL_ACK_POLICY_B_ACK_REQ: u16 = 0x0018;

// 7.1.2.1.4 Frame Type
const FRAME_CTRL_TYPE_BEACON: u16 = 0x0000;
const FRAME_CTRL_TYPE_CONTROL: u16 = 0x0020; // 0b0000_0000_0010_0000
const FRAME_CTRL_TYPE_COMMAND: u16 = 0x0040; // 0b0000_0000_0100_0000
const FRAME_CTRL_TYPE_DATA: u16 = 0x0060; // 0b0000_0000_0110_0000
const FRAME_CTRL_TYPE_AGGREGATED: u16 = 0x0080; // 0b0000_0000_1000_0000

// 7.1.2.1.5 Frame subtypes
// Beacon: Table 14
const FRAME_CTRL_SUBTYPE_BEACON_REGULAR: u16 = 0x0000;
const FRAME_CTRL_SUBTYPE_BEACON_SIGNALLING: u16 = 0x0000;
const FRAME_CTRL_SUBTYPE_BEACON_ECHO: u16 = 0x0000;
const FRAME_CTRL_SUBTYPE_BEACON_RESERVED: u16 = 0x0000; // 3 -15

// Control: Table 21
const FRAME_CTRL_SUBTYPE_CTRL_IMM_ACK: u16 = 0x0000;
const FRAME_CTRL_SUBTYPE_CTRL_B_ACK: u16 = 0x0000;
const FRAME_CTRL_SUBTYPE_CTRL_RTS: u16 = 0x0000;
const FRAME_CTRL_SUBTYPE_CTRL_CTS: u16 = 0x0000;
const FRAME_CTRL_SUBTYPE_CTRL_UCA: u16 = 0x0000;
const FRAME_CTRL_SUBTYPE_CTRL_UCR: u16 = 0x0000;
const FRAME_CTRL_SUBTYPE_CTRL_RESRVED: u16 = 0x0000; // 6 - 13, 15
const FRAME_CTRL_SUBTYPE_CTRL_APP_SPECIFIC: u16 = 0x0000;

// Command: Table 26
const FRAME_CTRL_SUBTYPE_CMD_CRP_RESERVATION_REQ: u16 = 0x0000; // CRP reservation request

// 7.1.2.1.6 Retry
const FRAME_CTRL_RETRY_ON: u16 = 0x0800;
const FRAME_CTRL_RETRY_OFF: u16 = 0x0000;

// Section 7.1 MAC Frame
pub struct MacFrame {
    header: MacHeader,
    body: Option<MacFrameBody>,
}

pub struct MacHeader {
    frame_ctrl: u16,
    dest_addr: u16,
    src_addr: u16,
    seq_ctrl: u16, // Order of MSDUs/MCDUs
    access_ctrl: u16,
}

pub struct MacFrameBody {
    payload: PayloadType,
    fcs: u32, // Frame Check Sequence
}

pub struct SecureFramePayload {
    tkid: [u8; 3], // Temporary Identifier
    secure_payload: Vec<u8>,
    mic: u64, // Message Integrity Code
}

pub enum PayloadType {
    Secure(SecureFramePayload),
    NotSecure(Vec<u8>),
}

impl MacFrameBody {
    pub fn is_zero(&self) -> bool {
        if self.payload_len() == 0 {
            true
        } else {
            false
        }
    }

    pub fn payload_len(&self) -> usize {
        match *self.payload {
            PayloadType::Secure(SecureFramePayload { secure_payload, .. }) => {
                return secure_payload.len()
            }
            PayloadType::NotSecure(v) => return v.len(),
        }
    }
}
