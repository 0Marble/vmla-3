use std::{
    cmp::Ordering,
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

#[derive(Clone, Debug)]
pub struct LongInt {
    digits: Vec<u8>,
    positive: bool,
}

#[allow(dead_code)]
impl LongInt {
    pub fn new() -> Self {
        Self {
            digits: Vec::new(),
            positive: true,
        }
    }

    pub fn get(&self, ind: usize) -> u8 {
        if ind >= self.digits.len() {
            0
        } else {
            self.digits[ind]
        }
    }

    pub fn set(&mut self, ind: usize, d: u8) {
        if self.digits.len() <= ind {
            self.digits.resize(ind + 1, 0);
        }
        self.digits[ind] = d;
    }

    pub fn abs(&self) -> Self {
        Self {
            digits: self.digits.clone(),
            positive: true,
        }
    }

    fn shift_left(&mut self, by_digits: usize) {
        let old_len = self.digits.len();
        self.digits.resize(old_len + by_digits, 0);

        for i in 0..old_len {
            let i = old_len - i - 1;
            self.digits[i + by_digits] = self.digits[i];
        }
    }

    fn shift_right(&mut self, by_digits: usize) {
        let len = self.digits.len();

        let mut copy = 0;
        for i in by_digits..len {
            let i = len - i - 1;
            let t = self.digits[i - by_digits];
            self.digits[i] = copy;
            self.digits[i - by_digits] = self.digits[i];
            copy = t;
        }
    }

    fn bit_shift_left(&mut self, by_bits: usize) {
        let digit_shift = by_bits >> 3;
        let bit_shift = by_bits & 3;
        self.shift_left(digit_shift);

        let old_len = self.digits.len();
        self.digits.resize(old_len + 1, 0);

        for i in 0..old_len {
            let i = old_len - i - 1;
            let shifted = (self.digits[i] as u16) << bit_shift;
            let carry = (shifted >> 8) as u8;
            self.digits[i + 1] |= carry;
            self.digits[i] = (shifted & u8::MAX as u16) as u8;
        }
    }

    fn bit_shift_right(&mut self, by_bits: usize) {
        let digit_shift = by_bits >> 3;
        let bit_shift = by_bits & 3;
        self.shift_right(digit_shift);

        let old_len = self.digits.len();
        self.digits.resize(old_len + 1, 0);

        let mut carry = 0;
        for i in 0..old_len {
            let i = old_len - i - 1;
            let shifted = (self.digits[i] as u16) << (8 - bit_shift);
            self.digits[i] = (shifted >> 8) as u8 | carry;
            carry = (shifted & u8::MAX as u16) as u8;
        }
    }

    fn actual_length(&self) -> usize {
        for i in 0..self.digits.len() {
            let i = self.digits.len() - i - 1;
            if self.digits[i] != 0 {
                return i + 1;
            }
        }

        return 0;
    }

    fn get_bit(&self, bit: usize) -> bool {
        let digit = bit / 8;
        let bit = bit - digit * 8;
        let mask = (1 << bit) as u8;

        if digit >= self.digits.len() {
            return false;
        }

        return (self.digits[digit] & mask) >> bit == 1;
    }

    fn set_bit(&mut self, bit: usize, val: bool) {
        let digit = bit / 8;
        let bit = bit - digit * 8;
        let mask = (1 << bit) as u8;
        if digit >= self.digits.len() {
            self.digits.resize(digit + 1, 0);
        }

        self.digits[digit] &= !mask;
        if val {
            self.digits[digit] |= mask;
        }
    }

    fn trim(&mut self) {
        for i in 0..self.digits.len() {
            let i = self.digits.len() - i - 1;
            if self.digits[i] != 0 {
                self.digits.resize(i + 1, 0);
                return;
            }
        }
    }

    pub fn to_decimal(&self) -> String {
        if self == &0.into() {
            return "0".to_owned();
        }

        let mut div = self.abs();
        let mut s = String::new();

        while &div > &0.into() {
            let digit;
            (div, digit) = div_ignore_sign(&div, &10.into());
            s.push(digit.get(0).to_string().chars().next().unwrap());
        }

        let mut res = if self >= &0.into() {
            String::new()
        } else {
            "-".to_owned()
        };
        res += &s.chars().rev().collect::<String>();

        res
    }

    fn hex_digit(dec: u8) -> char {
        match dec {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            10 => 'A',
            11 => 'B',
            12 => 'C',
            13 => 'D',
            14 => 'E',
            15 => 'F',
            _ => unreachable!(),
        }
    }

    pub fn to_hex(&self) -> String {
        if self == &0.into() {
            return "|00|".to_owned();
        }

        let mut s = if self >= &0.into() {
            "|".to_owned()
        } else {
            "-|".to_owned()
        };

        for d in &self.digits {
            let lo = d % 16;
            let hi = d / 16;
            s.push(Self::hex_digit(hi));
            s.push(Self::hex_digit(lo));
            s.push('|');
        }

        s
    }
}

impl Display for LongInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "{}", self.to_hex())
        write!(f, "{}", self.to_decimal())
    }
}

