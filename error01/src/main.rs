pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;


fn main() -> Result<()> {
    let files = list_files("./")?;
    println!("{:#?}", files);
    Ok(())
}

fn list_files(path: &str) -> Result<Vec<String>> {
    let files = std::fs::read_dir(path)?
    .filter_map(|re| re.ok())
    .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
    .map(|e| e.file_name().into_string().unwrap())
    .collect();
 
    Ok(files)
}