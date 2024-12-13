use clap_mangen::Man;
use clap::CommandFactory;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use termux_clock::cli::Args; // Import Args from your main project crate

fn main() -> std::io::Result<()> {
    let out_dir = "target/manpages";
    std::fs::create_dir_all(out_dir)?;

    // Generate the main command's manpage
    let main_man_path = Path::new(out_dir).join("termux-clock.1");
    let mut main_file = BufWriter::new(File::create(&main_man_path)?);
    let cmd = Args::command();
    Man::new(cmd).render(&mut main_file)?;
    println!("Manpage generated at {}", main_man_path.display());

    // Generate manpages for each subcommand
    let command = Args::command();
    let subcommands = command.get_subcommands();
    for subcmd in subcommands {
        let name = format!("termux-clock-{}.1", subcmd.get_name());
        let subcmd_path = Path::new(out_dir).join(&name);
        let mut subcmd_file = BufWriter::new(File::create(&subcmd_path)?);
        Man::new(subcmd.clone()).render(&mut subcmd_file)?;
        println!("Subcommand manpage generated at {}", subcmd_path.display());
    }

    Ok(())
}
