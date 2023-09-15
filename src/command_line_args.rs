use clap::Parser;

#[derive(Parser, Clone, Debug)]
pub struct DatabaseArgs {
    /// URL to database
    #[arg(long, env, required = true)]
    pub database_url: String,
}

/// Program options to be read via clap
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CommandLineArgs {
    #[command(flatten)]
    pub database: DatabaseArgs,
}
