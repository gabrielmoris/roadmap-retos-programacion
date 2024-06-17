fn main() {
    let json_data = r#"{
        "name":"gabrielcmoris",
        "age":34, 
        "date_of_birth":"17.09.1989", 
        "list_of_programming_languages": ["javascript", "typescript","php","rust"]
    }"#;

    let xml_data = r#"<person name="gabrielcmoris">
    <details>
      <age>34</age>
      <date_of_birth>17.09.1989</date_of_birth>
      <languages>
        <language name="javascript">javascript</language>
        <language name="typescript">typescript</language>
        <language name="php">php</language>
        <language name="rust">rust</language>
      </languages>
    </details>
  </person>"#;

    let json1 = file_creator("gab", json_data, "json");
    let json2 = create_json_file("gabrielcmoris", json_data);

    match json1 {
        Ok(_) => println!("File .json created successfully"),
        Err(err) => println!("Error creating file: {}", err),
    }
    match json2 {
        Ok(_) => println!("File .json created successfully"),
        Err(err) => println!("Error creating file: {}", err),
    }

    let xml1 = file_creator("gab", xml_data, "xml");
    match xml1 {
        Ok(_) => println!("File .xml created successfully"),
        Err(err) => println!("Error creating file: {}", err),
    }
}

/*
* IMPORTANT: You should only upload the code file as part of the exercise.
*
* EXERCISE:
* Develop a program capable of creating an XML and JSON file that saves the
* following data (using the correct syntax in each case):
* - Name
* - Age
* - Date of birth
* - List of programming languages
* Display the content of the files.
* Delete the files.
*/

use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Write};

// BOTH XML and JSON
fn file_creator(file: &str, file_content: &str, extension: &str) -> Result<(), Error> {
    let file_name = format!("{}.{}", file, extension);
    let copied_file_name = file_name.clone();

    if !file_name.is_empty() {
        let _ = File::create(file_name);
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(copied_file_name)?;

        file.write_all(file_content.as_bytes())?;

        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::InvalidInput,
            "You must provide a file name",
        ))
    }
}

//············ JSON ············

// This is a easier approach with a module
use serde_json::{to_string_pretty, Value};

fn create_json_file(filename: &str, file_content: &str) -> Result<(), Error> {
    let name_with_extension = format!("{}.json", filename);

    let json_value: Value = serde_json::from_str(file_content)?; // Parse JSON
    let pretty_json = to_string_pretty(&json_value)?;
    let mut file = File::create(name_with_extension)?;
    file.write_all(pretty_json.to_string().as_bytes())?; // Write serialized JSON
    Ok(())
}

/* EXTRA DIFFICULTY (optional):
* Using the logic of creating the previous files, create a
* program capable of reading and transforming into a custom class of your
* language the data stored in the XML and JSON.
* Delete the files.
*/
