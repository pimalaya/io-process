//! Module dedicated to the [`Command`] builder.

use std::{borrow::Cow, collections::HashMap, fmt, path::PathBuf, process::Stdio};

/// The command builder.
///
/// The aim of this builder is to be able to declare a command using
/// the same API from [`std::process::Command`], without any I/O
/// interaction. I/O connectors can then take data from this builder
/// to build I/O-specific commands.
///
/// Refs: [`std::process::Command`]
#[derive(Default)]
pub struct Command {
    /// Path to the program.
    ///
    /// Refs: [`std::process::Command::get_program`]
    program: String,

    /// Arguments that will be passed to the program.
    ///
    /// Refs: [`std::process::Command::get_args`]
    args: Option<Vec<String>>,

    /// Environment variables explicitly set for the child process.
    ///
    /// Refs: [`std::process::Command::get_envs`]
    pub envs: Option<HashMap<String, String>>,

    /// Working directory of the child process.
    ///
    /// Refs: [`std::process::Command::get_current_dir`]
    pub current_dir: Option<PathBuf>,

    /// Configuration for the child process's standard input (stdin)
    /// handle.
    ///
    /// Refs: [`std::process::Command::stdin`]
    pub stdin: Option<Stdio>,

    /// Configuration for the child process's standard output (stdout)
    /// handle.
    ///
    /// Refs: [`std::process::Command::stdout`]
    pub stdout: Option<Stdio>,

    /// Configuration for the child process's standard error (stderr)
    /// handle.
    ///
    /// Refs: [`std::process::Command::stderr`]
    pub stderr: Option<Stdio>,

    /// Should shell-expand program and arguments.
    ///
    /// When true, tilde `~` and environment variables `$ENV` are
    /// expanded for the program and arguments only.
    ///
    /// Requires the `expand` cargo feature.
    #[cfg(feature = "expand")]
    pub expand: bool,
}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("Command");

        debug.field("program", &self.program);

        if let Some(args) = &self.args {
            debug.field("args", args);
        }

        if let Some(envs) = &self.envs {
            debug.field("envs", &envs.keys());
        }

        if let Some(dir) = &self.current_dir {
            debug.field("current_dir", &dir);
        }

        if let Some(stdin) = &self.stdin {
            debug.field("stdin", &stdin);
        }

        if let Some(stdout) = &self.stdout {
            debug.field("stdout", &stdout);
        }

        if let Some(stderr) = &self.stderr {
            debug.field("stderr", &stderr);
        }

        #[cfg(feature = "expand")]
        debug.field("expand", &self.expand);

        debug.finish()
    }
}

impl Command {
    /// Constructs a new [`Command`] for launching the program at path
    /// `program`. This is just a builder, it does not launch any
    /// program on its own. Only I/O connectors do spawn processes.
    ///
    /// Refs: [`std::process::Command::new`]
    pub fn new<S: ToString>(program: S) -> Self {
        Self {
            program: program.to_string(),
            args: None,
            envs: None,
            current_dir: None,
            stdin: None,
            stdout: None,
            stderr: None,
            #[cfg(feature = "expand")]
            expand: false,
        }
    }

