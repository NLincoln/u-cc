use std::path::PathBuf;
use std::env;
use std::io;

struct TestCase {
    filename: PathBuf
}

enum TestResult {
    Passed,
    WrongStatusCode { expected: i32, received: i32 }
}

impl TestCase {
    pub fn run(&self) -> io::Result<TestResult> {
        // nasm -f macho64 ret_const.asm && gcc ret_const.o && ./a.out
        unimplemented!()
    }

    pub fn name(&self) -> String {
        // our filenames will always be progname.c
        self.filename.file_stem().unwrap().to_os_string().into_string().unwrap()
    }

    pub fn file_path(&self) -> io::Result<PathBuf> {
        return Ok(env::current_dir()?.join("tests").join(&self.filename))
    }
}

fn main() {
    println!("Hello, world!");
}
