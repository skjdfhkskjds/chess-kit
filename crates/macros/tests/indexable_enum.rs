use chess_kit_derive::IndexableEnum;

#[derive(IndexableEnum, Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
enum File {
    A,
    B,
    C,
    D,
}

#[test]
fn converts_indices_and_indexes_arrays() {
    assert_eq!(File::from_idx(0), File::A);
    assert_eq!(File::C.idx(), 2);
    assert_eq!(File::from_idx(3), File::D);
    assert_eq!(File::from_idx_safe(4), None);

    let mut labels = ["a", "b", "c", "d"];
    assert_eq!(labels[File::B], "b");
    labels[File::B] = "B";
    assert_eq!(labels[File::B], "B");
}

#[test]
#[should_panic(expected = "enum index out of range")]
fn from_idx_rejects_invalid_indices() {
    let _ = File::from_idx(4);
}