fn add_ignore_sign(a: &LongInt, b: &LongInt) -> LongInt {
    let len = usize::max(a.digits.len(), b.digits.len());
    let mut v = Vec::with_capacity(len);
    let mut carry = 0u16;
    for i in 0..len {
        let sum = a.get(i) as u16 + b.get(i) as u16 + carry;
        let digit = (sum & u8::MAX as u16) as u8;
        carry = sum >> 8;

        v.push(digit);
    }

    if carry != 0 {
        v.push(carry as u8);
    }

    let mut res = LongInt {
        digits: v,
        positive: true,
    };
    res.trim();
    res
}

fn sub_ignore_sign(a: &LongInt, b: &LongInt) -> LongInt {
    match ord_ignore_sign(&a, &b).unwrap() {
        Ordering::Less => return -sub_ignore_sign(b, a),
        Ordering::Equal => return 0.into(),
        Ordering::Greater => {}
    }

    let len = usize::max(a.digits.len(), b.digits.len());
    let mut b = b.clone();

    let mut v = Vec::with_capacity(len);
    for i in 0..len {
        if a.get(i) < b.get(i) {
            let mut carry = Vec::with_capacity(i + 2);
            carry.resize(i + 2, 0);
            carry[i + 1] = 1;
            b = add_ignore_sign(
                &b,
                &LongInt {
                    digits: carry,
                    positive: true,
                },
            );
            v.push((a.get(i) as u16 + u8::MAX as u16 - b.get(i) as u16) as u8);
        } else {
            v.push(a.get(i) - b.get(i));
        }
    }

    let mut res = LongInt {
        digits: v,
        positive: true,
    };
    res.trim();
    res
}

fn mul_ignore_sign(a: &LongInt, b: &LongInt) -> LongInt {
    let mut res = 0.into();

    for i in 0..b.digits.len() {
        let d = b.get(i);
        let mut c = Vec::with_capacity(i + a.digits.len() + 1);
        c.resize(i + a.digits.len() + 1, 0);

        let mut carry = 0u16;
        for j in 0..a.digits.len() {
            let mul = a.get(j) as u16 * d as u16 + carry;
            c[i + j] = (mul & u8::MAX as u16) as u8;
            carry = mul >> 8;
        }
        c[i + a.digits.len()] = carry as u8;
        let c = LongInt {
            digits: c,
            positive: true,
        };
        res = &res + &c;
    }

    res.trim();

    res
}

fn div_ignore_sign(n: &LongInt, d: &LongInt) -> (LongInt, LongInt) {
    if d == &LongInt::from(0) {
        panic!("Division by zero!")
    }
    if ord_ignore_sign(n, d) == Some(Ordering::Less) {
        return (0.into(), n.clone());
    }

    let mut r: LongInt = 0.into();
    let mut q: LongInt = 0.into();
    let len_in_bits = n.actual_length() * 8;

    for i in 0..len_in_bits {
        let i = len_in_bits - i - 1;
        r.bit_shift_left(1);
        r.set_bit(0, n.get_bit(i));

        if &r >= &d {
            r = &r - d;
            q.set_bit(i, true);
        }
    }

    q.trim();
    r.trim();
    (q, r)
}

impl Neg for LongInt {
    type Output = LongInt;

    fn neg(self) -> Self::Output {
        LongInt {
            digits: self.digits,
            positive: !self.positive,
        }
    }
}

impl Neg for &LongInt {
    type Output = LongInt;

    fn neg(self) -> Self::Output {
        LongInt {
            digits: self.digits.clone(),
            positive: !self.positive,
        }
    }
}

impl PartialEq for LongInt {
    fn eq(&self, other: &Self) -> bool {
        self.digits == other.digits && self.positive == other.positive
    }
}

fn ord_ignore_sign(a: &LongInt, b: &LongInt) -> Option<Ordering> {
    let len = usize::max(a.digits.len(), b.digits.len());

    for i in 0..len {
        let i = len - i - 1;
        if let Some(c) = a.get(i).partial_cmp(&b.get(i)) {
            match c {
                Ordering::Less => return Some(Ordering::Less),
                Ordering::Greater => return Some(Ordering::Greater),
                Ordering::Equal => {}
            }
        }
    }

    Some(Ordering::Equal)
}

