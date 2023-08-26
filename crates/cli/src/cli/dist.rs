use std::{
    env,
    fs::{self, File},
    io::{BufWriter, Write},
    path::PathBuf,
};

use anyhow::{Context, Result};
use clap::{Args, ValueHint};
use tracing::{debug, info};
use zip::ZipWriter;

use crate::project::Project;

#[derive(Args, Debug)]
pub struct DistArgs {
    #[arg(long, env = "FACTI_DIST_PATH", value_hint = ValueHint::FilePath)]
    path: Option<PathBuf>,

    #[arg(short, long)]
    clean: bool,
}

impl DistArgs {
    pub fn run(&self) -> Result<()> {
        let path = self
            .path
            .to_owned()
            .unwrap_or(env::current_dir().context("Failed to get current directory")?);
        let project = Project::load(&path)?;

        info!(
            "Loaded project ({} v{} by {}) at {}",
            project.mod_info.name,
            project.mod_info.version,
            project.mod_info.author,
            project.path.display()
        );
        info!("Mod source files at {}", project.mod_path.display());

        let dist_path = project.dist_path();

        if dist_path.exists() && self.clean {
            info!("Cleaning dist dir {}", dist_path.display());
            fs::remove_dir_all(&dist_path).context("Failed to remove dist dir")?;
        }

        fs::create_dir_all(&dist_path).context("Failed to create dist dir")?;
        info!("Dist files will be written to {}", dist_path.display());

        let dist_name = format!("{}_{}", project.mod_info.name, project.mod_info.version);
        let zip_name = format!("{}.zip", dist_name);
        let zip_path = dist_path.join(zip_name);

        info!("Target dist zip: {}", zip_path.display());

        let zip_file = File::create(&zip_path).context("Failed to create ZIP file for writing")?;
        let writer = BufWriter::new(zip_file);
        let mut zip = ZipWriter::new(writer);
        let options = zip::write::FileOptions::default();
        zip.add_directory(&dist_name, options)
            .context("Failed to add dist_name inner dir to ZIP")?;
        zip.start_file(format!("{}/info.json", dist_name), options)
            .context("Failed to start adding info.json to ZIP")?;

        let infojson_str = serde_json::to_string_pretty(&project.mod_info)
            .context("Failed to serialize mod info")?;
        let infojson_bytes = infojson_str.as_bytes();
        info!("Writing info.json to ZIP package");
        zip.write_all(infojson_bytes)
            .context("Failed to write mod info to ZIP")?;

        debug!("Finishing ZIP file");
        zip.finish().context("Failed to finish ZIP file")?;

        info!(
            "Finished packing mod into ZIP package: {}",
            zip_path.display()
        );

        Ok(())
    }
}
