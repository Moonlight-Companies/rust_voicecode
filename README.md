# voicecode

voice code hasher for Produce Traceability Initiative (PTI) labels

Reference: https://producetraceability.org/voice-pick-code-calculator/

This struct is used to store all parts of the input to be hashed
and resulting output.

Be aware that the example impl returns case sensitive results so be careful
if your Lot Code could be mixed case.

## Example for GTIN 61414100734933 and Lot 32abcd with pack date 2003-01-02

```rust
let voice_code = voicecode::HashVoiceCode::new("123456789012", "LOT123", "01", "02", "03").unwrap();
println!("Voice Code: {}", voice_code.voice_code); // expects 8079
println!("Major: {}", voice_code.voice_code_major); // expects 80
println!("Minor: {}", voice_code.voice_code_minor); // expects 79
```
