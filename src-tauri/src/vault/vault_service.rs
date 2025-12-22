use crate::vault::errors::VaultError;
use crate::vault::types::{ Manifest, VaultHandle };
use crate::helpers::zip::extract_zip;

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use zip::write::FileOptions;

pub struct VaultService {
    app_version: String,
    schema_version: u32,
    workspaces_root: PathBuf,
}

impl VaultService {
    /// Create a new `VaultService`.
    ///
    /// This constructs a lightweight factory that holds configuration used when
    /// creating and loading vaults. The `app_version` and `schema_version` are
    /// recorded in a vault's `manifest.json` when a vault is created. The
    /// `workspace_root` is the base directory used for extracted workspaces when
    /// loading a vault.
    ///
    /// Parameters:
    /// - `app_version`: human-readable application version to embed in the manifest.
    /// - `schema_version`: numeric schema version to embed in the manifest.
    /// - `workspace_root`: base path where temporary workspaces are created.
    ///
    /// Returns:
    /// A configured `VaultService` instance.
    pub fn new(app_version: String, schema_version: u32, workspaces_root: PathBuf) -> Self {
        Self {
            app_version,
            schema_version,
            workspaces_root,
        }
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

    /// Load a vault from `vault_path` into a newly created workspace under `self.workspaces_root`.
    ///
    /// The function will:
    /// - Verify that `vault_path` exists (returns `VaultError::InvalidPath` if not).
    /// - Ensure `self.workspaces_root` and the per-vault workspace directory exist.
    /// - Extract the ZIP archive at `vault_path` into the workspace via `extract_zip`.
    /// - Verify the presence of `manifest.json` in the workspace (returns
    ///   `VaultError::InvalidFormat` if missing).
    /// - Ensure the `user-files` directory exists (it will be created if absent).
    ///
    /// Returns a `VaultHandle` that points at the original `vault_path` and the
    /// extracted workspace on success. Other IO / ZIP / serialization failures are
    /// propagated as `VaultError`.
    pub fn load_vault(&self, vault_path: impl AsRef<Path>) -> Result<VaultHandle, VaultError> {
        let vault_path = vault_path.as_ref();

        if !vault_path.exists() {
            return Err(VaultError::InvalidPath("vault file does not exist".into()));
        }

        fs::create_dir_all(&self.workspaces_root)?;

        let workspace_id = unique_workspace_id();
        let workspace = self.workspaces_root.join(workspace_id);
        fs::create_dir_all(&workspace)?;

        extract_zip(vault_path, &workspace)?;

        let manifest = workspace.join("manifest.json");
        let user_files_dir = workspace.join("user-files");

        if !manifest.exists() {
            return Err(VaultError::InvalidFormat("missing manifest.json".into()));
        }
        if !user_files_dir.exists() {
            fs::create_dir_all(&user_files_dir)?;
        }

        Ok(VaultHandle {
            source: vault_path.to_path_buf(),
            workspace,
            manifest,
            objects_dir: user_files_dir,
        })
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

fn unique_workspace_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    format!("vault-{}", ts)
}
