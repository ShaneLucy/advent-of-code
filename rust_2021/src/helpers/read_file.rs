use std::env;
use std::fs;
use std::path::Path;

pub fn read_file(filename: &str) -> std::string::String {
    let cwd = env::current_dir().unwrap();
    let cwd_as_string = cwd.into_os_string().into_string().unwrap();

    let path_as_string = cwd_as_string + "/src/inputs/" + filename;
    let path = Path::new(&path_as_string);

    let contents = fs::read_to_string(path);
    let contents = match contents {
        Ok(file) => file,
        Err(error) => panic!("Problem reading the file: {:?}", error),
    };

    return contents;
}
