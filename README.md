# voicecode

voice code hasher for Produce Traceability Initiative (PTI) labels

Reference: https://producetraceability.org/voice-pick-code-calculator/

This struct is used to store all parts of the input to be hashed
and resulting output.

Be aware that the example impl returns case sensitive results so be careful
if your Lot Code could be mixed case.

## Example for GTIN 123456789012 and Lot LOT123 with pack date 2003-01-02

```rust
let voice_code = HashVoiceCode::new("123456789012", "LOT123", "01", "02", "03").unwrap();
println!("Voice Code: {}", voice_code.voice_code); // expects 8812
println!("Minor: {}", voice_code.voice_code_minor); // expects 88
println!("Major: {}", voice_code.voice_code_major); // expects 12
```

```rust
if let Some(date) = chrono::NaiveDate::from_ymd_opt(2003, 1, 2) {
    let voice_code = HashVoiceCode::new_naive("123456789012", "LOT123", date);
    println!("Voice Code: {}", voice_code.voice_code); // expects 8812
    println!("Minor: {}", voice_code.voice_code_minor); // expects 88
    println!("Major: {}", voice_code.voice_code_major); // expects 12
}
```
