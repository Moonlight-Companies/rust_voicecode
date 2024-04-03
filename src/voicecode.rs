#![deny(const_item_mutation)]
use lazy_static::lazy_static;

use chrono::NaiveDate;
use regex::Regex;

lazy_static! {
    static ref LOT_REGEX: Regex = Regex::new(r##"^[\!"%&'()\*\+,\-\./0-9:;<=>\?A-Z_a-z]{1,20}$"##).expect("Invalid regex");
}

/// Generate CRC look up table similar to reference impl on producetraceability.org using 40961 as the polynomial
use crate::create_crc_lut::create_crc_lut;
lazy_static! {
    static ref HASH_VOICE_CHECKSUM_HASH_T: [u16; 256] = create_crc_lut(40961);
}

use std::fmt;

#[allow(dead_code)]
/// Represents a voice code hasher for Produce Traceability Initiative (PTI)
///
/// Reference: [Voice Pick Code Calculator](https://producetraceability.org/voice-pick-code-calculator/)
/// Reference: [javascript impl](https://voicecode.harvestmark.com/voicecodewidget.js)
///
/// This struct is used to store all parts of the input to be hashed
/// and resulting output.
///
/// Be aware that the example impl returns case sensitive results so be careful if your Lot Code could be mixed case.
///
/// for GTIN 12345678901244 and Lot LOT123 with pack date 2003-01-02
///
/// # Example
/// ```
/// use voicecode::{ HashVoiceCode };
/// let mm = "01";
/// let dd = "02";
/// let yy = "03";
/// let voice_code = HashVoiceCode::new("12345678901244", "LOT123", yy, mm, dd).unwrap();
/// println!("Voice Code: {}", voice_code.voice_code); // expects 6991
/// println!("Minor: {}", voice_code.voice_code_minor); // expects 69
/// println!("Major: {}", voice_code.voice_code_major); // expects 91
/// assert_eq!(voice_code.voice_code, "6991");
/// ```

#[derive(Clone)]
pub struct HashVoiceCode {
    pub hash_text: String,
    pub gtin: String,
    pub lot: String,
    pub pack_date: String,
    pub voice_code: String,
    pub voice_code_major: String,
    pub voice_code_minor: String,
}

impl HashVoiceCode {
    #[allow(dead_code)]
    /// Create a new HashVoiceCode struct with date mm, dd and yy as strings
    ///
    /// # Note
    /// date string components outside normal bounds will
    /// hash and not be valid for the PTI label format
    /// ex mm="99" dd="99" yy="99" is valid and is not a real date
    ///
    /// this method assumes you've provided valid date parts
    ///
    /// # Example
    /// ```
    /// use voicecode::{ HashVoiceCode };
    /// let mm = "01";
    /// let dd = "02";
    /// let yy = "03";
    /// let voice_code = HashVoiceCode::new("12345678901244", "LOT123", yy, mm, dd).unwrap();
    /// println!("Voice Code: {}", voice_code.voice_code); // expects 6991
    /// println!("Minor: {}", voice_code.voice_code_minor); // expects 69
    /// println!("Major: {}", voice_code.voice_code_major); // expects 91
    ///
    /// assert_eq!(voice_code.voice_code, "6991");
    /// ```
    pub fn new(gtin: &str, lot: &str, pack_date_yy: &str, pack_date_mm: &str, pack_date_dd: &str) -> Result<Self, &'static str> {
        if !pack_date_yy.chars().all(char::is_numeric) || pack_date_yy.len() > 2 || pack_date_yy.len() < 1 {
            return Err("Date component YY must be numeric and 1 or 2 digits");
        }

        if !pack_date_mm.chars().all(char::is_numeric) || pack_date_mm.len() > 2 || pack_date_mm.len() < 1 {
            return Err("Date component MM must be numeric and 1 or 2 digits");
        }

        if !pack_date_dd.chars().all(char::is_numeric) || pack_date_dd.len() > 2 || pack_date_dd.len() < 1 {
            return Err("Date component DD must be numeric and 1 or 2 digits");
        }

        if !Self::validate_lot(lot) {
            // note - gs1 codes use (xx)data to indicate various kinds of data, allowing parens should probably not be allowed
            return Err(r##"LOT must be alphanumeric and/or !, ", %, &, ', (, ), *, +, -, ., /, :, ;, <, =, >, ?, _ and comma"##);
        }

        if !Self::validate_gtin(gtin) {
            return Err("GTIN must be numeric 14 digits");
        }

        let yy = format!("{:0>2}", pack_date_yy);
        let mm = format!("{:0>2}", pack_date_mm);
        let dd = format!("{:0>2}", pack_date_dd);

        let hash_text = format!("{}{}{}{}{}", gtin, lot, pack_date_yy, pack_date_mm, pack_date_dd);
        let voice_code = HashVoiceCode::generate_voice_code_hash(&hash_text);

        Ok(HashVoiceCode {
            hash_text,
            gtin: gtin.to_string(),
            lot: lot.to_string(),
            pack_date: format!("{}{}{}", yy, mm, dd),
            voice_code: voice_code.clone(),
            voice_code_major: voice_code[2..].to_string(),
            voice_code_minor: voice_code[..2].to_string(),
        })
    }

    /// Create a new HashVoiceCode struct with date mm, dd and yy provided from NaiveDate
    ///
    /// # Example
    /// ```
    /// use voicecode::{ HashVoiceCode };
    /// let pack_date = chrono::NaiveDate::from_ymd_opt(2003, 1, 2);
    /// match pack_date {
    ///    Some(pack_date) => {
    ///       let voice_code = HashVoiceCode::new_naive("12345678901244", "LOT123", pack_date).unwrap();
    ///       println!("Voice Code: {}", voice_code.voice_code); // expects 6991
    ///       println!("Minor: {}", voice_code.voice_code_minor); // expects 69
    ///       println!("Major: {}", voice_code.voice_code_major); // expects 91
    ///
    ///       assert_eq!(voice_code.voice_code, "6991");
    ///    },
    ///    None => {
    ///       println!("Invalid date");
    ///       assert!(false);
    ///    }
    /// }
    ///
    /// ```
    #[allow(dead_code)]
    pub fn new_naive(gtin: &str, lot: &str, pack_date: NaiveDate) -> Result<Self, &'static str> {
        let date_yy = pack_date.format("%y").to_string();
        let date_mm = pack_date.format("%m").to_string();
        let date_dd = pack_date.format("%d").to_string();

        Self::new(gtin, lot, &date_yy, &date_mm, &date_dd)
    }

    /// Validate a LOT string
    /// # Example
    /// ```
    /// use voicecode::{ HashVoiceCode };
    /// let lot = "55ABFC";
    /// assert!(HashVoiceCode::validate_lot(lot));
    /// ```
    pub fn validate_lot(lot: &str) -> bool {
        LOT_REGEX.is_match(lot)
    }

    /// Validate a GTIN string
    /// # Example
    /// ```
    /// use voicecode::{ HashVoiceCode };
    /// let gtin = "12345678901244";
    /// assert!(HashVoiceCode::validate_gtin(gtin));
    /// ```
    pub fn validate_gtin(gtin: &str) -> bool {
        return gtin.chars().all(char::is_numeric) && (gtin.len() == 8 || gtin.len() == 12 || gtin.len() == 13 || gtin.len() == 14)
    }

    ///
    /// Generate a voice code text from a string parts, for free form input
    ///
    ///
    /// # Example
    /// ```
    /// use voicecode::{ HashVoiceCode };
    /// let voice_code = HashVoiceCode::generate_voice_code_text("a", "b", "yy", "m", "dd");
    /// assert_eq!(voice_code, "abyy0mdd");
    /// ```
    pub fn generate_voice_code_text(gtin: &str, lot: &str, pack_date_yy: &str, pack_date_mm: &str, pack_date_dd: &str) -> String {
        format!("{}{}{:0>2}{:0>2}{:0>2}", gtin, lot, pack_date_yy, pack_date_mm, pack_date_dd)
    }

    ///
    /// Generate a voice code hash from a string
    ///
    /// Caller should provide format!("{}{}{}{}{}", gtin, lot, pack_date_yy, pack_date_mm, pack_date_dd);
    ///
    /// pack_date_XX is a two digit string for month, day and year
    ///
    /// # Example
    /// ```
    /// use voicecode::{ HashVoiceCode };
    /// let input_lot = "LOT123";
    /// let input_gtin = "12345678901244";
    /// let mm = "01";
    /// let dd = "02";
    /// let yy = "03";
    /// let input_text = format!("{}{}{:0>2}{:0>2}{:0>2}", input_gtin, input_lot, yy, mm, dd);
    /// let voice_code = HashVoiceCode::generate_voice_code_hash(&input_text);
    /// println!("Voice Code: {}", voice_code); // expects 6991
    /// assert_eq!(voice_code, "6991");
    /// ```
    pub fn generate_voice_code_hash(input: &str) -> String {
        let mut output: u16 = 0;
        for ch in input.chars() {
            output = (output >> 8) ^ HASH_VOICE_CHECKSUM_HASH_T[((output ^ (ch as u16)) % 256) as usize];
        }
        format!("{:04}", output % 10000)
    }
}

