use std::io::Write;
use toml::Value;

fn main() {
    let project_name = project_name();
    if project_name.is_none() {
        println!("Failed to get project name from Cargo.toml");
        println!("Please make sure exist Cargo.toml in the project root directory and it has a package name");
    } else {
        let project_name = project_name.unwrap();
        let output = std::fs::File::create("submission.rs").unwrap();
        let mut writer = std::io::BufWriter::new(output);
        main_parse(&mut writer);
        lib_parse(project_name, &mut writer);
    }
}

fn project_name() -> Option<String> {
    let toml_str = std::fs::read_to_string("Cargo.toml").unwrap_or_else(|_| "".to_string());
    match toml_str.parse::<Value>() {
        Ok(value) => {
            let project_name = value["package"]["name"].as_str().unwrap();
            Some(project_name.to_string())
        }
        Err(_) => None,
    }
}

fn main_parse(writer: &mut std::io::BufWriter<std::fs::File>) {
    let contents = std::fs::read_to_string("src/main.rs").unwrap_or_else(|_| "".to_string());
    for line in contents.lines() {
        writer.write_all(line.as_bytes()).unwrap();
        writer.write_all("\n".as_bytes()).unwrap();
    }
}

/// parse src/lib.rs file
fn lib_parse(project_name: String, writer: &mut std::io::BufWriter<std::fs::File>) {
    let contents = std::fs::read_to_string("src/lib.rs").unwrap_or_else(|_| "".to_string());
    writer
        .write_all(format!("pub mod {} {{\n", project_name).as_bytes())
        .unwrap();
    for line in contents.lines() {
        if line.contains("pub mod") {
            let mod_name = line.split_whitespace().collect::<Vec<&str>>()[2];
            let mod_name = &mod_name[..mod_name.len() - 1]; // remove last ;
            writer
                .write_all(format!("    pub mod {} {{\n", mod_name).as_bytes())
                .unwrap();

            if std::fs::metadata(&format!("src/{}.rs", mod_name)).is_ok() {
                parse(
                    &format!("src/{}.rs", mod_name),
                    project_name.clone(),
                    writer,
                    2,
                );
            } else {
                mod_parse(&format!("src/{}/", mod_name), writer, 2);
            }
            writer.write_all("    }\n".as_bytes()).unwrap();
        } else {
            writer.write_all("    ".as_bytes()).unwrap();
            writer.write_all(line.as_bytes()).unwrap();
            writer.write_all("\n".as_bytes()).unwrap();
        }
    }
    writer.write_all("}\n".as_bytes()).unwrap();
}

/// parse mod.rs file
fn mod_parse(path: &str, writer: &mut std::io::BufWriter<std::fs::File>, indent: usize) {
    let contents =
        std::fs::read_to_string(format!("{}mod.rs", path)).unwrap_or_else(|_| "".to_string());
    for line in contents.lines() {
        if line.contains("pub mod") {
            let mod_name = line.split_whitespace().collect::<Vec<&str>>()[2];
            let mod_name = &mod_name[..mod_name.len() - 1]; // remove last ;
            for _ in 0..indent {
                writer.write_all("    ".as_bytes()).unwrap();
            }
            writer
                .write_all(format!("pub mod {} {{\n", mod_name).as_bytes())
                .unwrap();

            // to do: parse mod_name.rs file or mod_name/mod.rs file
            if std::fs::metadata(&format!("{}{}.rs", path, mod_name)).is_ok() {
                parse(
                    &format!("{}{}.rs", path, mod_name),
                    "".to_string(),
                    writer,
                    indent + 1,
                );
            } else {
                mod_parse(&format!("{}{}/", path, mod_name), writer, indent + 1);
            }

            for _ in 0..indent {
                writer.write_all("    ".as_bytes()).unwrap();
            }
            writer.write_all("}\n".as_bytes()).unwrap();
        } else {
            for _ in 0..indent {
                writer.write_all("    ".as_bytes()).unwrap();
            }
            writer.write_all(line.as_bytes()).unwrap();
            writer.write_all("\n".as_bytes()).unwrap();
        }
    }
}

/// parse other files
fn parse(
    path: &str,
    project_name: String,
    writer: &mut std::io::BufWriter<std::fs::File>,
    indent: usize,
) {
    let contents = std::fs::read_to_string(path).unwrap_or_else(|_| "".to_string());
    for line in contents.lines() {
        if line.contains("use crate::") {
            for _ in 0..indent {
                writer.write_all("    ".as_bytes()).unwrap();
            }
            let line = line.replace("use crate::", &format!("use crate::{}::", project_name));
            writer.write_all(line.as_bytes()).unwrap();
        } else {
            for _ in 0..indent {
                writer.write_all("    ".as_bytes()).unwrap();
            }
            writer.write_all(line.as_bytes()).unwrap();
            writer.write_all("\n".as_bytes()).unwrap();
        }
    }
}
