use crate::vault::errors::VaultError;

use std::fs::{self, File};
use std::io;
use std::path::Path;

use zip::ZipArchive;

/// Extracts the ZIP archive located at `zip_path` into the directory `dest`.
///
/// The function will create directories as needed and write file entries,
/// preserving the archive's relative paths. Entries whose enclosed name cannot
/// be determined (e.g. dangerous paths) are skipped to avoid path traversal
/// (ZipSlip) attacks.
///
/// Parameters:
/// - `zip_path`: path to the `.zip` file to extract.
/// - `dest`: destination directory where contents will be written.
///
/// Errors:
/// Returns a `VaultError` if opening the file, reading the archive, creating
/// directories, or writing files fails.
pub fn extract_zip(zip_path: &Path, dest: &Path) -> Result<(), VaultError> {
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        let out_path = match entry.enclosed_name() {
            Some(p) => dest.join(p),
            None => continue,
        };

        if entry.is_dir() {
            fs::create_dir_all(&out_path)?;
        } else {
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut out = File::create(&out_path)?;
            io::copy(&mut entry, &mut out)?;
        }
    }

    Ok(())
}