impl fmt::Debug for HashVoiceCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HashVoiceCode")
            .field("gtin", &self.gtin)
            .field("lot", &self.lot)
            .field("pack_date", &self.pack_date)
            .field("voice_code", &self.voice_code)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_date(input: &str) -> Result<NaiveDate, chrono::format::ParseError> {
        let formats = vec!["%m/%d/%Y", "%m%d%Y", "%Y-%m-%d", "%+"];
        for format in formats {
            if let Ok(date) = NaiveDate::parse_from_str(input, format) {
                return Ok(date);
            }
        }
        NaiveDate::parse_from_str(input, "")
    }

    #[test]
    fn test_chrono() {
        if let Some(date) = chrono::NaiveDate::from_ymd_opt(2003, 1, 2) {
            let voice_code = HashVoiceCode::new_naive("12345678901234", "LOT123", date);
            match voice_code {
                Ok(voice_code) => {
                    println!("Voice Code: {}", voice_code.voice_code); // expects 6991
                    println!("Minor: {}", voice_code.voice_code_minor); // expects 69
                    println!("Major: {}", voice_code.voice_code_major); // expects 91
                }
                Err(e) => {
                    println!("Error: {}", e);
                    assert!(false);
                }
            }
        }
    }

    #[test]
    fn test_hash_voice_code_string() {
        // raw hash test
        let voice_code = HashVoiceCode::generate_voice_code_hash("12345678901244LOT123030102");
        assert_eq!(voice_code, "6991");
    }

    #[test]
    fn test_naive_date() {
        let gtin = "61414100734933";
        let lot = "32ABCD";
        let pack_date = parse_date("01/01/2001").unwrap();

        let hash_voice_code = HashVoiceCode::new_naive(gtin, lot, pack_date).unwrap();

        assert_eq!(hash_voice_code.voice_code, "1085");
        assert_eq!(hash_voice_code.voice_code_minor, "10");
        assert_eq!(hash_voice_code.voice_code_major, "85");
    }

    #[test]
    fn test1() {
        // basic sanity test

        let gtin = "61414100734933";
        let lot = "32ABCD";
        let pack_date_yy = "01";
        let pack_date_mm = "01";
        let pack_date_dd = "01";

        let hash_voice_code = HashVoiceCode::new(gtin, lot, pack_date_yy, pack_date_mm, pack_date_dd).unwrap();

        assert_eq!(hash_voice_code.voice_code, "1085");
        assert_eq!(hash_voice_code.voice_code_minor, "10");
        assert_eq!(hash_voice_code.voice_code_major, "85");
    }

    #[test]
    fn test2() {
        // these are case sensitive

        let gtin = "61414100734933";
        let lot = "32abcd";
        let pack_date_yy = "03";
        let pack_date_mm = "01";
        let pack_date_dd = "02";

        let hash_voice_code = HashVoiceCode::new(gtin, lot, pack_date_yy, pack_date_mm, pack_date_dd).unwrap();

        assert_eq!(hash_voice_code.voice_code, "8079");
        assert_eq!(hash_voice_code.voice_code_minor, "80");
        assert_eq!(hash_voice_code.voice_code_major, "79");
    }

    #[test]
    fn test2_1() {
        // these are case sensitive, so this should not match test2 as lot is ABCD instead of abcd

        let gtin = "61414100734933";
        let lot = "32ABCD";
        let pack_date_yy = "03";
        let pack_date_mm = "01";
        let pack_date_dd = "02";

        let hash_voice_code = HashVoiceCode::new(gtin, lot, pack_date_yy, pack_date_mm, pack_date_dd).unwrap();

        assert_ne!(hash_voice_code.voice_code, "8079");
        assert_ne!(hash_voice_code.voice_code_minor, "80");
        assert_ne!(hash_voice_code.voice_code_major, "79");
    }

    #[test]
    fn test3() {
        let gtin = "61414100734933";
        let lot = "32abcd";
        let pack_date_yy = "03";
        let pack_date_mm = "01";
        let pack_date_dd = "02";

        let hash_voice_code = HashVoiceCode::new(gtin, lot, pack_date_yy, pack_date_mm, pack_date_dd).unwrap();

        assert_ne!(hash_voice_code.voice_code, "9190");
        assert_ne!(hash_voice_code.voice_code_minor, "91");
        assert_ne!(hash_voice_code.voice_code_major, "90");
    }

    #[test]
    fn test_invalid_month() {
        let result = HashVoiceCode::new("61414100734933", "32abcd", "03", "mm", "03");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_day() {
        let result = HashVoiceCode::new("61414100734933", "32abcd", "03", "02", "dd");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_year() {
        let result = HashVoiceCode::new("61414100734933", "32abcd", "yy", "01", "02");
        assert!(result.is_err());
    }

}
