use anyhow::Result;
use clap::{Args, Subcommand};
use facti_api::blocking::ApiClient;
use tracing::info;

#[derive(Args, Debug)]
pub struct ImagesArgs {
    #[command(subcommand)]
    pub command: ImagesCommands,
}

#[derive(Subcommand, Debug)]
pub enum ImagesCommands {
    List(ListImagesArgs),
}

#[derive(Args, Debug)]
pub struct ListImagesArgs {
    pub name: String,
}

impl ImagesArgs {
    pub fn run(&self, client: &ApiClient) -> Result<()> {
        match &self.command {
            ImagesCommands::List(args) => args.run(client),
        }
    }
}

impl ListImagesArgs {
    pub fn run(&self, client: &ApiClient) -> Result<()> {
        info!("Fetching images for {}", self.name);
        let images = client.get_images(&self.name)?;

        for image in images {
            println!("{}: {}", image.id, image.url);
        }

        Ok(())
    }
}