impl PartialOrd for LongInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.positive && !other.positive {
            return Some(Ordering::Greater);
        }

        if !self.positive && other.positive {
            return Some(Ordering::Less);
        }

        ord_ignore_sign(self, other)
    }
}

impl From<i32> for LongInt {
    fn from(x: i32) -> Self {
        Self {
            digits: x.abs().to_le_bytes().to_vec(),
            positive: x >= 0,
        }
    }
}

impl Add<&LongInt> for &LongInt {
    type Output = LongInt;

    fn add(self, rhs: &LongInt) -> Self::Output {
        if self.positive && rhs.positive {
            add_ignore_sign(&self, &rhs)
        } else if self.positive && !rhs.positive {
            sub_ignore_sign(&self, &rhs)
        } else if !self.positive && rhs.positive {
            -sub_ignore_sign(&self, &rhs)
        } else {
            -add_ignore_sign(&self, &rhs)
        }
    }
}

impl Sub<&LongInt> for &LongInt {
    type Output = LongInt;

    fn sub(self, rhs: &LongInt) -> Self::Output {
        if self.positive && rhs.positive {
            sub_ignore_sign(&self, &rhs)
        } else if self.positive && !rhs.positive {
            add_ignore_sign(&self, &rhs)
        } else if !self.positive && rhs.positive {
            -add_ignore_sign(&self, &rhs)
        } else {
            -sub_ignore_sign(&self, &rhs)
        }
    }
}

impl Mul<&LongInt> for &LongInt {
    type Output = LongInt;

    fn mul(self, rhs: &LongInt) -> Self::Output {
        if self.positive && rhs.positive {
            mul_ignore_sign(&self, &rhs)
        } else if self.positive && !rhs.positive {
            -mul_ignore_sign(&self, &rhs)
        } else if !self.positive && rhs.positive {
            -mul_ignore_sign(&self, &rhs)
        } else {
            mul_ignore_sign(&self, &rhs)
        }
    }
}

impl Div<&LongInt> for &LongInt {
    type Output = LongInt;

    fn div(self, rhs: &LongInt) -> Self::Output {
        if self.positive && rhs.positive {
            div_ignore_sign(&self, &rhs).0
        } else if self.positive && !rhs.positive {
            -div_ignore_sign(&self, &rhs).0
        } else if !self.positive && rhs.positive {
            -div_ignore_sign(&self, &rhs).0
        } else {
            div_ignore_sign(&self, &rhs).0
        }
    }
}

impl Add<LongInt> for &LongInt {
    type Output = LongInt;

    fn add(self, rhs: LongInt) -> Self::Output {
        if self.positive && rhs.positive {
            add_ignore_sign(self, &rhs)
        } else if self.positive && !rhs.positive {
            sub_ignore_sign(self, &rhs)
        } else if !self.positive && rhs.positive {
            -sub_ignore_sign(self, &rhs)
        } else {
            -add_ignore_sign(self, &rhs)
        }
    }
}

impl Sub<LongInt> for &LongInt {
    type Output = LongInt;

    fn sub(self, rhs: LongInt) -> Self::Output {
        if self.positive && rhs.positive {
            sub_ignore_sign(self, &rhs)
        } else if self.positive && !rhs.positive {
            add_ignore_sign(self, &rhs)
        } else if !self.positive && rhs.positive {
            -add_ignore_sign(self, &rhs)
        } else {
            -sub_ignore_sign(self, &rhs)
        }
    }
}

impl Mul<LongInt> for &LongInt {
    type Output = LongInt;

    fn mul(self, rhs: LongInt) -> Self::Output {
        if self.positive && rhs.positive {
            mul_ignore_sign(self, &rhs)
        } else if self.positive && !rhs.positive {
            -mul_ignore_sign(self, &rhs)
        } else if !self.positive && rhs.positive {
            -mul_ignore_sign(self, &rhs)
        } else {
            mul_ignore_sign(self, &rhs)
        }
    }
}

impl Div<LongInt> for &LongInt {
    type Output = LongInt;

    fn div(self, rhs: LongInt) -> Self::Output {
        if self.positive && rhs.positive {
            div_ignore_sign(self, &rhs).0
        } else if self.positive && !rhs.positive {
            -div_ignore_sign(self, &rhs).0
        } else if !self.positive && rhs.positive {
            -div_ignore_sign(self, &rhs).0
        } else {
            div_ignore_sign(self, &rhs).0
        }
    }
}

impl Add<&LongInt> for LongInt {
    type Output = LongInt;

