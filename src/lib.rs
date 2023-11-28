use chrono::NaiveDate;
use regex::Regex;

const HASH_VOICE_CHECKSUM_HASH_T: [u16; 256] = [
    0x0000, 0xc0c1, 0xc181, 0x0140, 0xc301, 0x03c0, 0x0280, 0xc241, 0xc601, 0x06c0, 0x0780, 0xc741, 0x0500, 0xc5c1, 0xc481, 0x0440,
    0xcc01, 0x0cc0, 0x0d80, 0xcd41, 0x0f00, 0xcfc1, 0xce81, 0x0e40, 0x0a00, 0xcac1, 0xcb81, 0x0b40, 0xc901, 0x09c0, 0x0880, 0xc841,
    0xd801, 0x18c0, 0x1980, 0xd941, 0x1b00, 0xdbc1, 0xda81, 0x1a40, 0x1e00, 0xdec1, 0xdf81, 0x1f40, 0xdd01, 0x1dc0, 0x1c80, 0xdc41,
    0x1400, 0xd4c1, 0xd581, 0x1540, 0xd701, 0x17c0, 0x1680, 0xd641, 0xd201, 0x12c0, 0x1380, 0xd341, 0x1100, 0xd1c1, 0xd081, 0x1040,
    0xf001, 0x30c0, 0x3180, 0xf141, 0x3300, 0xf3c1, 0xf281, 0x3240, 0x3600, 0xf6c1, 0xf781, 0x3740, 0xf501, 0x35c0, 0x3480, 0xf441,
    0x3c00, 0xfcc1, 0xfd81, 0x3d40, 0xff01, 0x3fc0, 0x3e80, 0xfe41, 0xfa01, 0x3ac0, 0x3b80, 0xfb41, 0x3900, 0xf9c1, 0xf881, 0x3840,
    0x2800, 0xe8c1, 0xe981, 0x2940, 0xeb01, 0x2bc0, 0x2a80, 0xea41, 0xee01, 0x2ec0, 0x2f80, 0xef41, 0x2d00, 0xedc1, 0xec81, 0x2c40,
    0xe401, 0x24c0, 0x2580, 0xe541, 0x2700, 0xe7c1, 0xe681, 0x2640, 0x2200, 0xe2c1, 0xe381, 0x2340, 0xe101, 0x21c0, 0x2080, 0xe041,
    0xa001, 0x60c0, 0x6180, 0xa141, 0x6300, 0xa3c1, 0xa281, 0x6240, 0x6600, 0xa6c1, 0xa781, 0x6740, 0xa501, 0x65c0, 0x6480, 0xa441,
    0x6c00, 0xacc1, 0xad81, 0x6d40, 0xaf01, 0x6fc0, 0x6e80, 0xae41, 0xaa01, 0x6ac0, 0x6b80, 0xab41, 0x6900, 0xa9c1, 0xa881, 0x6840,
    0x7800, 0xb8c1, 0xb981, 0x7940, 0xbb01, 0x7bc0, 0x7a80, 0xba41, 0xbe01, 0x7ec0, 0x7f80, 0xbf41, 0x7d00, 0xbdc1, 0xbc81, 0x7c40,
    0xb401, 0x74c0, 0x7580, 0xb541, 0x7700, 0xb7c1, 0xb681, 0x7640, 0x7200, 0xb2c1, 0xb381, 0x7340, 0xb101, 0x71c0, 0x7080, 0xb041,
    0x5000, 0x90c1, 0x9181, 0x5140, 0x9301, 0x53c0, 0x5280, 0x9241, 0x9601, 0x56c0, 0x5780, 0x9741, 0x5500, 0x95c1, 0x9481, 0x5440,
    0x9c01, 0x5cc0, 0x5d80, 0x9d41, 0x5f00, 0x9fc1, 0x9e81, 0x5e40, 0x5a00, 0x9ac1, 0x9b81, 0x5b40, 0x9901, 0x59c0, 0x5880, 0x9841,
    0x8801, 0x48c0, 0x4980, 0x8941, 0x4b00, 0x8bc1, 0x8a81, 0x4a40, 0x4e00, 0x8ec1, 0x8f81, 0x4f40, 0x8d01, 0x4dc0, 0x4c80, 0x8c41,
    0x4400, 0x84c1, 0x8581, 0x4540, 0x8701, 0x47c0, 0x4680, 0x8641, 0x8201, 0x42c0, 0x4380, 0x8341, 0x4100, 0x81c1, 0x8081, 0x4040,
];

#[allow(dead_code)]
/// Represents a voice code hasher for Produce Traceability Initiative (PTI)
///
/// Reference: https://producetraceability.org/voice-pick-code-calculator/
///
/// This struct is used to store all parts of the input to be hashed
/// and resulting output.
///
/// Be aware that the example impl returns case sensitive results so be careful if your Lot Code could be mixed case.
///
/// for GTIN 61414100734933 and Lot 32abcd with pack date 2003-01-02
///
/// # Example
/// ```
/// let mm = "01";
/// let dd = "02";
/// let yy = "03";
/// let voice_code = voicecode::HashVoiceCode::new("12345678901244", "LOT123", yy, mm, dd).unwrap();
/// println!("Voice Code: {}", voice_code.voice_code); // expects 6991
/// println!("Minor: {}", voice_code.voice_code_minor); // expects 69
/// println!("Major: {}", voice_code.voice_code_major); // expects 91
/// assert_eq!(voice_code.voice_code, "6991");
/// ```
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
    /// let mm = "01";
    /// let dd = "02";
    /// let yy = "03";
    /// let voice_code = voicecode::HashVoiceCode::new("12345678901244", "LOT123", yy, mm, dd).unwrap();
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
    /// let pack_date = chrono::NaiveDate::from_ymd_opt(2003, 1, 2);
    /// match pack_date {
    ///    Some(pack_date) => {
    ///       let voice_code = voicecode::HashVoiceCode::new_naive("12345678901244", "LOT123", pack_date).unwrap();
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
    /// let lot = "55ABFC";
    /// assert!(voicecode::HashVoiceCode::validate_lot(lot));
    /// ```
    pub fn validate_lot(lot: &str) -> bool {
        let re = Regex::new(r##"^[\!"%&'()\*\+,\-\./0-9:;<=>\?A-Z_a-z]{1,20}$"##).unwrap();
        re.is_match(lot)
    }

    /// Validate a GTIN string
    /// # Example
    /// ```
    /// let gtin = "12345678901244";
    /// assert!(voicecode::HashVoiceCode::validate_gtin(gtin));
    /// ```
    pub fn validate_gtin(gtin: &str) -> bool {
        return gtin.chars().all(char::is_numeric) || gtin.len() != 14
    }

    ///
    /// Generate a voice code text from a string parts, for free form input
    ///
    ///
    /// # Example
    /// ```
    /// let voice_code = voicecode::HashVoiceCode::generate_voice_code_text("a", "b", "yy", "m", "dd");
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
    /// let input_lot = "LOT123";
    /// let input_gtin = "12345678901244";
    /// let mm = "01";
    /// let dd = "02";
    /// let yy = "03";
    /// let input_text = format!("{}{}{:0>2}{:0>2}{:0>2}", input_gtin, input_lot, yy, mm, dd);
    /// let voice_code = voicecode::HashVoiceCode::generate_voice_code_hash(&input_text);
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
