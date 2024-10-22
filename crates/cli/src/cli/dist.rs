use std::{
    env,
    fs::{self, File},
    io::{BufWriter, Read, Write},
    path::PathBuf,
};

use anyhow::{Context, Result};
use clap::{Args, ValueHint};
use tracing::{debug, error, info};
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
        let project = Project::load(&path).with_context(|| {
            format!(
                "Failed to load Factorio mod project from {}",
                path.display()
            )
        })?;

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
        let zip_inner_prefix = PathBuf::from(&dist_name);

        info!("Target dist zip: {}", zip_path.display());

        let zip_file = File::create(&zip_path).context("Failed to create ZIP file for writing")?;
        let writer = BufWriter::new(zip_file);
        let mut zip = ZipWriter::new(writer);
        let options = zip::write::SimpleFileOptions::default();

        let mut overrides = ignore::overrides::OverrideBuilder::new(&project.mod_path);
        overrides
            .add("!/dist/")
            .context("Failed to add dist dir to ignore overrides")?;
        let overrides = overrides
            .build()
            .context("Failed to build ignore overrides")?;
        let mut builder = ignore::WalkBuilder::new(&project.mod_path);
        builder.overrides(overrides);
        let walker = builder.build();
        for entry in walker {
            match entry {
                Ok(path) if path.path().is_file() => {
                    let rel_path = path
                        .path()
                        .strip_prefix(&project.mod_path)
                        .context("Failed to strip mod path prefix")?;
                    let zip_path = zip_inner_prefix.join(rel_path);
                    let zip_path_str = zip_path
                        .to_str()
                        .context("Failed to convert zip path to str")?;
                    info!(
                        "Adding path {} to ZIP as {}",
                        path.path().display(),
                        zip_path.display()
                    );
                    zip.start_file(zip_path_str, options).with_context(|| {
                        format!("Failed to start adding {} to ZIP", rel_path.display())
                    })?;
                    let mut file =
                        File::open(path.path()).context("Failed to open file for reading")?;
                    let mut buffer = Vec::new();
                    file.read_to_end(&mut buffer)
                        .context("Failed to read file contents")?;
                    zip.write(&buffer)
                        .context("Failed to write file contents to ZIP")?;
                }
                Ok(path) => debug!("Ignoring non-file: {}", path.path().display()),
                Err(e) => error!("Glob error: {:?}", e),
            }
        }

        debug!("Finishing ZIP file");
        zip.finish().context("Failed to finish ZIP file")?;

        info!(
            "Finished packing mod into ZIP package: {}",
            zip_path.display()
        );

        Ok(())
    }
}
