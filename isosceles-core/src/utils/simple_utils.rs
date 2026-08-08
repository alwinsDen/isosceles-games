use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

pub fn simple_command_runner(bin: &'static str, args: &[&str]) -> Result<(), String> {
    let mut child = Command::new(bin)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Errored out from {e}"))?;
    let continous_stdout = child.stdout.take().unwrap();
    let continous_stderr = child.stderr.take().unwrap();
    let out_thread = std::thread::spawn(move || {
        for line in BufReader::new(continous_stdout).lines() {
            match line {
                Ok(t) => {
                    println!("[LOGS] {}", t);
                }
                Err(_) => {}
            }
        }
    });
    let err_thread = std::thread::spawn(move || {
        for line in BufReader::new(continous_stderr).lines() {
            match line {
                Ok(t) => {
                    println!("[ERR] {}", t)
                }
                Err(_) => {}
            }
        }
    });
    // wait for the child process exec
    let _ = child
        .wait()
        .map_err(|_e| format!("Child process failed."))
        .unwrap();
    let _ = out_thread.join();
    let _ = err_thread.join();
    Ok(())
}
