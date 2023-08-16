use std::{
    fs::{File, OpenOptions},
    io::Write,
};

fn main() {
    let file_path = "/sys/class/backlight/acpi_video0/brightness";
    let open_result = OpenOptions::new().write(true).read(false).open(file_path);
    match open_result {
        Ok(file) => write_zero_to_file(file),
        Err(e) => println!("Error when opening file {}", e),
    }
}

fn write_zero_to_file(mut file: File) {
    file.write_all(&[0x30 /* '0' */, 0x0A]).unwrap();
}
