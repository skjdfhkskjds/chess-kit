use chess_kit_derive::Arithmetic;

#[derive(Arithmetic, Copy, Clone, Debug, PartialEq, Eq)]
struct Scalar(i32);

#[test]
fn arithmetic_uses_the_wrapped_primitive_as_the_rhs() {
    let value = Scalar(10);

    assert_eq!(value + 20, Scalar(30));
    assert_eq!(value - 5, Scalar(5));
    assert_eq!(value * 2, Scalar(20));
}
