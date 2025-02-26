use std::{fs, path::Path};

use anyhow::{Context, Result, anyhow};

pub fn dist<P>(artifacts: &[P], dist_dir: &Path) -> Result<()>
where
    P: AsRef<Path>,
{
    println!("Copying build artifacts to dist dir");
    artifacts
        .iter()
        .try_for_each(|x| {
            let filename = x.as_ref().file_name();
            if filename.is_none() {
                return Err(anyhow!("Build artifacts dont't have a filename"));
            }
            let mut file = dist_dir.to_path_buf();
            file.push(filename.unwrap());
            println!(
                "Copying {} to {}",
                x.as_ref().to_string_lossy(),
                file.to_string_lossy()
            );
            fs::create_dir_all(&dist_dir)?;
            fs::copy(x, &file)?;
            Ok(())
        })
        .context("Failed to copy the build artifacts to the dist directory")?;
    Ok(())
}
