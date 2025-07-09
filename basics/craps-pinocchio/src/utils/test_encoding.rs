//! Tests for bet encoding utilities

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_encode_decode_bet() {
        // Test encoding and decoding various bet types with amounts
        let test_cases = [
            (0u8, 1_000_000_000),      // BET_PASS, 1 CRAP
            (1, 100_000_000_000),      // BET_DONT_PASS, 100 CRAP
            (4, 500_000_000_000),      // BET_FIELD, 500 CRAP
            (48, 1_000_000_000_000),   // BET_NEXT_7, 1000 CRAP
            (63, 10_000_000_000_000),  // BET_REPEATER_12, 10000 CRAP
        ];

        for &(bet_type, amount) in test_cases.iter() {
            let encoded = encode_bet(bet_type, amount).unwrap();
            let (decoded_type, decoded_amount) = decode_bet(encoded);
            
            assert_eq!(bet_type, decoded_type, "Bet type mismatch");
            assert_eq!(amount, decoded_amount, "Amount mismatch");
        }
    }

    #[test]
    fn test_amount_encoding_ranges() {
        // Test boundary values for each encoding range
        let test_amounts = [
            1_000_000_000,      // 1 CRAP (min)
            100_000_000_000,    // 100 CRAP (end of direct encoding)
            105_000_000_000,    // 105 CRAP (by 5s)
            500_000_000_000,    // 500 CRAP (end of by 5s)
            510_000_000_000,    // 510 CRAP (by 10s)
            1_500_000_000_000,  // 1500 CRAP (end of by 10s)
            1_525_000_000_000,  // 1525 CRAP (by 25s)
            5_000_000_000_000,  // 5000 CRAP (end of by 25s)
            5_050_000_000_000,  // 5050 CRAP (by 50s)
            10_000_000_000_000, // 10000 CRAP (end of by 50s)
            20_000_000_000_000, // 20000 CRAP (by 100s)
            40_000_000_000_000, // 40000 CRAP (by 250s)
            60_000_000_000_000, // 60000 CRAP (by 500s)
            80_000_000_000_000, // 80000 CRAP (by 1000s)
            100_000_000_000_000, // 100000 CRAP (max)
        ];

        for &amount in test_amounts.iter() {
            let index = encode_amount(amount).unwrap();
            let decoded = decode_amount(index);
            assert_eq!(amount, decoded, "Amount encoding/decoding failed for {}", amount);
        }
    }

    #[test]
    fn test_invalid_amounts() {
        // Test invalid amounts that should fail encoding
        let invalid_amounts = [
            0,                      // Zero amount
            999_999_999,           // Less than 1 CRAP
            1_000_000_001,         // Not whole CRAP
            103_000_000_000,       // 103 CRAP (not multiple of 5 in that range)
            100_001_000_000_000,   // Over max
        ];

        for &amount in invalid_amounts.iter() {
            assert!(encode_amount(amount).is_err(), "Should fail to encode amount {}", amount);
        }
    }

    #[test]
    fn test_all_bet_types() {
        // Test that all 64 bet types can be encoded
        let amount = 1_000_000_000; // 1 CRAP
        
        for bet_type in 0..=63u8 {
            let encoded = encode_bet(bet_type, amount).unwrap();
            let (decoded_type, _) = decode_bet(encoded);
            assert_eq!(bet_type, decoded_type, "Failed for bet type {}", bet_type);
        }
    }

    #[test]
    fn test_max_values() {
        // Test maximum bet type and amount
        let max_bet_type = 63u8;
        let max_amount = 100_000_000_000_000u64; // 100k CRAP
        
        let encoded = encode_bet(max_bet_type, max_amount).unwrap();
        let (decoded_type, decoded_amount) = decode_bet(encoded);
        
        assert_eq!(max_bet_type, decoded_type);
        assert_eq!(max_amount, decoded_amount);
    }
}