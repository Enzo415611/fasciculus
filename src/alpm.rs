use alpm::{Alpm, SigLevel};

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
}

pub fn get_pkg_by_name(alpm: &Alpm, package_name: &str) -> Option<Package> {
    for db in alpm.syncdbs() {
        if let Ok(pkg) = db.pkg(package_name) {
            return Some(Package {
                name: pkg.name().to_string(),
            });
        }
    }
    None
}

pub fn sync_alpm_dbs() -> anyhow::Result<Alpm> {
    let mut handle = Alpm::new("/", "/var/lib/pacman/")?;
    for db in get_dbs()? {
        handle.register_syncdb_mut(db, SigLevel::USE_DEFAULT)?;
    }
    Ok(handle)
}

fn get_dbs() -> anyhow::Result<Vec<String>> {
    let mut dbs: Vec<String> = Vec::new();
    for entry in std::fs::read_dir("/var/lib/pacman/sync/")? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().unwrap_or_default() == "db" {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                dbs.push(stem.to_string());
            }
        }
    }
    Ok(dbs)
}
