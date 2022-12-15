## ECMA-392
MAC implementation document.

## MAC Layer

### Frame Formats
![image](https://user-images.githubusercontent.com/3691485/185741495-a22bb397-8ff6-4a79-8caa-b33aede7ea37.png)

The ECMA392 MAC frame is made up of a fixed-length header (80bits) and an optional variable-length payload. 
```rust
struct MacFrame {
    header: MacHeader,
    body: Option<MacFrameBody>,
}
```
The ECMA 392 MAC header is 80bits in size.
```rust
struct MacHeader {
    frame_ctrl_: u16,
    dest_addr: u16, //DevAddr of the frame recipient
    src_addr: u16,  // DevAddr of the transmitter of the frame
    seq_ctrl: u16,  // Order of MSDUs/MCDUs
    access_ctrl: u16,
}
```
