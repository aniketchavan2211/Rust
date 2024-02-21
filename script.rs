use std::io::Write;
fn main() {
   let var1 = std::io::stdout()
        .write("Aniket ".as_bytes()).unwrap();

   let var2 = std::io::stdout()
        .write(String::from("is Programming in Rust.").as_bytes()).unwrap();

   std::io::stdout()
        .write(format!("\n{} bytes of data has been written!",(var1+var2)).as_bytes()).unwrap();
}
