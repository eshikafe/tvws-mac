## IEEE 802.11af MAC implementation document.

### Frame Formats
![image](https://user-images.githubusercontent.com/3691485/185742135-b0f103a5-024d-4242-a539-b1e90ea78b2d.png)

The MAC header data structure.
```rust
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
```

The MAC frame data structure:
```rust
pub struct MacFrame {
    hdr: MacHeader,
    data: [u8; 7951],
    fcs: u32,
}
```