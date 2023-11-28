#[cfg(feature = "naive_date")]
use chrono::NaiveDate;

const HASH_VOICE_CHECKSUM_HASH_T: [u16; 256] = [ 0, 49345, 49537, 320, 49921, 960, 640, 49729, 50689, 1728, 1920, 51009, 1280, 50625, 50305, 1088, 52225, 3264, 3456, 52545, 3840, 53185, 52865, 3648, 2560, 51905, 52097, 2880, 51457, 2496, 2176, 51265, 55297, 6336, 6528, 55617, 6912, 56257, 55937, 6720, 7680, 57025, 57217, 8000, 56577, 7616, 7296, 56385, 5120, 54465, 54657, 5440, 55041, 6080, 5760, 54849, 53761, 4800, 4992, 54081, 4352, 53697, 53377, 4160, 61441, 12480, 12672, 61761, 13056, 62401, 62081, 12864, 13824, 63169, 63361, 14144, 62721, 13760, 13440, 62529, 15360, 64705, 64897, 15680, 65281, 16320, 16000, 65089, 64001, 15040, 15232, 64321, 14592, 63937, 63617, 14400, 10240, 59585, 59777, 10560, 60161, 11200, 10880, 59969, 60929, 11968, 12160, 61249, 11520, 60865, 60545, 11328, 58369, 9408, 9600, 58689, 9984, 59329, 59009, 9792, 8704, 58049, 58241, 9024, 57601, 8640, 8320, 57409, 40961, 24768, 24960, 41281, 25344, 41921, 41601, 25152, 26112, 42689, 42881, 26432, 42241, 26048, 25728, 42049, 27648, 44225, 44417, 27968, 44801, 28608, 28288, 44609, 43521, 27328, 27520, 43841, 26880, 43457, 43137, 26688, 30720, 47297, 47489, 31040, 47873, 31680, 31360, 47681, 48641, 32448, 32640, 48961, 32000, 48577, 48257, 31808, 46081, 29888, 30080, 46401, 30464, 47041, 46721, 30272, 29184, 45761, 45953, 29504, 45313, 29120, 28800, 45121, 20480, 37057, 37249, 20800, 37633, 21440, 21120, 37441, 38401, 22208, 22400, 38721, 21760, 38337, 38017, 21568, 39937, 23744, 23936, 40257, 24320, 40897, 40577, 24128, 23040, 39617, 39809, 23360, 39169, 22976, 22656, 38977, 34817, 18624, 18816, 35137, 19200, 35777, 35457, 19008, 19968, 36545, 36737, 20288, 36097, 19904, 19584, 35905, 17408, 33985, 34177, 17728, 34561, 18368, 18048, 34369, 33281, 17088, 17280, 33601, 16640, 33217, 32897, 16448 ];

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
/// # Example for GTIN 61414100734933 and Lot 32abcd with pack date 2003-01-02
///
/// ```
/// let voice_code = voicecode::HashVoiceCode::new("12345678901244", "LOT123", "01", "02", "03").unwrap();
/// println!("Voice Code: {}", voice_code.voice_code); // expects 6991
/// println!("Major: {}", voice_code.voice_code_major); // expects 69
/// println!("Minor: {}", voice_code.voice_code_minor); // expects 91
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
    /// let voice_code = voicecode::HashVoiceCode::new("12345678901244", "LOT123", "01", "02", "03").unwrap();
    /// println!("Voice Code: {}", voice_code.voice_code); // expects 6991
    /// println!("Major: {}", voice_code.voice_code_major); // expects 69
    /// println!("Minor: {}", voice_code.voice_code_minor); // expects 91
    /// ```
    pub fn new(gtin: &str, lot: &str, pack_date_mm: &str, pack_date_dd: &str, pack_date_yy: &str) -> Result<Self, &'static str> {
        if !pack_date_mm.chars().all(char::is_numeric) {
            return Err("Date component MM must be numeric");
        }

        if !pack_date_dd.chars().all(char::is_numeric) {
            return Err("Date component DD must be numeric");
        }

        if !pack_date_yy.chars().all(char::is_numeric) {
            return Err("Date component YY must be numeric");
        }

        if !gtin.chars().all(char::is_numeric) || gtin.len() != 14 {
            return Err("GTIN must be numeric 14 digits");
        }

        let mm = format!("{:0>2}", pack_date_mm);
        let dd = format!("{:0>2}", pack_date_dd);
        let yy = format!("{:0>2}", pack_date_yy);

        if mm.len() != 2 {
            return Err("Invalid mm date format");
        }

        if dd.len() != 2 {
            return Err("Invalid dd date format");
        }

        if yy.len() != 2 {
            return Err("Invalid yy date format");
        }

        let hash_text = format!("{}{}{}{}{}", gtin, lot, pack_date_yy, pack_date_mm, pack_date_dd);
        let voice_code = generate_voice_code_hash(&hash_text);

        Ok(HashVoiceCode {
            hash_text,
            gtin: gtin.to_string(),
            lot: lot.to_string(),
            pack_date: format!("{}{}{}", yy, mm, dd),
            voice_code: voice_code.clone(),
            voice_code_major: voice_code[..2].to_string(),
            voice_code_minor: voice_code[2..].to_string(),
        })
    }

    /// Create a new HashVoiceCode struct with date mm, dd and yy provided from NaiveDate
    /// if your unsure of the date format you should use this method
    ///
    /// Caller should provide format!("{}{}{}{}{}", gtin, lot, pack_date_yy, pack_date_mm, pack_date_dd);
    ///
    /// pack_date_XX is a two digit string for month, day and year
    ///
    /// # Example
    /// ```
    /// let pack_date = chrono::NaiveDate::from_ymd(2003, 1, 2);
    /// let voice_code = voicecode::HashVoiceCode::new_naive("12345678901244", "LOT123", pack_date).unwrap();
    /// println!("Voice Code: {}", voice_code.voice_code); // expects 6991
    /// println!("Major: {}", voice_code.voice_code_major); // expects 69
    /// println!("Minor: {}", voice_code.voice_code_minor); // expects 91
    /// ```
    #[cfg(feature = "naive_date")]
    #[allow(dead_code)]
    pub fn new_naive(gtin: &str, lot: &str, pack_date: NaiveDate) -> Result<Self, &'static str> {
        let date_yy = pack_date.format("%y").to_string();
        let date_mm = pack_date.format("%m").to_string();
        let date_dd = pack_date.format("%d").to_string();

        Self::new(gtin, lot, &date_mm, &date_dd, &date_yy)
    }
}

