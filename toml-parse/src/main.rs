use std::fs;
use toml::Value;

fn main() -> std::io::Result<()> {
    let toml_content = "
[package]
name = \"hello_world\"
version = \"0.1.0\"
authors = [\"Your Name\"]

[dependencies]
foo = \"0.3.0\"

[dependencies.request]
bar = \"0.3.0\"";

    fs::write("/tmp/test.toml", toml_content)?;
    let content = fs::read_to_string("/tmp/test.toml")?;

    let parse_result = toml::from_str::<toml::Value>(content.as_str());
    let mut parse_content: toml::Value;

    match parse_result {
        Ok(content) => parse_content = content,
        Err(e) => panic!("Error: {}", e),
    }

    println!("{:?}", parse_content["package"]["name"]);
    println!("{:?}", parse_content["package"]["version"]);
    println!("{:?}", parse_content["package"]["authors"]);
    println!("{:?}", parse_content["dependencies"]);
    println!("{:?}", parse_content["dependencies"]["request"]);

    Ok(())
}
