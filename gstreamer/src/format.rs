
pub enum FormatValue {
    Undefined(i64),
    Default(u64),
    Bytes(u64),
    Time(ClockTime),
    Buffers(u64),
    Percent(u32),
    Other(Format, i64),
}

impl FormatValue {
    pub fn new(format: Format, value: i64) -> Self {
        unimplemented!()
    }
}
