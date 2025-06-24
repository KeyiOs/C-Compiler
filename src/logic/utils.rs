use std::process::Command;


pub fn preprocess_source(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("cpp")
        .arg(file_path)
        .output()?;

    if !output.status.success() {
        return Err(format!("Preprocessor failed with code {:?}", output.status.code()).into());
    }

    let preprocessed_code = String::from_utf8(output.stdout)?;
    Ok(preprocessed_code)
}