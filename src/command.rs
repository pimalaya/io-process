//! I/O-free command builder.

use alloc::{
    borrow::Cow,
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

use crate::stdio::Stdio;

/// I/O-free command builder.
///
/// Describes a process to be spawned without performing any I/O
/// itself. Runtimes consume this builder to construct and spawn the
/// actual OS process.
///
/// Mirrors the API of [`std::process::Command`] but uses only
/// no_std-compatible types.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Command {
    /// Path to the program.
    program: String,

    /// Arguments passed to the program.
    args: Option<Vec<String>>,

    /// Environment variables explicitly set for the child process.
    pub envs: Option<BTreeMap<String, String>>,

    /// Working directory of the child process.
    pub current_dir: Option<String>,

    /// Configuration for the child process's stdin handle.
    pub stdin: Option<Stdio>,

    /// Configuration for the child process's stdout handle.
    pub stdout: Option<Stdio>,

    /// Configuration for the child process's stderr handle.
    pub stderr: Option<Stdio>,

    /// Whether to shell-expand program and arguments.
    ///
    /// When `true`, tilde `~` and environment variables `$ENV` are
    /// expanded for the program and its arguments.
    ///
    /// Requires the `expand` cargo feature.
    #[cfg(feature = "expand")]
    pub expand: bool,
}

impl Command {
    /// Constructs a new [`Command`] for launching the program at
    /// `program`.
    ///
    /// This is a pure builder — it does not spawn any process.
    pub fn new(program: impl ToString) -> Self {
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

    /// Shell-expands `input`, substituting `~` and `$ENV` variables.
    ///
    /// Environment variables set on this command take priority over
    /// those inherited from the parent process.
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

        if let Ok(expanded) = shellexpand::full_with_context(input, home_dir, get_env) {
            return expanded;
        }

        input.into()
    }

    /// Returns the program path, shell-expanded if the `expand`
    /// feature is enabled and [`Command::expand`] is `true`.
    pub fn get_program(&self) -> Cow<'_, str> {
        #[cfg(feature = "expand")]
        if self.expand {
            return self.expand(&self.program);
        }

        Cow::from(&self.program)
    }

    /// Appends an argument to the program's argument list.
    pub fn arg(&mut self, arg: impl ToString) -> &mut Self {
        match &mut self.args {
            Some(args) => args.push(arg.to_string()),
            None => self.args = Some(alloc::vec![arg.to_string()]),
        }
        self
    }

    /// Appends multiple arguments to the program's argument list.
    pub fn args(&mut self, args: impl IntoIterator<Item = impl ToString>) -> &mut Self {
        for arg in args {
            self.arg(arg);
        }
        self
    }

    /// Returns the argument list, shell-expanded if the `expand`
    /// feature is enabled and [`Command::expand`] is `true`.
    pub fn get_args(&self) -> Option<Vec<Cow<'_, str>>> {
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

    /// Inserts or updates an environment variable for the child
    /// process.
    pub fn env(&mut self, key: impl ToString, val: impl ToString) -> &mut Self {
        match &mut self.envs {
            Some(envs) => {
                envs.insert(key.to_string(), val.to_string());
            }
            None => {
                let mut map = BTreeMap::new();
                map.insert(key.to_string(), val.to_string());
                self.envs = Some(map);
            }
        }
        self
    }

    /// Inserts or updates multiple environment variables for the
    /// child process.
    pub fn envs(
        &mut self,
        vars: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> &mut Self {
        for (key, val) in vars {
            self.env(key, val);
        }
        self
    }

    /// Removes an explicitly set environment variable.
    pub fn env_remove(&mut self, key: impl AsRef<str>) -> &mut Self {
        if let Some(envs) = &mut self.envs {
            envs.remove(key.as_ref());
        }
        self
    }

    /// Clears all explicitly set environment variables.
    pub fn env_clear(&mut self) -> &mut Self {
        self.envs = None;
        self
    }

    /// Sets the working directory for the child process.
    pub fn current_dir(&mut self, dir: impl ToString) -> &mut Self {
        self.current_dir = Some(dir.to_string());
        self
    }

    /// Configures the child process's stdin handle.
    pub fn stdin(&mut self, cfg: impl Into<Stdio>) -> &mut Self {
        self.stdin = Some(cfg.into());
        self
    }

    /// Configures the child process's stdout handle.
    pub fn stdout(&mut self, cfg: impl Into<Stdio>) -> &mut Self {
        self.stdout = Some(cfg.into());
        self
    }

    /// Configures the child process's stderr handle.
    pub fn stderr(&mut self, cfg: impl Into<Stdio>) -> &mut Self {
        self.stderr = Some(cfg.into());
        self
    }
}