    #[cfg(feature = "expand")]
    pub fn expand<'a>(&self, input: &'a str) -> Cow<'a, str> {
        let home_dir = || dirs::home_dir().map(|p| p.to_string_lossy().to_string());
        let get_env = |key: &str| -> Result<Option<Cow<str>>, ()> {
            if let Some(envs) = &self.envs {
                if let Some(val) = envs.get(key) {
                    return Ok(Some(val.into()));
                }
            }

            match std::env::var(key) {
                Ok(val) => Ok(Some(val.into())),
                Err(_) => Ok(None),
            }
        };

        if let Ok(input) = shellexpand::full_with_context(input, home_dir, get_env) {
            return input;
        }

        input.into()
    }

    /// Gets the program as str [`Cow`].
    ///
    /// If the `expand` cargo feature is enabled, and
    /// [`Command::expand`] is true, then program is also
    /// shell-expanded.
    pub fn get_program(&self) -> Cow<str> {
        #[cfg(feature = "expand")]
        if self.expand {
            return self.expand(&self.program);
        }

        Cow::from(&self.program)
    }

    /// Adds an argument to pass to the program.
    ///
    /// Refs: [`std::process::Command::arg`]
    pub fn arg<S: ToString>(&mut self, arg: S) -> &mut Self {
        match &mut self.args {
            Some(args) => {
                args.push(arg.to_string());
            }
            None => {
                self.args = Some(vec![arg.to_string()]);
            }
        }
        self
    }

    /// Adds multiple arguments to pass to the program.
    ///
    /// Refs: [`std::process::Command::args`]
    pub fn args<I, S>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: ToString,
    {
        for arg in args {
            self.arg(arg);
        }
        self
    }

    /// Gets the arguments as str [`Cow`]s.
    ///
    /// If the `expand` cargo feature is enabled, and
    /// [`Command::expand`] is true, then arguments are also
    /// shell-expanded.
    pub fn get_args(&self) -> Option<Vec<Cow<str>>> {
        let self_args = self.args.as_ref()?;
        let mut args = Vec::with_capacity(self_args.len());

        for self_arg in self_args {
            #[cfg(feature = "expand")]
            if self.expand {
                args.push(self.expand(self_arg));
                continue;
            }

            args.push(self_arg.into());
        }

        Some(args)
    }

    /// Inserts or updates an explicit environment variable mapping.
    ///
    /// Refs: [`std::process::Command::env`]
    pub fn env<K, V>(&mut self, key: K, val: V) -> &mut Self
    where
        K: ToString,
        V: ToString,
    {
        let key = key.to_string();
        let val = val.to_string();

        match &mut self.envs {
            Some(envs) => {
                envs.insert(key, val);
            }
            None => {
                self.envs = Some(HashMap::from_iter(Some((key, val))));
            }
        }
        self
    }

    /// Inserts or updates multiple explicit environment variable
    /// mappings.
    ///
    /// Refs: [`std::process::Command::envs`]
    pub fn envs<I, K, V>(&mut self, vars: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: ToString,
        V: ToString,
    {
        for (key, val) in vars {
            self.env(key, val);
        }
        self
    }

    /// Removes an explicitly set environment variable and prevents
    /// inheriting it from a parent process.
    ///
    /// Refs: [`std::process::Command::env_remove`]
    pub fn env_remove<K: AsRef<str>>(&mut self, key: K) -> &mut Self {
        if let Some(envs) = &mut self.envs {
            envs.remove(key.as_ref());
        }
        self
    }

    /// Clears all explicitly set environment variables and prevents
    /// inheriting any parent process environment variables.
    ///
    /// Refs: [`std::process::Command::env_clear`]
    pub fn env_clear(&mut self) -> &mut Self {
        if let Some(envs) = &mut self.envs {
            envs.clear();
        }
        self.envs = None;
        self
    }

    /// Sets the working directory for the child process.
    ///
    /// Refs: [`std::process::Command::current_dir`]
    pub fn current_dir<P: Into<PathBuf>>(&mut self, dir: P) -> &mut Self {
        self.current_dir = Some(dir.into());
        self
    }

    /// Configuration for the child process's standard input (stdin)
    /// handle.
    ///
    /// Refs: [`std::process::Command::stdin`]
    pub fn stdin<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Command {
        self.stdin = Some(cfg.into());
        self
    }

    /// Configuration for the child process's standard output (stdout)
    /// handle.
    ///
    /// Refs: [`std::process::Command::stdout`]
    pub fn stdout<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Command {
        self.stdout = Some(cfg.into());
        self
    }

    /// Configuration for the child process's standard error (stderr)
    /// handle.
    ///
    /// Refs: [`std::process::Command::stderr`]
    pub fn stderr<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Command {
        self.stderr = Some(cfg.into());
        self
    }
}

impl Clone for Command {
    fn clone(&self) -> Self {
        let mut command = Command::new(&self.program);

        #[cfg(feature = "expand")]
        {
            command.expand = self.expand;
        }

        if let Some(args) = self.args.as_ref() {
            for arg in args {
                command.arg(arg);
            }
        }

        if let Some(envs) = self.envs.as_ref() {
            for (key, val) in envs {
                command.env(key, val);
            }
        }

        if let Some(dir) = self.current_dir.as_ref() {
            command.current_dir(dir);
        }

        command
    }
}

impl Eq for Command {}

impl PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        if self.program != other.program {
            return false;
        }

        if self.args != other.args {
            return false;
        }

        if self.current_dir != other.current_dir {
            return false;
        }

        #[cfg(feature = "expand")]
        if self.expand != other.expand {
            return false;
        }

        true
    }
}
