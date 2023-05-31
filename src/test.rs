#[cfg(test)]
mod tests {
    use crate::get_position;
    use midas::engine::bitboard::Bitboard;

    #[test]
    fn test_get_position() {
        let mut output = " ".repeat(64 * 3);
        let white_king_bits: Bitboard =
            0b1110001110000000000000000000000000000000000000000000000000000000;
        let white_king_piece = String::from("WKi");
        let black_bishop_bits: Bitboard =
            0b0001110000000000000100010100110001000000000000000000000000000000;
        let black_bishop_piece = String::from("BBi");
        get_position(&mut output, white_king_bits, white_king_piece);
        get_position(&mut output, black_bishop_bits, black_bishop_piece);

        let mut index = 0;
        while index < output.len() {
            let end_index = std::cmp::min(index + (8 * 3), output.len());
            let substring = &output[index..end_index];
            println!("{}", substring);
            index += 8 * 3;
        }
    }
}
