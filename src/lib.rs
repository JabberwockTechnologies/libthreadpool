#[no_mangle]
pub unsafe extern "C" fn multiply_by_two(val: u64) -> u64 {
    return val * 2u64;
}
