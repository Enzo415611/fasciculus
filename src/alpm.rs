use std::path::Path;

use alpm::{Alpm, SigLevel};

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub icon: Option<slint::Image>,
    pub desc: Option<String>,
    pub url: Option<String>,
    pub size: i64,
    pub depends: Vec<String>,
    pub db: Option<String>,
}

pub fn get_pkg_by_name(package_name: &str) -> Option<Package> {
    if let Ok(alpm) = sync_alpm_dbs() {
        let syncdbs = alpm.syncdbs();
        for db in syncdbs.iter() {
            if let Ok(pkg) = db.pkg(package_name) {
                let builder = freedesktop_icons::lookup(pkg.name()).find();
                let image = if let Some(path) = builder {
                    Some(slint::Image::load_from_path(&path).unwrap())
                } else {
                    None
                };
                return Some(Package {
                    name: pkg.name().to_string(),
                    icon: image,
                    desc: pkg.desc().map(|desc| desc.to_string()),
                    url: pkg.url().map(|url| url.to_string()),
                    size: pkg.size(),
                    depends: pkg
                        .depends()
                        .into_iter()
                        .map(|d| d.name().to_string())
                        .collect(),
                    db: pkg.db().map(|db| db.name().to_string()),
                });
            }
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
