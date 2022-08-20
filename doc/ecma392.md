# ECMA 392 Implementation Document

## MAC Layer

### Frame Format
![image](https://user-images.githubusercontent.com/3691485/185741495-a22bb397-8ff6-4a79-8caa-b33aede7ea37.png)

### MAC layer data structures
```rust
pub struct MacFrame {
    header: MacHeader,
    body: Option<MacFrameBody>,
}
```
