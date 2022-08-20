// Copyright (c) TVWS Project

#![allow(unused)]

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
const ECMA392_FC_SECURE: u16 = 0x0004;

// 7.1.2.1.3 ACK Policy
// 0b0000_0000_000x_x000
const ECMA392_FC__ACK_POLICY_NO_ACK: u16 = 0x0000;
const ECMA392_FC__ACK_POLICY_IMM_ACK: u16 = 0x0008;
const ECMA392_FC__ACK_POLICY_B_ACK: u16 = 0x0010;
const ECMA392_FC__ACK_POLICY_B_ACK_REQ: u16 = 0x0018;

// 7.1.2.1.4 Frame Type
// 0b0000_0000_xxx0_0000
const ECMA392_FC_TYPE_BEACON: u16 = 0x0000;
const ECMA392_FC_TYPE_CTRL: u16 = 0x0020;
const ECMA392_FC_TYPE_CMD: u16 = 0x0040;
const ECMA392_FC_TYPE_DATA: u16 = 0x0060;
const ECMA392_FC_TYPE_AGGREGATED: u16 = 0x0080;

// 7.1.2.1.5 Frame subtypes
// Beacon: Table 14
const ECMA392_FC_STYPE_BEACON_REGULAR: u16 = 0x0000; // 0b0000_xxxx_0000_0000
const ECMA392_FC_STYPE_BEACON_SIGNALLING: u16 = 0x0100;
const ECMA392_FC_STYPE_BEACON_ECHO: u16 = 0x0200;
const ECMA392_FC_STYPE_BEACON_RESERVED: u16 = 0x0300; // 3 -15

// Control: Table 21
const ECMA392_FC_STYPE_IMM_ACK: u16 = 0x0000 | ECMA392_FC_TYPE_CTRL;
const ECMA392_FC_STYPE_B_ACK: u16 = 0x0100 | ECMA392_FC_TYPE_CTRL;
const ECMA392_FC_STYPE_RTS: u16 = 0x0200 | ECMA392_FC_TYPE_CTRL;
const ECMA392_FC_STYPE_CTS: u16 = 0x0300 | ECMA392_FC_TYPE_CTRL;
const ECMA392_FC_STYPE_UCA: u16 = 0x0400 | ECMA392_FC_TYPE_CTRL;
const ECMA392_FC_STYPE_UCR: u16 = 0x0E00 | ECMA392_FC_TYPE_CTRL;
const ECMA392_FC_STYPE_RESRVED: u16 = 0x0F00 | ECMA392_FC_TYPE_CTRL; // 6 - 13, 15
const ECMA392_FC_STYPE_APP_SPEC_CMD: u16 = 0x0000 | ECMA392_FC_TYPE_CTRL;

// Command: Table 26
const ECMA392_FC_STYPE_CRP_RESRVN_REQ: u16 = 0x0000 | ECMA392_FC_TYPE_CMD; // CRP reservation request
const ECMA392_FC_STYPE_CRP_RESRVN_RESP: u16 = 0x0100 | ECMA392_FC_TYPE_CMD; // CRP reservation response
const ECMA392_FC_STYPE_PROBE: u16 = 0x0200 | ECMA392_FC_TYPE_CMD;
const ECMA392_FC_STYPE_PTK: u16 = 0x0300 | ECMA392_FC_TYPE_CMD; // Pair-wise Temporary Key
const ECMA392_FC_STYPE_GTK: u16 = 0x0400 | ECMA392_FC_TYPE_CMD; // Group Temporary Key
const ECMA392_FC_STYPE_BEACON_PROMO_REQ: u16 = 0x0500 | ECMA392_FC_TYPE_CMD; // Beaconing Promotion Request
const ECMA392_FC_STYPE_CH_MEAS_REQ: u16 = 0x0600 | ECMA392_FC_TYPE_CMD; // Channel Measurement Request
const ECMA392_FC_STYPE_CH_MEAS_RESP: u16 = 0x0700 | ECMA392_FC_TYPE_CMD; // Channel Measurement Response
const ECMA392_FC_STYPE_CH_MEAS_RPRT: u16 = 0x0800 | ECMA392_FC_TYPE_CMD; // Channel Measurement Report
const ECMA392_FC_STYPE_CH_MEAS_RPRT_ACK: u16 = 0x0900 | ECMA392_FC_TYPE_CMD; // Channel Measurement Report Acknowledgement
const ECMA392_FC_STYPE_CH_SWITCH_CMD: u16 = 0x0A00 | ECMA392_FC_TYPE_CMD; // Channel Switch Command
const ECMA392_FC_STYPE_CH_SWITCH_RESP: u16 = 0x0B00 | ECMA392_FC_TYPE_CMD; // Channel Switch Response
const ECMA392_FC_STYPE_SW_SLOT_RESP: u16 = 0x0C00 | ECMA392_FC_TYPE_CMD; // SW Slot Response
const ECMA392_FC_STYPE_APP_SPEC_CTRL: u16 = 0x0D00 | ECMA392_FC_TYPE_CMD; // Application Specific
const ECMA392_FC_STYPE_ASSOC_REQ: u16 = 0x0E00 | ECMA392_FC_TYPE_CMD; // Association Request
const ECMA392_FC_STYPE_RESERVED: u16 = 0x0F00 | ECMA392_FC_TYPE_CMD; // Channel Switch Response

// 7.1.2.1.6 Retry
// 0b000x_0000_0000_0000
const ECMA392_FC_RETRY: u16 = 0x1000;

// 7.1.2.4 Sequence Control

// Section 7.1 MAC Frame
pub struct MacFrame {
    header: MacHeader,
    body: Option<MacFrameBody>,
}

#[derive(Debug)]
pub struct MacHeader {
    frame_ctrl_: u16,
    dest_addr: u16, //DevAddr of the frame recipient
    src_addr: u16,  // DevAddr of the transmitter of the frame
    seq_ctrl: u16,  // Order of MSDUs/MCDUs
    access_ctrl: u16,
}

// 7.2.1 Frame address
// DevAddr
//  - Generated locally within the device
//  - Frames are addressed using DevAddr
//  - Four types of DevAddrs: Private, Generated, Multicast, Broadcast
pub enum DevAddr {
    Private,      // 0x0000 - 0x00ff
    Generated,    // 0x0100 - 0xfeff
    Unassociated, // 0xff00
    McstAddr,     // 0xff01 - 0xfffe
    BcstAddr,     // 0xffff
}

fn device_addr_type(addr: u16) -> DevAddr {
    match addr {
        0x0000..=0x00ff => DevAddr::Private,
        0x0100..=0xfeff => DevAddr::Generated,
        0xff00 => DevAddr::Unassociated,
        0xff01..=0xfffe => DevAddr::McstAddr,
        0xffff => DevAddr::BcstAddr
    }
}

pub struct MacFrameBody {
    payload: PayloadType,
    fcs: u32, // Frame Check Sequence
}

pub struct SecureFramePayload {
    tkid: [u8; 3], // Temporary Identifier
    payload: Vec<u8>,
    mic: u64, // Message Integrity Code
}

pub enum PayloadType {
    Secure(SecureFramePayload),
    NotSecure(Vec<u8>),
}
