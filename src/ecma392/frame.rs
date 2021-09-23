// Copyright (c) TVWS Project


// Frame control field format: b15 ... b0
//  b0..b1: Protocol version 00
//  b2: secure 0 .. 1
//  b3..b4: ACK policy 00 [0 .. 3]
//  b5..b7: Frame type 000
//  b8..b11: Frame Subtype/Delivery ID 0000
//  b12: Retry: 1 or reserved
//  b13..b15: Reserved 000

// 7.1.2.1.2 Secure
// 0b0000_0000_0000_0x00
const FRAME_CRTL_SECURE: u16 = 0x0004;

// 7.1.2.1.3 ACK Policy
// 0b0000_0000_000x_x000
const FRAME_CTRL_ACK_POLICY_NO_ACK: u16 = 0x0000;
const FRAME_CTRL_ACK_POLICY_IMM_ACK: u16 = 0x0008;
const FRAME_CTRL_ACK_POLICY_B_ACK: u16 = 0x0010;
const FRAME_CTRL_ACK_POLICY_B_ACK_REQ: u16 = 0x0018;

// 7.1.2.1.4 Frame Type
// 0b0000_0000_xxx0_0000
const FRAME_CTRL_TYPE_BEACON: u16 = 0x0000;
const FRAME_CTRL_TYPE_CTRL: u16 = 0x0020;
const FRAME_CTRL_TYPE_CMD: u16 = 0x0040;
const FRAME_CTRL_TYPE_DATA: u16 = 0x0060;
const FRAME_CTRL_TYPE_AGGREGATED: u16 = 0x0080;

// 7.1.2.1.5 Frame subtypes
// Beacon: Table 14
const FRAME_CTRL_SUBTYPE_BEACON_REGULAR: u16 = 0x0000; // 0b0000_xxxx_0000_0000
const FRAME_CTRL_SUBTYPE_BEACON_SIGNALLING: u16 = 0x0100;
const FRAME_CTRL_SUBTYPE_BEACON_ECHO: u16 = 0x0200;
const FRAME_CTRL_SUBTYPE_BEACON_RESERVED: u16 = 0x0300; // 3 -15

// Control: Table 21
const FRAME_CTRL_SUBTYPE_IMM_ACK: u16 = 0x0000 | FRAME_CTRL_TYPE_CTRL;
const FRAME_CTRL_SUBTYPE_B_ACK: u16 = 0x0100 | FRAME_CTRL_TYPE_CTRL;
const FRAME_CTRL_SUBTYPE_RTS: u16 = 0x0200 | FRAME_CTRL_TYPE_CTRL;
const FRAME_CTRL_SUBTYPE_CTS: u16 = 0x0300 | FRAME_CTRL_TYPE_CTRL;
const FRAME_CTRL_SUBTYPE_UCA: u16 = 0x0400 | FRAME_CTRL_TYPE_CTRL;
const FRAME_CTRL_SUBTYPE_UCR: u16 = 0x0E00 | FRAME_CTRL_TYPE_CTRL;
const FRAME_CTRL_SUBTYPE_RESRVED: u16 = 0x0F00 | FRAME_CTRL_TYPE_CTRL; // 6 - 13, 15
const FRAME_CTRL_SUBTYPE_APP_SPEC: u16 = 0x0000 | FRAME_CTRL_TYPE_CTRL;

// Command: Table 26
const FRAME_CTRL_SUBTYPE_CRP_RESRVN_REQ: u16 = 0x0000 | FRAME_CTRL_TYPE_CMD; // CRP reservation request
const FRAME_CTRL_SUBTYPE_CRP_RESRVN_RESP: u16 = 0x0100 | FRAME_CTRL_TYPE_CMD; // CRP reservation response
const FRAME_CTRL_SUBTYPE_PROBE: u16 = 0x0200 | FRAME_CTRL_TYPE_CMD;
const FRAME_CTRL_SUBTYPE_PTK: u16 = 0x0300 | FRAME_CTRL_TYPE_CMD; // Pair-wise Temporary Key
const FRAME_CTRL_SUBTYPE_GTK: u16 = 0x0400 | FRAME_CTRL_TYPE_CMD; // Group Temporary Key
const FRAME_CTRL_SUBTYPE_BEACON_PROMO_REQ: u16 = 0x0500 | FRAME_CTRL_TYPE_CMD; // Beaconing Promotion Request
const FRAME_CTRL_SUBTYPE_CH_MEAS_REQ: u16 = 0x0600 | FRAME_CTRL_TYPE_CMD; // Channel Measurement Request
const FRAME_CTRL_SUBTYPE_CH_MEAS_RESP: u16 = 0x0700 | FRAME_CTRL_TYPE_CMD; // Channel Measurement Response
const FRAME_CTRL_SUBTYPE_CH_MEAS_RPRT: u16 = 0x0800 | FRAME_CTRL_TYPE_CMD; // Channel Measurement Report
const FRAME_CTRL_SUBTYPE_CH_MEAS_RPRT_ACK: u16 = 0x0900 | FRAME_CTRL_TYPE_CMD; // Channel Measurement Report Acknowledgement
const FRAME_CTRL_SUBTYPE_CH_SWITCH_CMD: u16 = 0x0A00 | FRAME_CTRL_TYPE_CMD; // Channel Switch Command
const FRAME_CTRL_SUBTYPE_CH_SWITCH_RESP: u16 = 0x0B00 | FRAME_CTRL_TYPE_CMD; // Channel Switch Response
const FRAME_CTRL_SUBTYPE_SW_SLOT_RESP: u16 = 0x0C00 | FRAME_CTRL_TYPE_CMD; // SW Slot Response
const FRAME_CTRL_SUBTYPE_APP_SPEC: u16 = 0x0D00 | FRAME_CTRL_TYPE_CMD; // Application Specific
const FRAME_CTRL_SUBTYPE_ASSOC_REQ: u16 = 0x0E00 | FRAME_CTRL_TYPE_CMD; // Association Request
const FRAME_CTRL_SUBTYPE_RESERVED: u16 = 0x0F00 | FRAME_CTRL_TYPE_CMD; // Channel Switch Response

// 7.1.2.1.6 Retry
// 0b000x_0000_0000_0000
const FRAME_CTRL_RETRY: u16 = 0x1000;


// 7.1.2.4 Sequence Control

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