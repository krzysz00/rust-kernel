#![crate_name = "rlibm"]
#![crate_type = "lib"]

#![no_std]
#![feature(no_std, core)]

#![no_builtins]

#![allow(unused_variables)]

extern crate core;

#[no_mangle]
pub extern fn ceil(arg: f64) -> f64 {
    core::num::Float::nan()
}

#[no_mangle]
pub extern fn ceilf(arg: f32) -> f32 {
    core::num::Float::nan()
}

#[no_mangle]
pub extern fn exp(arg: f64) -> f64 {
    core::num::Float::nan()
}
#[no_mangle]
pub extern fn expf(arg: f32) -> f32 {
    core::num::Float::nan()
}

#[no_mangle]
pub extern fn exp2(arg: f64) -> f64 {
    core::num::Float::nan()
}
#[no_mangle]
pub extern fn exp2f(arg: f32) -> f32 {
    core::num::Float::nan()
}

#[no_mangle]
pub extern fn floor(arg: f64) -> f64 {
    core::num::Float::nan()
}

#[no_mangle]
pub extern fn floorf(arg: f32) -> f32 {
    core::num::Float::nan()
}

#[no_mangle]
pub extern fn fma(a: f64, b: f64, c: f64) -> f64 {
    a * b + c
}
#[no_mangle]
pub extern fn fmaf(a: f32, b: f32, c: f32) -> f32 {
    a * b + c
}

#[no_mangle]
pub extern fn fmod(arga: f64, argb: f64) -> f64 {
    core::num::Float::nan()
}

#[no_mangle]
pub extern fn fmodf(arga: f32, argb: f32) -> f32 {
    core::num::Float::nan()
}

#[no_mangle]
pub extern fn log(arg: f64) -> f64 {
    core::num::Float::nan()
}
#[no_mangle]
pub extern fn logf(arg: f32) -> f32 {
    core::num::Float::nan()
}

#[no_mangle]
pub extern fn log10(arg: f64) -> f64 {
    core::num::Float::nan()
}
#[no_mangle]
pub extern fn log10f(arg: f32) -> f32 {
    core::num::Float::nan()
}

#[no_mangle]
pub extern fn log2(arg: f64) -> f64 {
    core::num::Float::nan()
}
#[no_mangle]
pub extern fn log2f(arg: f32) -> f32 {
    core::num::Float::nan()
}

#[no_mangle]
pub extern fn pow(arga: f64, argb: f64) -> f64 {
    core::num::Float::nan()
}
#[no_mangle]
pub extern fn powf(arg: f32) -> f32 {
    core::num::Float::nan()
}

#[no_mangle]
pub extern fn round(arg: f64) -> f64 {
    core::num::Float::nan()
}
#[no_mangle]
pub extern fn roundf(arg: f32) -> f32 {
    core::num::Float::nan()
}

#[no_mangle]
pub extern fn trunc(arg: f64) -> f64 {
    core::num::Float::nan()
}
#[no_mangle]
pub extern fn truncf(arg: f32) -> f32 {
    core::num::Float::nan()
}
