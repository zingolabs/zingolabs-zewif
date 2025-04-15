#[macro_export]
macro_rules! test_envelope_roundtrip {
    // Only type parameter - use defaults for iterations and print
    ($type:ty) => {
        test_envelope_roundtrip!($type, 20, false);
    };

    // Type and iterations parameters - use default for print
    ($type:ty, $iterations:expr) => {
        test_envelope_roundtrip!($type, $iterations, false);
    };

    // Type, iterations, and print parameters
    ($type:ty, $iterations:expr, $print:expr) => {
        #[test]
        fn test_envelope() {
            $crate::test_envelope_roundtrip::<$type>($iterations, $print);
        }
    };

    // Type, iterations, print, and custom function name
    ($type:ty, $iterations:expr, $print:expr, $name:ident) => {
        #[test]
        fn $name() {
            $crate::test_envelope_roundtrip::<$type>($iterations, $print);
        }
    };
}

#[macro_export]
macro_rules! test_cbor_roundtrip {
    // Only type parameter - use defaults for iterations and print
    ($type:ty) => {
        test_cbor_roundtrip!($type, 20, false);
    };

    // Type and iterations parameters - use default for print
    ($type:ty, $iterations:expr) => {
        test_cbor_roundtrip!($type, $iterations, false);
    };

    // Type, iterations, and print parameters
    ($type:ty, $iterations:expr, $print:expr) => {
        #[test]
        fn test_cbor() {
            $crate::test_cbor_roundtrip::<$type>($iterations, $print);
        }
    };

    // Type, iterations, print, and custom function name
    ($type:ty, $iterations:expr, $print:expr, $name:ident) => {
        #[test]
        fn $name() {
            $crate::test_cbor_roundtrip::<$type>($iterations, $print);
        }
    };
}
