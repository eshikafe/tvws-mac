## ECMA-392 MAC implementation document

### Reference
[MAC and PHY for operation in TV white space, 2nd edition, June 2012](https://www.ecma-international.org/wp-content/uploads/ECMA-392_2nd_edition_june_2012.pdf)

## MAC Layer

### Frame Formats
![image](https://user-images.githubusercontent.com/3691485/185741495-a22bb397-8ff6-4a79-8caa-b33aede7ea37.png)

The MAC header data structure.
```rust
struct MacHeader {
    frame_ctrl_: u16,
    dest_addr: u16, //DevAddr of the frame recipient
    src_addr: u16,  // DevAddr of the transmitter of the frame
    seq_ctrl: u16,  // Order of MSDUs/MCDUs
    access_ctrl: u16,
}
```

The MAC frame consists of a fixed-length header (80bits) and an optional variable-length payload. 
```rust
struct MacFrame {
    header: MacHeader,
    body: Option<MacFrameBody>,
}
```

