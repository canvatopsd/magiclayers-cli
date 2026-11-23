use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "magiclayers",
    version,
    about = "Guest image-to-PSD tasks with one-time payment"
)]
pub struct Arguments {
    #[arg(
        long,
        global = true,
        env = "MAGICLAYERS_BASE_URL",
        default_value = "https://magiclayers.app"
    )]
    pub base_url: String,
    #[command(subcommand)]
    pub command: Command,
}
#[derive(Subcommand)]
pub enum Command {
    /// Read guest service configuration; does not create a task or payment.
    Doctor,
    /// Show the API-owned command inventory offline.
    Discover,
    /// Inspect or install the bundled Agent Skill without network access.
    Skill {
        #[command(subcommand)]
        command: SkillCommand,
    },
    /// Submit an image and freeze its one-time quote. No payment is created.
    Convert(ConversionArguments),
    /// Read one task using its locally saved private access key.
    Status {
        #[arg(value_parser = crate::task_id::parse)]
        task_id: uuid::Uuid,
    },
    /// Return the checkout link; does not open a browser or charge a card.
    Checkout {
        #[arg(value_parser = crate::task_id::parse)]
        task_id: uuid::Uuid,
        #[arg(long, value_enum, default_value = "creem")]
        provider: PaymentProvider,
    },
    /// Read the last verified financial state.
    Payment {
        #[arg(value_parser = crate::task_id::parse)]
        task_id: uuid::Uuid,
    },
    /// Verify an existing checkout with the provider and activate a paid task.
    VerifyPayment {
        #[arg(value_parser = crate::task_id::parse)]
        task_id: uuid::Uuid,
        #[arg(long, required_if_eq("provider", "creem"))]
        checkout_id: Option<String>,
        /// PayPal confirmation captures an order only after buyer approval.
        #[arg(long, value_enum, default_value = "creem")]
        provider: PaymentProvider,
    },
    /// Download and verify a published artifact; never overwrites an existing file.
    Download {
        #[arg(value_parser = crate::task_id::parse)]
        task_id: uuid::Uuid,
        #[arg(long, default_value="package", value_parser=["psd","preview","package"])]
        kind: String,
        #[arg(long)]
        output: PathBuf,
    },
}

#[derive(Clone, Copy, ValueEnum)]
pub enum PaymentProvider {
    Creem,
    Paypal,
}

#[derive(clap::Args)]
pub struct ConversionArguments {
    pub file: PathBuf,
    /// Extract spot-color print separations instead of editable design layers.
    #[arg(long)]
    pub print: bool,
    #[arg(
        long,
        requires = "print",
        help = "Use a saved explicit ink setup for an existing task; new tasks extract colors automatically"
    )]
    pub print_settings: Option<PathBuf>,
    #[arg(long)]
    pub email: String,
    #[arg(long, default_value = "en-US")]
    pub locale: String,
    /// Reuse the original UUIDv7 and saved key after an uncertain response.
    #[arg(long, value_parser = crate::task_id::parse)]
    pub task_id: Option<uuid::Uuid>,
    /// Preview local arguments without saving credentials or sending requests.
    #[arg(long)]
    pub dry_run: bool,
}

#[derive(Subcommand)]
pub enum SkillCommand {
    Show,
    Install(SkillTarget),
    Status(SkillTarget),
}
#[derive(clap::Args)]
pub struct SkillTarget {
    #[arg(default_value = "codex", value_parser = ["codex", "claude-code", "cursor", "copilot", "agents"])]
    pub agent: String,
    /// Parent skills directory; the magiclayers subdirectory is added.
    #[arg(long)]
    pub dir: Option<PathBuf>,
}
