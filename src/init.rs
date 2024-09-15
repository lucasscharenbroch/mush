use clap::Args;

use crate::MushSubcommand;

#[derive(Args)]
pub struct InitArgs {

}

impl MushSubcommand for InitArgs {
    fn execute(&self) {
        todo!()
    }
}
