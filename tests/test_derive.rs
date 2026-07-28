use borsh::{BorshDeserialize, BorshSerialize};
use borsh_clone::BorshClone;

#[derive(Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize, BorshClone)]
struct SimpleStruct {
    a: u32,
    b: String,
}

#[derive(Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize, BorshClone)]
struct TupleStruct(u64, bool);

#[derive(Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize, BorshClone)]
struct UnitStruct;

#[derive(Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize, BorshClone)]
enum TestEnum {
    A,
    B(u32),
    C { x: String, y: u8 },
}

#[derive(Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize, BorshClone)]
struct GenericStruct<T> {
    val: T,
}

mod reexport {
    pub use borsh;
}

#[derive(Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize, BorshClone)]
#[borsh(crate = "reexport::borsh")]
struct CustomCratePathStruct {
    x: i32,
}

struct UnserializableType;

#[derive(Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize, BorshClone)]
#[borsh_clone(skip_bounds)]
struct PhantomStruct<T> {
    val: u32,
    #[borsh(skip)]
    _marker: std::marker::PhantomData<T>,
}

#[derive(Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize, BorshClone)]
#[borsh_clone(bound = "T: BorshSerialize + BorshDeserialize")]
struct CustomBoundStruct<T> {
    val: T,
}

#[test]
fn test_simple_struct_clone() {
    let s = SimpleStruct {
        a: 42,
        b: "hello".to_string(),
    };
    let cloned = s.clone();
    assert_eq!(s, cloned);
}

#[test]
fn test_tuple_struct_clone() {
    let s = TupleStruct(12345, true);
    let cloned = s.clone();
    assert_eq!(s, cloned);
}

#[test]
fn test_unit_struct_clone() {
    let s = UnitStruct;
    let cloned = s.clone();
    assert_eq!(s, cloned);
}

#[test]
fn test_enum_clone() {
    let e1 = TestEnum::A;
    assert_eq!(e1, e1.clone());

    let e2 = TestEnum::B(99);
    assert_eq!(e2, e2.clone());

    let e3 = TestEnum::C {
        x: "world".to_string(),
        y: 7,
    };
    assert_eq!(e3, e3.clone());
}

#[test]
fn test_generic_struct_clone() {
    let g = GenericStruct { val: 100u32 };
    let cloned = g.clone();
    assert_eq!(g, cloned);
}

#[test]
fn test_custom_crate_path() {
    let s = CustomCratePathStruct { x: 777 };
    let cloned = s.clone();
    assert_eq!(s, cloned);
}

#[test]
fn test_phantom_struct_skip_bounds() {
    let p = PhantomStruct::<UnserializableType> {
        val: 123,
        _marker: std::marker::PhantomData,
    };
    let cloned = p.clone();
    assert_eq!(p.val, cloned.val);
}

#[test]
fn test_custom_bound_struct() {
    let c = CustomBoundStruct { val: 42u64 };
    let cloned = c.clone();
    assert_eq!(c, cloned);
}
