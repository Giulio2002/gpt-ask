#[macro_use]
extern crate serde_derive;

mod error;
mod env;
mod communication;
mod gpt;

use std::process::exit;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "cli")]
enum Cli {
    #[structopt(name = "ask")]
    Ask {
        /// Show the help manual
        #[structopt(short, long)]
        help: bool,

        /// Show the list of commands
        #[structopt(short, long)]
        question: String,
    },

    #[structopt(name = "refactor")]
    Refactor {
        /// Show the help manual
        #[structopt(short, long)]
        help: bool,

        /// Show the list of commands
        #[structopt(short, long)]
        file: String,
    },
}

#[tokio::main]
async fn main() {
    // Retreive key
    let api_key = match env::get_openai_key() {
        Ok(k) => k,
        Err(err) => {
            println!("$OPENAI_KEY is not set: {}", err.to_string());
            exit(0);
        }
    };

    let chat_gpt = gpt::OpenAIClient::new(
        api_key, 
        gpt::API_URL_DEFAULT.to_string(),
        gpt::MAX_TOKENS,
        gpt::DA_VINCI_MODEL.to_string(),
    );

    let opt_cli = Cli::from_args();
    match opt_cli {
        Cli::Ask { help, question } => {
            if help || question == "" {
                println!("Usage: --question=\"YOUR_QUESTION\"");
                exit(0);
            }

            let answer = match chat_gpt.ask(question).await {
                Ok(out) => out,
                Err(err) => {
                    println!("could not query openai: {}", err.to_string());
                    exit(0);
                }
            };
            println!("{}", answer);
        },
        Cli::Refactor { help, file } => {
            unimplemented!()
        }
    }
}
