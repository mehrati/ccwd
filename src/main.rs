use std::env;
use std::fs;
use std::process::{exit, Command, Stdio};

pub struct Clipboard<'a> {
    cmd: String,
    args: &'a [&'a str],
}

impl<'a> Clipboard<'a> {
    pub fn new() -> Self {
        if let Ok(n) = is_program_in_path("xsel") {
            Clipboard {
                cmd: n.to_owned(),
                args: &["-b", "-i"],
            }
        } else {
            if let Ok(n) = is_program_in_path("xclip") {
                Clipboard {
                    cmd: n.to_owned(),
                    args: &["-selection", "c"],
                }
            } else {
                println!("xclip and xset not found !!!");
                println!("please install xclip or xset !!!");
                exit(1);
            }
        }
    }
    pub fn run(self, input: String) {
        let echo = Command::new("echo")
            .arg("-n")
            .arg(input)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap()
            .stdout
            .unwrap();
        let cmd = Command::new(self.cmd)
            .args(self.args)
            .stdin(echo)
            .spawn()
            .unwrap();
        cmd.wait_with_output().unwrap();
    }
}

fn main() {
    let clip = Clipboard::new();
    clip.run(get_cwd_name());
}
pub fn get_cwd_name() -> String {
    let cwd = env::current_dir().unwrap();
    let arg: Vec<String> = env::args().collect();
    if arg.len() > 1 {
        if &arg[1] == "." {
            return cwd.to_str().unwrap().to_owned();
        } else if &arg[1] == ".." {
            let s = cwd.to_str().unwrap();
            let rs: String = s.chars().rev().collect();
            let l = rs.len() - rs.find('/').unwrap();
            return s[..l].to_owned();
        }
        return cwd.join(&arg[1]).to_str().unwrap().to_owned();
    }
    cwd.to_str().unwrap().to_owned()
}

pub fn is_program_in_path(program: &str) -> Result<String, String> {
    if let Ok(path) = env::var("PATH") {
        for p in path.split(":") {
            let p_str = format!("{}/{}", p, program);
            if fs::metadata(p_str).is_ok() {
                return Ok(program.to_owned());
            }
        }
    }
    Err(format!("{} not found !", program))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_program_in_path() {
        assert_eq!(is_program_in_path("ls").unwrap(), "ls");
    }
    #[test]
    fn test_get_cwd_name() {
        assert_eq!(
            get_cwd_name(),
            env::var_os("PWD").unwrap().to_str().unwrap()
        );
    }
}
