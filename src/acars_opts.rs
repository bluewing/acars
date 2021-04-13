extern crate structopt;

use self::structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Acars")]
pub struct AcarsOpts {
    #[structopt(long)]
    pub webhook_url: String
}
