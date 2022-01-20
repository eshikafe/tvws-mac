// Copyright (c) 2021, TVWS-Project
//
// IEEE 802.11af MAC Frames module
// Based on IEEE standard 802.11-2012 and 802.11af-2013

#![allow(unused)]
//  See 8.2.4.1 of IEEE 802.11-2012
//  Frame control fields [b15 ... b0]
//  b0..b1: Protocol version {0}
//  b2..b3: Type {Management, Control & Data}
//  b4..b7: Subtype
//  b8    : To DS
//  b9    : From DS
//  b10   : More Fragments
//  b11   : Retry
//  b12   : Power Management
//  b13   : More data
//  b14   : Protected data
//  b15   : Order

// See 8.2.4.1.3 (IEEE 802.11-2012)
// Frame Control Type
pub const IEEE80211AF_FC_TYPE_MGT: u16 = 0x0000;
pub const IEEE80211AF_FC_TYPE_CTRL: u16 = 0x0004;
pub const IEEE80211AF_FC_TYPE_DATA: u16 = 0x0008;

// Frame Control Management Subtype
pub const IEEE80211AF_FC_STYPE_ASSOC_REQ: u16 = 0x0000;
pub const IEEE80211_FC_STYPE_ASSOC_RESP: u16 = 0x0010;
pub const IEEE80211_FC_STYPE_REASSOC_REQ: u16 = 0x0020;
pub const IEEE80211_FC_STYPE_REASSOC_RESP: u16 = 0x0030;
pub const IEEE80211_FC_STYPE_PROBE_REQ: u16 = 0x0040;
pub const IEEE80211_FC_STYPE_PROBE_RESP: u16 = 0x0050;
pub const IEEE80211_FC_STYPE_BEACON: u16 = 0x0080;
pub const IEEE80211_FC_STYPE_ATIM: u16 = 0x0090;
pub const IEEE80211_FC_STYPE_DISASSOC: u16 = 0x00A0;
pub const IEEE80211_FC_STYPE_AUTH: u16 = 0x00B0;
pub const IEEE80211_FC_STYPE_DEAUTH: u16 = 0x00C0;
pub const IEEE80211_FC_STYPE_ACTION: u16 = 0x00D0;

// Frame Control Control Subtype
pub const IEEE80211_FC_STYPE_CTL_EXT: u16 = 0x0060;
pub const IEEE80211_FC_STYPE_BACK_REQ: u16 = 0x0080;
pub const IEEE80211_FC_STYPE_BACK: u16 = 0x0090;
pub const IEEE80211_FC_STYPE_PSPOLL: u16 = 0x00A0;
pub const IEEE80211_FC_STYPE_RTS: u16 = 0x00B0;
pub const IEEE80211_FC_STYPE_CTS: u16 = 0x00C0;
pub const IEEE80211_FC_STYPE_ACK: u16 = 0x00D0;
pub const IEEE80211_FC_STYPE_CFEND: u16 = 0x00E0;
pub const IEEE80211_FC_STYPE_CFENDACK: u16 = 0x00F0;

// Frame control Data Subtype
pub const IEEE80211_FC_STYPE_DATA: u16 = 0x0000;
pub const IEEE80211_FC_STYPE_DATA_CFACK: u16 = 0x0010;
pub const IEEE80211_FC_STYPE_DATA_CFPOLL: u16 = 0x0020;
pub const IEEE80211_FC_STYPE_DATA_CFACKPOLL: u16 = 0x0030;
pub const IEEE80211_FC_STYPE_NULLFUNC: u16 = 0x0040;
pub const IEEE80211_FC_STYPE_CFACK: u16 = 0x0050;
pub const IEEE80211_FC_STYPE_CFPOLL: u16 = 0x0060;
pub const IEEE80211_FC_STYPE_CFACKPOLL: u16 = 0x0070;
pub const IEEE80211_FC_STYPE_QOS_DATA: u16 = 0x0080;
pub const IEEE80211_FC_STYPE_QOS_DATA_CFACK: u16 = 0x0090;
pub const IEEE80211_FC_STYPE_QOS_DATA_CFPOLL: u16 = 0x00A0;
pub const IEEE80211_FC_STYPE_QOS_DATA_CFACKPOLL: u16 = 0x00B0;
pub const IEEE80211_FC_STYPE_QOS_NULLFUNC: u16 = 0x00C0;
pub const IEEE80211_FC_STYPE_QOS_CFACK: u16 = 0x00D0;
pub const IEEE80211_FC_STYPE_QOS_CFPOLL: u16 = 0x00E0;
pub const IEEE80211_FC_STYPE_QOS_CFACKPOLL: u16 = 0x00F0;

// 8.2 MAC frame
#[derive(Debug)]
pub struct MacHeader {
    frame_ctrl: u16,
    duration_id: u16,
    addr1: [u8; 6], // MAC address EUI-48
    addr2: [u8; 6],
    addr3: [u8; 6],
    seq_ctrl: u16,
    addr4: [u8; 6],
    qos_ctrl: u16,
    ht_ctrl: u32, // VHT
}

pub struct MacFrame {
    hdr: MacHeader,
    data: [u8; 7951],
    fcs: u32,
}
