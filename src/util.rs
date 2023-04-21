pub fn parse_binary(bits: &[bool]) -> usize {
    let mut x = 0;
    let mut p2 = 1;

    for &bit in bits {
        if bit {
            x |= p2
        };
        p2 <<= 1;
    }

    x
}

pub fn count_true<const N: usize>(bits: [bool; N]) -> usize {
    bits.into_iter().filter(|&b| b).count()
}