///
/// Generate a voice code hash from a string
/// # Example
/// ```
/// let voice_code = voicecode::generate_voice_code_hash("12345678901244LOT123030102");
/// println!("Voice Code: {}", voice_code); // expects 6991
/// ```
pub fn generate_voice_code_hash(input: &str) -> String {
    let mut output: u16 = 0;
    for ch in input.chars() {
        output = (output >> 8) ^ HASH_VOICE_CHECKSUM_HASH_T[((output ^ (ch as u16)) % 256) as usize];
    }
    format!("{:04}", output % 10000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "naive_date")]
    use chrono::NaiveDate;

    #[cfg(feature = "naive_date")]
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
        let voice_code = generate_voice_code_hash("12345678901244LOT123030102");
        assert_eq!(voice_code, "6991");
    }

    #[cfg(feature = "naive_date")]
    #[test]
    fn test_naive_date() {
        let gtin = "61414100734933";
        let lot = "32ABCD";
        let pack_date = parse_date("01/01/2001").unwrap();

        let hash_voice_code = HashVoiceCode::new_naive(gtin, lot, pack_date).unwrap();

        assert_eq!(hash_voice_code.voice_code, "1085");
        assert_eq!(hash_voice_code.voice_code_major, "10");
        assert_eq!(hash_voice_code.voice_code_minor, "85");
    }

    #[test]
    fn test1() {
        // basic sanity test

        let gtin = "61414100734933";
        let lot = "32ABCD";
        let pack_date_mm = "01";
        let pack_date_dd = "01";
        let pack_date_yy = "01";

        let hash_voice_code = HashVoiceCode::new(gtin, lot, pack_date_mm, pack_date_dd, pack_date_yy).unwrap();

        assert_eq!(hash_voice_code.voice_code, "1085");
        assert_eq!(hash_voice_code.voice_code_major, "10");
        assert_eq!(hash_voice_code.voice_code_minor, "85");
    }

    #[test]
    fn test2() {
        // these are case sensitive

        let gtin = "61414100734933";
        let lot = "32abcd";
        let pack_date_mm = "01";
        let pack_date_dd = "02";
        let pack_date_yy = "03";

        let hash_voice_code = HashVoiceCode::new(gtin, lot, pack_date_mm, pack_date_dd, pack_date_yy).unwrap();

        assert_eq!(hash_voice_code.voice_code, "8079");
        assert_eq!(hash_voice_code.voice_code_major, "80");
        assert_eq!(hash_voice_code.voice_code_minor, "79");
    }

    #[test]
    fn test3() {
        let gtin = "61414100734933";
        let lot = "32abcd";
        let pack_date_mm = "01";
        let pack_date_dd = "02";
        let pack_date_yy = "03";

        let hash_voice_code = HashVoiceCode::new(gtin, lot, pack_date_mm, pack_date_dd, pack_date_yy).unwrap();

        assert_ne!(hash_voice_code.voice_code, "9190");
        assert_ne!(hash_voice_code.voice_code_major, "91");
        assert_ne!(hash_voice_code.voice_code_minor, "90");
    }

    #[test]
    fn test_invalid_month() {
        let result = HashVoiceCode::new("61414100734933", "32abcd", "ab", "02", "03");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_day() {
        let result = HashVoiceCode::new("61414100734933", "32abcd", "01", "zz", "03");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_year() {
        let result = HashVoiceCode::new("61414100734933", "32abcd", "01", "02", "zz");
        assert!(result.is_err());
    }

}
