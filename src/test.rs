#[cfg(test)]
mod tests {
    use crate::get_position;
    use midas::Bitboard;

    #[test]
    fn test_get_position() {
        let mut output = " ".repeat(64 * 3);
        let bits: Bitboard = 0b1111111110000000000000000000000000000000000000000000000000000000;
        let piece = String::from("WKi");
        get_position(&mut output, bits, piece);

        let mut index = 0;
        while index < output.len() {
            let end_index = std::cmp::min(index + (8 * 3), output.len());
            let substring = &output[index..end_index];
            println!("{}", substring);
            index += 8 * 3;
        }
    }
}
