use std::{
    fs::{File, OpenOptions},
    io::Write,
};

fn main() {
    // open the ACPI brightness control of the main display on my laptop
    let file_path = "/sys/class/backlight/acpi_video0/brightness";
    let open_result = OpenOptions::new().write(true).read(false).open(file_path);

    match open_result {
        Ok(file) => write_zero_to_file(file),
        Err(e) => println!("Error when opening file {}", e),
    }
}

// Just writes a "0\n" to the specfied file.
fn write_zero_to_file(mut file: File) {
    file.write_all(&[
        0x30 /* '0' in ascii */, 
        0x0A /* '/n' in ascii */]
    ).expect("Cannot write to the brightness control file.");
}
