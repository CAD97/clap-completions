use clap::{Arg, Command, ValueHint};
use core::fmt::{self, Display};

/// Completions for [nushell].
///
/// [nushell]: https://www.nushell.sh/
#[derive(Debug, Clone, Copy)]
pub struct Completions<'a, 'help> {
    app: &'a Command<'help>,
}

impl<'a, 'help> Completions<'a, 'help> {
    /// Create a new completions generator.
    pub fn new(app: &'a Command<'help>) -> Self {
        Self { app }
    }
}

impl Display for Completions<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_completion_module(self.app, f)
    }
}

fn write_completion_module(app: &Command<'_>, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let name = app.get_name();

    write_nu_completes(app, f, FullCommandName { name, parent: None })?;
    write_export_externs(app, f, FullCommandName { name, parent: None })?;

    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct FullCommandName<'a> {
    name: &'a str,
    parent: Option<&'a FullCommandName<'a>>,
}

impl Display for FullCommandName<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let FullCommandName { name, parent } = self;
        match parent {
            Some(parent) => write!(f, "{parent} {name}"),
            None => write!(f, "{name}"),
        }
    }
}

fn nu_value_hint(hint: ValueHint) -> &'static str {
    use ValueHint::*;
    #[allow(clippy::wildcard_in_or_patterns)]
    #[cfg_attr(nightly, warn(non_exhaustive_omitted_patterns))]
    match hint {
        Unknown => "string",
        AnyPath | FilePath | DirPath | ExecutablePath => "path",
        CommandName | CommandString | CommandWithArguments | Username | Hostname | Url
        | EmailAddress => "string",
        Other | _ => "any",
    }
}

fn write_nu_completes(
    app: &Command<'_>,
    f: &mut fmt::Formatter<'_>,
    name: FullCommandName<'_>,
) -> fmt::Result {
    if app.is_hide_set() {
        return Ok(());
    }

    for arg in app.get_arguments() {
        if arg.is_hide_set() {
            continue;
        }

        let nu_type = nu_value_hint(arg.get_value_hint());
        if nu_type != "string" {
            continue;
        }

        let id = arg.get_id();
        let value_matcher = arg.get_value_parser();

        if let Some(possible_values) = value_matcher.possible_values() {
            writeln!(f, "def 'nu-complete {name} --{id}' [] {{")?;
            write!(f, "  [ ")?;
            for possible_value in possible_values {
                write!(
                    f,
                    "\"{}\", ",
                    possible_value.get_name().replace('\"', "\\\"")
                )?;
            }
            writeln!(f, "]")?;
            writeln!(f, "}}")?;
            writeln!(f)?;
        }
    }

    for app in app.get_subcommands() {
        let parent = Some(&name);
        let name = app.get_name();
        write_nu_completes(app, f, FullCommandName { name, parent })?;
    }

    Ok(())
}

fn write_export_externs(
    app: &Command<'_>,
    f: &mut fmt::Formatter<'_>,
    name: FullCommandName<'_>,
) -> fmt::Result {
    if app.is_hide_set() {
        return Ok(());
    }

    if let Some(about) = app.get_about() {
        let about = about.split(['\r', '\n']).next().unwrap();
        writeln!(f, "# {about}")?;
    }

    writeln!(f, "export extern '{name}' [")?;

    let mut write_param = |arg: &Arg<'_>| -> fmt::Result {
        let id = arg.get_id();
        let help = arg.get_help();
        let long = arg
            .get_long()
            .or_else(|| arg.get_long_and_visible_aliases().map(|v| v[0]));
        let short = arg
            .get_short()
            .or_else(|| arg.get_short_and_visible_aliases().map(|v| v[0]));
        let value_hint = arg.get_value_hint();
        let required = arg.is_required_set();
        let takes_value = arg.is_takes_value_set();
        let takes_many = arg.is_multiple_values_set();

        let value_matcher = arg.get_value_parser();
        let has_value_completion = value_matcher.possible_values().is_some();

        let splat = if takes_many { "..." } else { "" };
        let nu_type = nu_value_hint(value_hint);
        let nu_type = if has_value_completion {
            format!("{nu_type}@'nu-complete {name} --{id}'")
        } else {
            nu_type.to_string()
        };
        let nu_optional = if required { "" } else { "?" };
        let help = help.unwrap_or("").split(['\r', '\n']).next().unwrap();

        match (long, short, takes_value) {
            (Some(long), Some(short), true) => {
                writeln!(f, "  --{long}(-{short}): {nu_type} # {help}")?;
            }
            (Some(long), Some(short), false) => {
                writeln!(f, "  --{long}(-{short}) # {help}")?;
            }
            (Some(long), None, true) => {
                writeln!(f, "  --{long}: {nu_type} # {help}")?;
            }
            (Some(long), None, false) => {
                writeln!(f, "  --{long} # {help}")?;
            }
            (None, Some(short), true) => {
                writeln!(f, "  -{short}: {nu_type} # {help}")?;
            }
            (None, Some(short), false) => {
                writeln!(f, "  -{short} # {help}")?;
            }
            (None, None, _) => {
                writeln!(f, "  {splat}{id}{nu_optional}: {nu_type} # {help}")?;
            }
        };

        Ok(())
    };

    app.get_arguments()
        .filter(|arg| !arg.is_hide_set())
        .filter(|arg| arg.is_positional())
        .try_for_each(&mut write_param)?;
    app.get_arguments()
        .filter(|arg| !arg.is_hide_set())
        .filter(|arg| !arg.is_positional())
        .try_for_each(&mut write_param)?;

    writeln!(f, "]")?;
    writeln!(f)?;

    for app in app.get_subcommands() {
        let parent = Some(&name);
        let name = app.get_name();
        write_export_externs(app, f, FullCommandName { name, parent })?;
    }

    Ok(())
}
