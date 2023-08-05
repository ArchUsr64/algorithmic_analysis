fn lfsr_fib() -> i32 {
    const START_STATE: i32 = 0xAFD2;
    static mut LFSR: i32 = START_STATE;
    let bit;

    unsafe {
        bit = ((LFSR >> 0) ^ (LFSR >> 2) ^ (LFSR >> 3) ^ (LFSR >> 5)) & 1;
        LFSR = (LFSR >> 1) | (bit << 15);
        return LFSR;
    }
}

}

fn main() {
    loop {
        println!("{}", lfsr_fib());
    }
}
