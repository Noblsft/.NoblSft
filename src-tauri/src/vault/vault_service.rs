use crate::vault::errors::VaultError;
use crate::vault::types::Manifest;

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use zip::write::FileOptions;

pub struct VaultService {
    app_version: String,
    schema_version: u32,
}

impl VaultService {
    /// Create a new `VaultService`.
    ///
    /// `app_version` and `schema_version` are recorded in the vault's
    /// `manifest.json` when a vault is created. This type is a lightweight
    /// factory that encapsulates those values for use by `create_vault`.
    ///
    /// Parameters:
    /// - `app_version`: human-readable application version to embed in the manifest.
    /// - `schema_version`: numeric schema version to embed in the manifest.
    ///
    /// Returns: a configured `VaultService` instance.
    pub fn new(app_version: String, schema_version: u32) -> Self {
        Self { app_version, schema_version }
    }

    /// Create an empty vault file at the given `target` path.
    ///
    /// The method performs the following steps:
    /// - Ensures the parent directory of `target` exists (creating it if needed).
    /// - Generates a temporary file next to `target` (hidden file named `.<filename>.tmp`).
    /// - Writes a ZIP archive into the temporary file which contains an empty
    ///   `user-files/` directory and a `manifest.json` file. The manifest embeds
    ///   this service's `app_version` and `schema_version` and is serialized as
    ///   pretty-formatted JSON.
    /// - Uses DEFLATED compression and sets entry permissions to 0o644.
    /// - Atomically replaces any existing `target` file with the newly created
    ///   temporary file (removing the existing target first if present).
    ///
    /// Parameters:
    /// - `target`: a path (or path-like) where the vault ZIP will be created.
    ///
    /// Errors:
    /// Returns `Err(VaultError)` if any IO, serialization, or ZIP operation fails.
    ///
    /// Example:
    /// ```no_run
    /// let svc = VaultService::new("1.0.0".into(), 1);
    /// svc.create_vault("/path/to/vault.zip")?;
    /// ```
    pub fn create_vault(&self, target: impl AsRef<Path>) -> Result<(), VaultError> {
        let target = target.as_ref().to_path_buf();
        let parent = target.parent().unwrap_or_else(|| Path::new("."));
        fs::create_dir_all(parent)?;

        let tmp = temp_path_next_to(&target)?;
        if tmp.exists() {
            fs::remove_file(&tmp)?;
        }

        let manifest = Manifest::new(&self.app_version.clone(), self.schema_version);
        let manifest_bytes = serde_json::to_vec_pretty(&manifest)?;

        {
            let f = File::create(&tmp)?;
            let mut zip = zip::ZipWriter::new(f);

            let opts: FileOptions<'_, zip::write::ExtendedFileOptions> = FileOptions::default()
                .compression_method(zip::CompressionMethod::Deflated)
                .unix_permissions(0o644);

            zip.add_directory("user-files/", opts.clone())?;

            zip.start_file("manifest.json", opts.clone())?;
            zip.write_all(&manifest_bytes)?;

            zip.finish()?;
        }

        atomic_replace(&tmp, &target)?;
        Ok(())
    }
}

fn temp_path_next_to(path: &Path) -> Result<PathBuf, VaultError> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let file_name = path
        .file_name()
        .ok_or_else(|| VaultError::InvalidPath("missing filename".into()))?
        .to_string_lossy();
    Ok(parent.join(format!(".{}.tmp", file_name)))
}

fn atomic_replace(tmp: &Path, target: &Path) -> Result<(), VaultError> {
    if target.exists() {
        fs::remove_file(target)?;
    }
    fs::rename(tmp, target)?;
    Ok(())
}
