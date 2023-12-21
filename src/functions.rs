use std::fs::File; 
use std::io::{BufRead, BufReader, Error, ErrorKind};

pub fn get_linux_distro_name() -> Result<String, Error> {
    let file = File::open("/etc/os-release")?;
    let reader = BufReader::new(file);

    let distro = reader.lines()
        .map(|l| l.unwrap())
        .find(|l| l.starts_with("PRETTY_NAME="))
        .map(|found| found[13..].trim_matches('"').to_string());
    
    if let Some(distro_name) = distro {
        return Ok(distro_name);
    } else {
        Err(Error::new(ErrorKind::Other, "coulf not find"))
    }

    
}

pub fn get_kernel_version() -> Result<String, Error>{

}

