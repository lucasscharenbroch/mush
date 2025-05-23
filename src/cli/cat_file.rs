use crate::cli::ExitType;
use crate::cli::MushSubcommand;
use crate::cli_expect;
use crate::io::read_object;
use crate::io::read_object_header;
use crate::revision::RevisionSpec;

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
    #[arg(short, group = "variant")]
    tipe: bool,

    /// Pretty-print the object
    #[arg(short, group = "variant")]
    pretty_print: bool,

    /// Check if the object exists
    #[arg(short, group = "variant")]
    exists: bool,

    /// Show the size (in bytes) of the object
    #[arg(short, group = "variant")]
    size: bool,
}

impl CatFileVariantArgs {
    fn to_enum(&self) -> CatFileVariant {
        match (self.tipe, self.pretty_print, self.exists, self.size) {
            (true, false, false, false) => CatFileVariant::Type,
            (false, true, false, false) => CatFileVariant::PrettyPrint,
            (false, false, true, false) => CatFileVariant::Exists,
            (false, false, false, true) => CatFileVariant::Size,
            _ => panic!("Clap invariant violated: args not mutually exclusive"),
        }
    }
}

#[derive(Clone)]
enum CatFileVariant {
    Type,
    PrettyPrint,
    Exists,
    Size,
}

impl MushSubcommand for CatFileArgs {
    fn execute(&self) -> ExitType {
        let revision_spec = crate::cli_expect!(RevisionSpec::parse(&self.object));
        let hash = crate::cli_expect!(revision_spec.dereference());
        let header = cli_expect!(read_object_header(&hash));

        match self.variant.to_enum() {
            CatFileVariant::Type => {
                println!("{}", header.tipe.to_str());
            }
            CatFileVariant::Exists => (), // `hash` has been verified to exist (asserted with `dereference()`)
            CatFileVariant::Size => {
                println!("{}", header.size);
            }
            CatFileVariant::PrettyPrint => {
                let object = cli_expect!(read_object(&hash));
                print!("{}", cli_expect!(object.pretty_print()));
            }
        }

        ExitType::Ok
    }
}
