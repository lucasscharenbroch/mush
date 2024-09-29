use crate::cli::MushSubcommand;
use crate::cli::ExitType;
use crate::cli_expect;
use crate::object::Object;
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

#[derive(Clone, clap::ValueEnum)]
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
        let object_filename = crate::dot_mush_slash!(hash.path());
        let file = crate::open_filename!(object_filename, "get object header");
        let header = crate::cli_expect!(crate::object::ObjectHeader::extract_from_file(file, &hash));

        match self.variant.to_enum() {
            CatFileVariant::Type => {
                println!("{}", header.tipe.to_str());
            },
            CatFileVariant::Exists => (), // `hash` has been verified to exist (asserted with `dereference()`)
            CatFileVariant::Size => {
                println!("{}", header.size);
            },
            CatFileVariant::PrettyPrint => {
                let object_contents_str = crate::read_filename_to_bytes!(object_filename, "read object");
                println!("{}", cli_expect!(Object::from_compressed_bytes(&object_contents_str)));
            },
        }

        ExitType::Ok
    }
}
