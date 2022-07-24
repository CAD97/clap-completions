use clap::Command;
use std::{io, process};

pub fn get_clap_help(app: &mut Command) -> String {
    use std::io::Write;
    let mut buf = Vec::new();

    app.write_help(&mut buf).unwrap();
    writeln!(buf).unwrap();

    for app in app.get_subcommands_mut() {
        writeln!(buf, "{}", get_clap_help(app)).unwrap();
    }

    String::from_utf8(buf).unwrap()
}

#[cfg(feature = "nu")]
pub fn get_nu_help(app: &Command) -> io::Result<String> {
    use clap_completions::nu;

    fn build_nu_help<'a>(app: &'a Command<'_>, parent: &mut Vec<&'a str>) -> String {
        if app.is_hide_set() {
            return String::new();
        }

        let mut buf = format!(
            "echo '〉help {0} {1}'; help {0} {1};\n",
            parent.join(" "),
            app.get_name(),
        );
        parent.push(app.get_name());
        for app in app.get_subcommands() {
            buf += &build_nu_help(app, parent);
        }
        parent.pop();
        buf
    }

    let mut program = nu::Completions::new(app).to_string();
    program = format!("module m {{\n{program}\n}}\nuse m *\n");
    program += &build_nu_help(app, &mut vec![]);

    let out = process::Command::new("nu")
        .arg("-c")
        .arg(&program)
        .output()?;

    if out.status.success() {
        Ok(String::from_utf8(out.stdout).unwrap())
    } else {
        panic!(
            "nu failed: {}",
            program + &String::from_utf8(out.stderr).unwrap()
        );
    }
}
