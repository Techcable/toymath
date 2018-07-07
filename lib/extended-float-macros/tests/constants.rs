extern crate extended_float_sys as sys;
extern crate extended_float_macros;

use extended_float_macros::extended_float;

use sys::ExtendedFloat as ExtendedFloat;

#[test]
fn integer() {
    assert_eq!(
        extended_float!(1),
        ExtendedFloat([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0xFF, 0x3F])
    )
}

#[test]
fn float() {
    assert_eq!(
        extended_float!(0.5),
        ExtendedFloat([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0xFE, 0x3F])
    )
}

#[test]
fn string() {
    assert_eq!(
        extended_float!("3.14159265358979323846264338327950288419716939937510582097494459230781640628620899862"),
        ExtendedFloat([0x00, 0xC0, 0x68, 0x21, 0xA2, 0xDA, 0x0F, 0xC9, 0x00, 0x40])
    );
}