    fn add(self, rhs: &LongInt) -> Self::Output {
        if self.positive && rhs.positive {
            add_ignore_sign(&self, &rhs)
        } else if self.positive && !rhs.positive {
            sub_ignore_sign(&self, &rhs)
        } else if !self.positive && rhs.positive {
            -sub_ignore_sign(&self, &rhs)
        } else {
            -add_ignore_sign(&self, &rhs)
        }
    }
}

impl Sub<&LongInt> for LongInt {
    type Output = LongInt;

    fn sub(self, rhs: &LongInt) -> Self::Output {
        if self.positive && rhs.positive {
            sub_ignore_sign(&self, &rhs)
        } else if self.positive && !rhs.positive {
            add_ignore_sign(&self, &rhs)
        } else if !self.positive && rhs.positive {
            -add_ignore_sign(&self, &rhs)
        } else {
            -sub_ignore_sign(&self, &rhs)
        }
    }
}

impl Mul<&LongInt> for LongInt {
    type Output = LongInt;

    fn mul(self, rhs: &LongInt) -> Self::Output {
        if self.positive && rhs.positive {
            mul_ignore_sign(&self, &rhs)
        } else if self.positive && !rhs.positive {
            -mul_ignore_sign(&self, &rhs)
        } else if !self.positive && rhs.positive {
            -mul_ignore_sign(&self, &rhs)
        } else {
            mul_ignore_sign(&self, &rhs)
        }
    }
}

impl Div<&LongInt> for LongInt {
    type Output = LongInt;

    fn div(self, rhs: &LongInt) -> Self::Output {
        if self.positive && rhs.positive {
            div_ignore_sign(&self, &rhs).0
        } else if self.positive && !rhs.positive {
            -div_ignore_sign(&self, &rhs).0
        } else if !self.positive && rhs.positive {
            -div_ignore_sign(&self, &rhs).0
        } else {
            div_ignore_sign(&self, &rhs).0
        }
    }
}

impl Add<LongInt> for LongInt {
    type Output = LongInt;

    fn add(self, rhs: LongInt) -> Self::Output {
        if self.positive && rhs.positive {
            add_ignore_sign(&self, &rhs)
        } else if self.positive && !rhs.positive {
            sub_ignore_sign(&self, &rhs)
        } else if !self.positive && rhs.positive {
            -sub_ignore_sign(&self, &rhs)
        } else {
            -add_ignore_sign(&self, &rhs)
        }
    }
}

impl Sub<LongInt> for LongInt {
    type Output = LongInt;

    fn sub(self, rhs: LongInt) -> Self::Output {
        if self.positive && rhs.positive {
            sub_ignore_sign(&self, &rhs)
        } else if self.positive && !rhs.positive {
            add_ignore_sign(&self, &rhs)
        } else if !self.positive && rhs.positive {
            -add_ignore_sign(&self, &rhs)
        } else {
            -sub_ignore_sign(&self, &rhs)
        }
    }
}

impl Mul<LongInt> for LongInt {
    type Output = LongInt;

    fn mul(self, rhs: LongInt) -> Self::Output {
        if self.positive && rhs.positive {
            mul_ignore_sign(&self, &rhs)
        } else if self.positive && !rhs.positive {
            -mul_ignore_sign(&self, &rhs)
        } else if !self.positive && rhs.positive {
            -mul_ignore_sign(&self, &rhs)
        } else {
            mul_ignore_sign(&self, &rhs)
        }
    }
}

impl Div<LongInt> for LongInt {
    type Output = LongInt;

    fn div(self, rhs: LongInt) -> Self::Output {
        if self.positive && rhs.positive {
            div_ignore_sign(&self, &rhs).0
        } else if self.positive && !rhs.positive {
            -div_ignore_sign(&self, &rhs).0
        } else if !self.positive && rhs.positive {
            -div_ignore_sign(&self, &rhs).0
        } else {
            div_ignore_sign(&self, &rhs).0
        }
    }
}

impl Rem<&LongInt> for &LongInt {
    type Output = LongInt;

    fn rem(self, rhs: &LongInt) -> Self::Output {
        div_ignore_sign(&self, &rhs).1
    }
}
impl Rem<&LongInt> for LongInt {
    type Output = LongInt;

    fn rem(self, rhs: &LongInt) -> Self::Output {
        div_ignore_sign(&self, &rhs).1
    }
}
impl Rem<LongInt> for LongInt {
    type Output = LongInt;

    fn rem(self, rhs: LongInt) -> Self::Output {
        div_ignore_sign(&self, &rhs).1
    }
}
impl Rem<LongInt> for &LongInt {
    type Output = LongInt;

    fn rem(self, rhs: LongInt) -> Self::Output {
        div_ignore_sign(&self, &rhs).1
    }
}

impl Into<f32> for LongInt {
    fn into(self) -> f32 {
        todo!()
    }
}
