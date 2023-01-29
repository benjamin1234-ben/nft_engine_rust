use clap::{Parser, Subcommand};
use serde_json::json;
use rand::Rng;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Cli {
    /// Generate a Image folder containing NFT images.
    #[arg(short, long)]
    imggen : bool,
    #[command(subcommand)]
    command : Option<Commands>
}
#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate a Metadata folder containing NFT metadata files.
    Metagen {
        #[arg(short, long)]
        metadata : String
    }
}

fn img_gen() -> std::io::Result<()> {
    for _el in 1..11 {
        let from = PathBuf::from(r"C:\Users\benji\OneDrive\Documents\Projects\Web3.0\nft_engine_rust\nft.jpg");
        let mut to = PathBuf::from(r"C:\Users\benji\OneDrive\Documents\Projects\Web3.0\nft_engine_rust\NFTs\images");
        to.push(_el.to_string());
        to.set_extension("jpg");

        fs::copy(from, to)?;
    }

    Ok(())
}

fn meta_gen(metadata : &String) -> std::io::Result<()> {
    let swags : [&str; 3] = ["GOLD", "SILVER", "BRONZE"];
    let ext = ".jpg";

    for _el in 1..11 {
        let mut rng = rand::thread_rng();
        let swag = swags[rng.gen_range(0..3)];

        let json = json!({
            "name" : "DopeMonkey",
            "description" : "A day in the life of a dope monkey.",
            "image_url" : format!("{}{}{}",metadata, _el.to_string(), ext),
            "attributes" : [
                { "trait_type" : "swag", "value" : format!("{}", swag) }
            ]
        });

        let mut path = PathBuf::from(r"C:\Users\benji\OneDrive\Documents\Projects\Web3.0\nft_engine_rust\NFTs\metadata");
        path.push(_el.to_string());
        path.set_extension("json");
  
        println!("{:?}", path);
        println!("{}", json.to_string());
        fs::write(path, json.to_string())?;
    }

    Ok(())
}
fn main() {
    println!("Hello, World from Rust NFT Engine!");

    let cli = Cli::parse();

    println!("imggen : {}", cli.imggen);

    if cli.imggen {
        println!("Image Generator is ON");
        img_gen().unwrap();
    }

    match &cli.command {
        Some(Commands::Metagen { metadata }) => {
            println!("Metadata Generator is ON");
            meta_gen(metadata).unwrap();
        },
        None => {}
    }
}
