pub fn countdown(val: u8) -> u8 {
    if val > 0 {
        return val - 1;
    } else {
        return 0;
    }
}
