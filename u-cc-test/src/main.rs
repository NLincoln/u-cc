use colored::*;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

fn workdir() -> io::Result<PathBuf> {
    let pid = std::process::id();
    let temp_dir = std::env::temp_dir().join(&format!("u-cc-test-{}", pid));

    if !temp_dir.exists() {
        fs::create_dir(&temp_dir)?;
    }
    Ok(temp_dir)
}

struct TestCase {
    file_path: PathBuf,
}

enum TestResult {
    Passed,
    WrongStatusCode { expected: i32, received: i32 },
}

impl TestCase {
    pub fn new(file_path: PathBuf) -> TestCase {
        TestCase { file_path }
    }
    /// runs the input file by compiling it to gcc
    /// and returns the status code of the resulting
    /// program
    fn gcc_workdir(&self) -> io::Result<PathBuf> {
        let dir = self.workdir()?.join("gcc");
        fs::create_dir(&dir)?;
        Ok(dir)
    }
    fn run_gcc(&self) -> io::Result<i32> {
        let gcc_workdir = self.gcc_workdir()?;
        let gcc_exe_path = gcc_workdir.join("exec");
        duct::cmd!("gcc", "-o", &gcc_exe_path, self.file_path()?).run()?;

        let output = duct::cmd!(gcc_exe_path).unchecked().run()?;
        let status = &output.status;
        Ok(status.code().unwrap())
    }

    fn workdir(&self) -> io::Result<PathBuf> {
        let workdir = workdir()?.join(&self.name());
        if !workdir.exists() {
            fs::create_dir(&workdir)?;
        }

        Ok(workdir)
    }
    fn obj_file_path(&self) -> io::Result<PathBuf> {
        Ok(self.workdir()?.join(self.filename()?.with_extension("o")))
    }
    fn asm_file_path(&self) -> io::Result<PathBuf> {
        Ok(self.workdir()?.join(self.filename()?.with_extension("asm")))
    }
    fn executable_file_path(&self) -> io::Result<PathBuf> {
        Ok(self.workdir()?.join("out"))
    }
    // returns the file path to the generated asm
    fn compile_c_file(&self) -> io::Result<()> {
        let cmd = duct::cmd!("cargo", "run", "--bin", "u-cc", "--", self.file_path()?)
            .stderr_null()
            .stdout(self.asm_file_path()?);
        cmd.run()?;
        Ok(())
    }
    fn compile_asm_file(&self) -> io::Result<()> {
        let cmd = duct::cmd!(
            "nasm",
            "-f",
            "macho64",
            "-o",
            self.obj_file_path()?,
            self.asm_file_path()?,
        );
        cmd.run()?;
        Ok(())
    }
    fn link_obj_file(&self) -> io::Result<()> {
        let cmd = duct::cmd!(
            "gcc",
            self.obj_file_path()?,
            "-o",
            self.executable_file_path()?
        );
        cmd.run()?;
        Ok(())
    }
    pub fn run(&self) -> io::Result<TestResult> {
        // nasm -f macho64 ret_const.asm && gcc ret_const.o && ./a.out
        self.compile_c_file()?;
        self.compile_asm_file()?;
        self.link_obj_file()?;

        let status_code = {
            let output = duct::cmd!(self.executable_file_path()?).unchecked().run()?;

            output.status.code().unwrap()
        };
        let expected_status = self.run_gcc()?;
        if status_code != expected_status {
            return Ok(TestResult::WrongStatusCode {
                expected: expected_status,
                received: status_code,
            });
        }
        return Ok(TestResult::Passed);
    }

    pub fn name(&self) -> String {
        // our filenames will always be progname.c
        self.filename()
            .unwrap()
            .file_stem()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap()
    }

    pub fn file_path(&self) -> io::Result<&Path> {
        return Ok(self.file_path.as_path());
    }

    pub fn filename(&self) -> io::Result<PathBuf> {
        Ok(PathBuf::from(&self.file_path.file_name().unwrap()))
    }
}

fn collect_test_cases() -> io::Result<impl Iterator<Item = io::Result<TestCase>>> {
    Ok(fs::read_dir("tests")?
        .map(|test_case| -> io::Result<_> { Ok(TestCase::new(test_case?.path())) }))
}

fn main() -> io::Result<()> {
    for test_case in collect_test_cases()? {
        let test_case = test_case?;
        match test_case.run()? {
            TestResult::Passed => {
                println!("{} {}", "[PASSED]".green(), test_case.name());
            }
            TestResult::WrongStatusCode { expected, received } => println!(
                "{} Expected {}, Received {}",
                "[FAILED]".red(),
                expected,
                received
            ),
        }
    }
    return Ok(());
}
