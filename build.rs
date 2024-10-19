use std::process::Command;

fn get_git_description() {
    let output = Command::new("git").args(&["describe", "--dirty"]).output();

    match output {
        Ok(output) => match output.status.success() {
            true => {
                let desc = String::from_utf8(output.stdout).unwrap();
                println!("cargo:rustc-env=GIT_DESCRIPTION={}", desc);
            }
            false => println!("cargo:rustc-env=GIT_DESCRIPTION={}", "unknown"),
        },
        Err(_) => {
            println!("cargo:rustc-env=GIT_DESCRIPTION={}", "unknown");
        }
    }
}

fn get_git_branch() {
    let output = Command::new("git")
        .args(&["branch", "--show-current"])
        .output();

    match output {
        Ok(output) => match output.status.success() {
            true => {
                let branch = String::from_utf8(output.stdout).unwrap();
                println!("cargo:rustc-env=GIT_BRANCH={}", branch);
            }
            false => println!("cargo:rustc-env=GIT_BRANCH={}", "main"),
        },
        Err(_) => {
            println!("cargo:rustc-env=GIT_BRANCH={}", "main");
        }
    }
}

fn main() {
    get_git_description();
    get_git_branch();
}
