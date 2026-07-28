# borsh-clone

`borsh-clone` provides a procedural derive macro `BorshClone` for Rust that implements `Clone` for a type by serializing and deserializing it using [`borsh`](https://crates.io/crates/borsh).

## Usage

Add `borsh` and `borsh-clone` to your `Cargo.toml`:

```toml
[dependencies]
borsh = { version = "1.5", features = ["derive"] }
borsh-clone = "0.1"
```

### Basic Example

```rust
use borsh::{BorshDeserialize, BorshSerialize};
use borsh_clone::BorshClone;

#[derive(Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize, BorshClone)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 10, y: 20 };
    let p2 = p1.clone();
    assert_eq!(p1, p2);
}
```

### Custom `borsh` Crate Path

If `borsh` is re-exported or available under a non-standard module path:

```rust
#[derive(BorshSerialize, BorshDeserialize, BorshClone)]
#[borsh(crate = "path::to::borsh")]
struct MyStruct {
    data: Vec<u8>,
}
```

### Custom Bounds and Skipping Bounds

For generic types where type parameters do not require `BorshSerialize` or `BorshDeserialize` (such as `PhantomData` fields):

```rust
#[derive(BorshSerialize, BorshDeserialize, BorshClone)]
#[borsh_clone(skip_bounds)]
struct PhantomStruct<T> {
    id: u64,
    #[borsh(skip)]
    _marker: std::marker::PhantomData<T>,
}
```

Or specify custom bounds:

```rust
#[derive(BorshSerialize, BorshDeserialize, BorshClone)]
#[borsh_clone(bound = "T: BorshSerialize + BorshDeserialize")]
struct CustomBoundStruct<T> {
    val: T,
}
```

## License

Dual-licensed under MIT or Apache-2.0.
