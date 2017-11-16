# tokio-fmt-encoder

Encode items that implement formatting traits like `Debug` and `Display`.

This might be useful for debugging streams. Note that there is no corresponding `Decoder`.

To encode an item that implements `Debug`:

```rust
extern crate bytes;
extern crate tokio_fmt_encoder;
extern crate tokio_io;

fn main() {
    use bytes::BytesMut;
    use std::fmt::Formatter;
    use tokio_fmt_encoder::DebugEncoder;
    use tokio_io::codec::Encoder;

    let to_encode = Some(10);
    let mut buffer: BytesMut = Default::default();
    let mut encoder: DebugEncoder<Option<usize>> = Default::default();
    encoder.encode(to_encode, &mut buffer).unwrap();
    assert_eq!(&buffer.take(), &"Some(10)\n");
}
```

To encode an item that implements `Display`:

```rust
extern crate bytes;
extern crate tokio_fmt_encoder;
extern crate tokio_io;

fn main() {
    use bytes::BytesMut;
    use std::fmt::Formatter;
    use tokio_fmt_encoder::DisplayEncoder;
    use tokio_io::codec::Encoder;

    let to_encode = String::from("hello");
    let mut buffer: BytesMut = Default::default();
    let mut encoder: DisplayEncoder<String> = Default::default();
    encoder.encode(to_encode, &mut buffer).unwrap();
    assert_eq!(&buffer.take(), &"hello\n");
}
```


License: MIT/Apache-2.0
