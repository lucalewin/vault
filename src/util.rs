use rand::RngCore;

#[inline]
pub fn random_byte_array<const N: usize>() -> [u8; N] {
    let mut buffer = [0u8; N];
    rand::rng().fill_bytes(&mut buffer);
    buffer
}
