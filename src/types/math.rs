use crate::types::{Byte, HasValue, Word};
use std::ops::{BitAnd, BitOr};

fn logic_num<T, I>(this: T, other: T, method: fn(I, I) -> I) -> T
where
    T: HasValue<I> + From<I>,
{
    method(this.value(), other.value()).into()
}

fn math_num<T, I>(this: T, other: T, method: fn(I, I) -> (I, bool)) -> (T, bool)
where
    T: HasValue<I> + From<I>,
{
    let (result, overflow) = method(this.value(), other.value());
    (result.into(), overflow)
}

fn carrying_math_num<T, I>(
    this: T,
    other: T,
    carry: bool,
    method: fn(I, I) -> (I, bool),
) -> (T, bool)
where
    T: HasValue<I> + From<I> + From<bool>,
{
    let (partial, overflow1) = method(this.value(), other.value());
    let (result, overflow2) = method(partial, Into::<T>::into(carry).value());
    (result.into(), overflow1 | overflow2)
}

// LOGIC

pub fn and_byte(this: Byte, other: Byte, _carry: bool) -> (Byte, bool) {
    (logic_num(this, other, u8::bitand), false)
}

pub fn and_word(this: Word, other: Word, _carry: bool) -> (Word, bool) {
    (logic_num(this, other, u16::bitand), false)
}

pub fn or_byte(this: Byte, other: Byte, _carry: bool) -> (Byte, bool) {
    (logic_num(this, other, u8::bitor), false)
}

pub fn or_word(this: Word, other: Word, _carry: bool) -> (Word, bool) {
    (logic_num(this, other, u16::bitor), false)
}

pub fn xor_byte(this: Byte, other: Byte, _carry: bool) -> (Byte, bool) {
    (logic_num(this, other, u8::bitor), false)
}

pub fn xor_word(this: Word, other: Word, _carry: bool) -> (Word, bool) {
    (logic_num(this, other, u16::bitor), false)
}

// ADD

pub fn carrying_add_byte(this: Byte, other: Byte, carry: bool) -> (Byte, bool) {
    carrying_math_num(this, other, carry, u8::overflowing_add)
}

pub fn carrying_add_word(this: Word, other: Word, carry: bool) -> (Word, bool) {
    carrying_math_num(this, other, carry, u16::overflowing_add)
}

pub fn add_byte(this: Byte, other: Byte, _carry: bool) -> (Byte, bool) {
    math_num(this, other, u8::overflowing_add)
}

pub fn add_word(this: Word, other: Word, _carry: bool) -> (Word, bool) {
    math_num(this, other, u16::overflowing_add)
}

// SUB

pub fn carrying_sub_byte(this: Byte, other: Byte, carry: bool) -> (Byte, bool) {
    carrying_math_num(this, other, carry, u8::overflowing_sub)
}

pub fn carrying_sub_word(this: Word, other: Word, carry: bool) -> (Word, bool) {
    carrying_math_num(this, other, carry, u16::overflowing_sub)
}

pub fn sub_byte(this: Byte, other: Byte, _carry: bool) -> (Byte, bool) {
    math_num(this, other, u8::overflowing_sub)
}

pub fn sub_word(this: Word, other: Word, _carry: bool) -> (Word, bool) {
    math_num(this, other, u16::overflowing_sub)
}

// MUL

pub fn mul_byte(this: Byte, other: Byte, _carry: bool) -> (Byte, bool) {
    math_num(this, other, u8::overflowing_mul)
}

pub fn mul_word(this: Word, other: Word, _carry: bool) -> (Word, bool) {
    math_num(this, other, u16::overflowing_mul)
}

// DIV

pub fn div_byte(this: Byte, other: Byte, _carry: bool) -> (Byte, bool) {
    math_num(this, other, u8::overflowing_div)
}

pub fn div_word(this: Word, other: Word, _carry: bool) -> (Word, bool) {
    math_num(this, other, u16::overflowing_div)
}

// MULS

pub fn muls_byte(this: Byte, other: Byte, _carry: bool) -> (Byte, bool) {
    math_num(this, other, i8::overflowing_mul)
}

pub fn muls_word(this: Word, other: Word, _carry: bool) -> (Word, bool) {
    math_num(this, other, i16::overflowing_mul)
}

// DIVS

pub fn divs_byte(this: Byte, other: Byte, _carry: bool) -> (Byte, bool) {
    math_num(this, other, i8::overflowing_div)
}

pub fn divs_word(this: Word, other: Word, _carry: bool) -> (Word, bool) {
    math_num(this, other, i16::overflowing_div)
}
