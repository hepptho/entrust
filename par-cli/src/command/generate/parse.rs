use crate::command::BackendValueEnum;
use clap::{Args, ValueEnum};
use par_core::Backend;

#[derive(Args, Debug)]
pub struct GenerateArgs {
    #[arg(default_value = "phrase")]
    pub(crate) r#type: Type,
    #[command(flatten)]
    output: OutputArgs,
    /// Length of the password (default: 7 words for type phrase; 20 characters for type word)
    #[arg(short, long)]
    length: Option<u8>,
    /// Word separator for type phrase
    #[arg(long = "sep", default_value = " ")]
    pub(crate) separator: String,
    /// Do not add the file to the git repository if one exists (only effective with --store)
    #[arg(long)]
    pub(crate) no_git: bool,
    /// Skip the flashy animation when printing to stdout
    #[arg(short, long)]
    pub(crate) no_anim: bool,
    /// Choose gpg or age for en-/decryption
    #[arg(short, long, value_enum, default_value_t = BackendValueEnum::Age)]
    pub(crate) backend: BackendValueEnum,
}

#[derive(ValueEnum, Clone, Debug)]
pub(crate) enum Type {
    Phrase,
    Word,
}

#[derive(Args, Debug)]
#[group(multiple = false)]
pub(crate) struct OutputArgs {
    /// Print the generated password to stdout (default)
    #[arg(long, help_heading = "Output")]
    stdout: bool,
    /// Copy the generated password to the clipboard
    #[arg(
        short,
        long,
        default_missing_value = "10",
        num_args = 0..=1,
        require_equals = true,
        value_name = "CLEAR AFTER SECONDS",
        help_heading = "Output",
    )]
    clipboard: Option<u32>,
    /// Encrypt and store the generated password under the given key
    /// (also copy it to the clipboard)
    #[arg(short, long, help_heading = "Output", value_name = "KEY")]
    store: Option<String>,
}

impl GenerateArgs {
    pub(crate) fn length(&self) -> u8 {
        self.length.unwrap_or(match self.r#type {
            Type::Phrase => 7,
            Type::Word => 20,
        })
    }

    pub(crate) fn needs_backend(&self) -> Option<Backend> {
        if self.output.store.is_some() {
            Some(self.backend.into())
        } else {
            None
        }
    }

    pub(crate) fn output(&self) -> Output {
        match &self.output {
            OutputArgs {
                clipboard: Some(clear_delay_seconds),
                ..
            } => Output::Clipboard(*clear_delay_seconds),
            OutputArgs {
                store: Some(key), ..
            } => Output::Store(key),
            _ => Output::Stdout,
        }
    }
}

pub(crate) enum Output<'a> {
    Stdout,
    Clipboard(u32),
    Store(&'a String),
}
