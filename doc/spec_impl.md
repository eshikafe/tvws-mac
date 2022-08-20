# Implementation Document

## MAC Layer

### Frame Formats
![image](https://user-images.githubusercontent.com/3691485/185741495-a22bb397-8ff6-4a79-8caa-b33aede7ea37.png)

### MAC layer data structures

The MAC frame is made up of a fixed-length heade (80bits) and an optional variable-length frame payload. 
```rust
struct MacFrame {
    header: MacHeader,
    body: Option<MacFrameBody>,
}
```

The MAC header is 80bits in size.
```rust
struct MacHeader {
    frame_ctrl_: u16,
    dest_addr: u16, //DevAddr of the frame recipient
    src_addr: u16,  // DevAddr of the transmitter of the frame
    seq_ctrl: u16,  // Order of MSDUs/MCDUs
    access_ctrl: u16,
}
```
