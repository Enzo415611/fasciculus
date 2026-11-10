use alpm::{Alpm, SigLevel};
use size::Size;
use slint::{Image, ModelRc, SharedString, ToSharedString, VecModel};

use crate::SPackage;

pub fn get_pkg_by_name(package_name: &str, default_icon: Image) -> Option<SPackage> {
    if let Ok(alpm) = sync_alpm_dbs() {
        let syncdbs = alpm.syncdbs();
        for db in syncdbs.iter() {
            if let Ok(pkg) = db.pkg(package_name) {
                let builder = freedesktop_icons::lookup(pkg.name()).with_size(1).find();
                let image = if let Some(path) = builder {
                    slint::Image::load_from_path(&path).unwrap()
                } else {
                    // let path =
                    //     Path::new(env!("CARGO_MANIFEST_DIR")).join("/ui/icons/cross-small.png");
                    // println!(env!("CARGO_MANIFEST_DIR"));
                    // slint::Image::load_from_path(&path).unwrap()
                    default_icon
                };
                return Some(SPackage {
                    name: pkg.name().to_shared_string(),
                    icon: image,
                    desc: pkg.desc().unwrap_or_default().to_shared_string(),
                    url: pkg.url().unwrap_or_default().to_shared_string(),
                    size: Size::from_bytes(pkg.size())
                        .format()
                        .with_base(size::Base::Base10)
                        .to_shared_string(),
                    depends: ModelRc::new(VecModel::from(
                        pkg.depends()
                            .into_iter()
                            .map(|d| d.name().to_shared_string())
                            .collect::<Vec<SharedString>>(),
                    )),
                    db: pkg.db().map(|db| db.name().to_shared_string()).unwrap(),
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
