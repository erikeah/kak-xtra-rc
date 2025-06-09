use std::path::Path;

pub enum SourceKakError {
    Error,
}

pub fn source_kak(kak_file: &Path) -> Result<String, SourceKakError> {
    let rc_str = match kak_file.to_str() {
        Some(value) => value,
        None => return Err(SourceKakError::Error),
    };
    if kak_file.exists() && kak_file.is_file() {
        let mut buf = String::from("source ");
        buf.push_str(rc_str);
        buf.push_str("\n");
        Ok(buf)
    } else {
        Err(SourceKakError::Error)
    }
}
