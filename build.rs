use std::process::Command;

fn main() {
    Command::new("tailwindcss")
        .arg("-o")
        .arg("src/tailwind.css")
        .status()
        .expect("Failed to build tailwindcss");
}
