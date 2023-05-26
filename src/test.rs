#[cfg(test)]
mod tests {
    use crate::get_position;
    use midas::BitBoard;

    #[test]
    fn test_get_position() {
        let mut output = String::with_capacity(64 * 3);
        let bits: BitBoard = 0b1100110011001100110011001100110011001100110011001100110011001100;
        let piece = String::from("WKi");
        get_position(&mut output, bits, piece);
    }
}
