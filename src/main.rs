use anyhow::Result;
use clap::{Parser, ValueHint};
use dialoguer::{theme::ColorfulTheme, Editor, Password};
use mycrypt::{decrypt, encrypt, read_file, write_file};
use std::io::{self, Write};
use std::path::PathBuf;

/// Encrypt/decrypt your file
#[derive(Parser, Debug)]
#[clap(
    about = "Encrypt/decrypt your file with ase256 - https://github.com/sigoden/mycrypt",
    version
)]
struct Cli {
    #[clap(subcommand)]
    cmd: SubCmd,
}

#[derive(Parser, Debug)]
enum SubCmd {
    #[clap(alias = "e")]
    Encrypt(CmdEnc),
    #[clap(alias = "d")]
    Decrypt(CmdDec),
}

/// Encrypt your file
#[derive(Parser, Debug)]
struct CmdEnc {
    /// Specific the file to encrypt
    #[clap(short, long, parse(from_os_str))]
    file: Option<PathBuf>,
    /// Save the encrypted file to
    #[clap(parse(from_os_str))]
    output: PathBuf,
}

/// Decrypt your file
#[derive(Parser, Debug)]
struct CmdDec {
    /// Output file, dump to console when ommitted
    #[clap(short, long, parse(from_os_str))]
    output: Option<PathBuf>,
    /// File to decrypt
    #[clap(parse(from_os_str), value_hint = ValueHint::FilePath)]
    target: PathBuf,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.cmd {
        SubCmd::Encrypt(cmd) => {
            let plain_text = match cmd.file {
                Some(file) => read_file(&file)?,
                None => match Editor::new().edit("Enter text to encrypt")? {
                    Some(text) => text.as_bytes().to_vec(),
                    None => return Ok(()),
                },
            };
            let pass = Password::with_theme(&ColorfulTheme::default())
                .with_prompt("Password")
                .with_confirmation("Repeat password", "The passwords don't match.")
                .interact()?;

            let cipher_text = encrypt(&pass, &plain_text)?;
            write_file(&cmd.output, &cipher_text)?;
        }
        SubCmd::Decrypt(cmd) => {
            let cipher_text = read_file(&cmd.target)?;
            let pass = Password::with_theme(&ColorfulTheme::default())
                .with_prompt("Password")
                .interact()?;

            let plain_text = decrypt(&pass, &cipher_text)?;
            match cmd.output {
                Some(output) => {
                    write_file(&output, &plain_text)?;
                }
                None => {
                    io::stdout().write_all(&plain_text)?;
                }
            }
        }
    }
    Ok(())
}
