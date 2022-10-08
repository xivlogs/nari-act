/// Param to 2-byte integer
pub(crate) fn u16_from_param(inp: &str) -> u16 {
    u16::from_str_radix(inp, 16).unwrap()
}

/// Param to two 2-byte integers
pub(crate) fn u16x2_from_param(inp: &str) -> (u16, u16) {
    let num = u32::from_str_radix(inp, 16).unwrap();
    let param0 = (&num >> 16) as u16;
    let param1 = num as u16;
    (param0, param1)
}

/// Param to 4-byte float
pub(crate) fn f32_from_param(inp: &str) -> f32 {
    f32::from_bits(u32::from_str_radix(inp, 16).unwrap())
}

/// Param to 4-byte integer
pub(crate) fn u32_from_param(inp: &str) -> u32 {
    u32::from_str_radix(inp, 16).unwrap()
}

/// Param to four 1-byte integers
pub(crate) fn u8x4_from_param(inp: &str) -> (u8, u8, u8, u8) {
    let num = u32::from_str_radix(inp, 16).unwrap();
    let param0 = (&num >> 24) as u8;
    let param1 = (&num >> 16) as u8;
    let param2 = (&num >> 8) as u8;
    let param3 = num as u8;
    (param0, param1, param2, param3)
}
