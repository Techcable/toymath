#![feature(proc_macro, const_fn, ptr_offset_from)]

extern crate proc_macro;
extern crate extended_float_sys as sys;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use std::{mem, ptr};
use std::ffi::CString;

use syn::{LitInt, IntSuffix, LitStr, LitFloat};
use syn::synom::Synom;

use proc_macro::TokenStream;

enum ExtendedFloatLiteral {
    UnsignedInteger(u64),
    Integer(i64),
    Float(f64),
    String(String)
}
impl ExtendedFloatLiteral {
    pub fn parse(&self) -> sys::ExtendedFloat {
        unsafe  {
            let mut out = mem::uninitialized();
            match *self {
                ExtendedFloatLiteral::UnsignedInteger(value) => {
                    sys::extended_convert_from_u64(&mut out, value);
                },
                ExtendedFloatLiteral::Integer(value) => {
                    sys::extended_convert_from_i64(&mut out, value);
                },
                ExtendedFloatLiteral::Float(value) => {
                    sys::extended_convert_from_f64(&mut out, value);
                },
                ExtendedFloatLiteral::String(ref value) => {
                    assert!(!value.is_empty(), "Empty literal string!");
                    let first = value.chars().next().unwrap();
                    assert!(!first.is_whitespace(), "Literal starts with whitespace: {:?}", value);
                    let native = CString::new(value.as_bytes())
                        .unwrap_or_else(|_| panic!("Literal contains null byte: {:?}", value));
                    let mut end = ptr::null_mut();
                    let expected_end = native.as_ptr().add(value.len());
                    sys::extended_parse(&mut out,native.as_ptr(), &mut end);
                    if (end as *const _) != expected_end {
                        let consumed = end.offset_from(native.as_ptr());
                        panic!("Only parsed {} of {:?}", consumed, value)
                    }
                }
            }
            out
        }
    }
}
impl Synom for ExtendedFloatLiteral {
    named!(parse -> Self, alt!(
        syn!(LitInt) => { |lit| match lit.suffix() {
            IntSuffix::None | IntSuffix::I64 => ExtendedFloatLiteral::Integer(lit.value() as i64),
            IntSuffix::U64 => ExtendedFloatLiteral::UnsignedInteger(lit.value() as u64),
            _ => panic!("Invalid suffix {:?}", lit.suffix())
        } } |
        negative_int => { |value| ExtendedFloatLiteral::Integer(value) } |
        signed_float => { |value| ExtendedFloatLiteral::Float(value) } |
        syn!(LitStr) => { |lit| ExtendedFloatLiteral::String(lit.value()) }
    ));
}
named!(signed_float -> f64, alt!(
    negative_float |
    syn!(LitFloat) => { |lit| lit.value() }
));
named!(negative_float -> f64, do_parse!(
    punct!(-) >>
    lit: syn!(LitFloat) >>
    (-lit.value())
));
named!(negative_int -> i64, do_parse!(
    punct!(-) >>
    lit: syn!(LitInt) >>
    (-(lit.value() as i64))
));

fn emit(value: sys::ExtendedFloat) -> TokenStream {
    let bytes = &value.0;
    quote!(ExtendedFloat::from_bits([#(#bytes),*])).into()
}

#[proc_macro]
pub fn extended_float(input: TokenStream) -> TokenStream {
    let literal = ::syn::parse::<ExtendedFloatLiteral>(input.clone())
        .unwrap_or_else(|_| panic!("Invalid literal: {:?}", input));
    emit(literal.parse())
}