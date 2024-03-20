use anyhow::{bail, Context};
use clap::{arg, command, Parser};
use semver::{Version, VersionReq};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let req =
        VersionReq::parse(&cli.requested_version).context("could not parse requested versions")?;

    let version_to_test =
        Version::parse(&cli.version_to_test).context("could not parse version to test")?;

    if !req.matches(&version_to_test) {
        bail!("Did not pass version check");
    };

    Ok(())
}

#[derive(Parser)]
#[command(author = "Onypko Dmytro", name = "The Semantic Versioner")]
struct Cli {
    #[arg(
        short,
        help = "SemVer requirement describing the intersection of some version comparators, such as >=1.2.3, <1.8."
    )]
    requested_version: String,
    #[arg(help = "SemVer to ckeck")]
    version_to_test: String,
}
