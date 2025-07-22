use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {

    Add {

        date: String,
        amount: String,
        category: String,
        #[arg(short, long)]
        note: Option<String>,
    },
    List {
    },

}
