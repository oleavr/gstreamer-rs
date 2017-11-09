// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ClockTime;
use Format;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum FormatValue {
    Undefined(i64),
    Default(Option<u64>),
    Bytes(Option<u64>),
    Time(ClockTime),
    Buffers(Option<u64>),
    Percent(Option<u32>),
    Other(Format, i64),
}

impl FormatValue {
    pub fn new(format: Format, value: i64) -> Self {
        match format {
            Format::Undefined => FormatValue::Undefined(value),
            Format::Default => FormatValue::Default(if value == -1 {
                None
            } else {
                Some(value as u64)
            }),
            Format::Bytes => FormatValue::Bytes(if value == -1 {
                None
            } else {
                Some(value as u64)
            }),
            Format::Time => FormatValue::Time(if value == -1 {
                ClockTime::none()
            } else {
                ClockTime::from(value as u64)
            }),
            Format::Buffers => FormatValue::Buffers(if value == -1 {
                None
            } else {
                Some(value as u64)
            }),
            Format::Percent => FormatValue::Percent(if value == -1 {
                None
            } else {
                Some(value as u32)
            }),
            Format::__Unknown(_) => FormatValue::Other(format, value),
        }
    }

    pub fn to_format(&self) -> Format {
        match *self {
            FormatValue::Undefined(_) => Format::Undefined,
            FormatValue::Default(_) => Format::Default,
            FormatValue::Bytes(_) => Format::Bytes,
            FormatValue::Time(_) => Format::Time,
            FormatValue::Buffers(_) => Format::Buffers,
            FormatValue::Percent(_) => Format::Percent,
            FormatValue::Other(f, _) => f,
        }
    }

    pub fn to_value(&self) -> i64 {
        match *self {
            FormatValue::Undefined(v) => v,
            FormatValue::Default(v) => v.map(|v| v as i64).unwrap_or(-1),
            FormatValue::Bytes(v) => v.map(|v| v as i64).unwrap_or(-1),
            FormatValue::Time(v) => v.0.map(|v| v as i64).unwrap_or(-1),
            FormatValue::Buffers(v) => v.map(|v| v as i64).unwrap_or(-1),
            FormatValue::Percent(v) => v.map(|v| v as i64).unwrap_or(-1),
            FormatValue::Other(_, v) => v,
        }
    }

    pub fn to_time(&self) -> Option<ClockTime> {
        if let FormatValue::Time(v) = *self {
            Some(v)
        } else {
            None
        }
    }
}

impl From<ClockTime> for FormatValue {
    fn from(v: ClockTime) -> FormatValue {
        FormatValue::Time(v)
    }
}
