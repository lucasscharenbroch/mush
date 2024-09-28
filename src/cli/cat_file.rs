use crate::cli::MushSubcommand;
use crate::cli::ExitType;
use crate::object::Object;

#[derive(clap::Args)]
pub struct CatFileArgs {
    #[command(flatten)]
    variant: CatFileVariantArgs,

    /// The name of the object to show (hash or ref)
    #[arg(requires = "variant")]
    object: String,
}

/// CatFileVariant (as mutually exclusive flags):
#[derive(clap::Args)]
struct CatFileVariantArgs {
    /// Show the type of the object
    #[arg(short, group="variant")]
    tipe: bool,

    /// Pretty-print the object
    #[arg(short, group="variant")]
    pretty_print: bool,

    /// Check if the object exists
    #[arg(short, group="variant")]
    exists: bool,

    /// Show the size (in bytes) of the object
    #[arg(short, group="variant")]
    size: bool,
}

impl CatFileVariantArgs {
    fn toEnum(&self) -> CatFileVariant {
        match (self.tipe, self.pretty_print, self.exists, self.size) {
            (true, false, false, false) => CatFileVariant::Type,
            (false, true, false, false) => CatFileVariant::PrettyPrint,
            (false, false, true, false) => CatFileVariant::Exists,
            (false, false, false, true) => CatFileVariant::Size,
            _ => panic!("Clap invariant violated: args not mutually exclusive"),
        }
    }
}

#[derive(Clone, clap::ValueEnum)]
enum CatFileVariant {
    Type,
    PrettyPrint,
    Exists,
    Size,
}

impl MushSubcommand for CatFileArgs {
    fn execute(&self) -> ExitType {
        todo!()
    }
}
