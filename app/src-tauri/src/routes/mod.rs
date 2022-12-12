use std::{path::Path, io::{Cursor, self}, str::Bytes};

use anyhow::Result;
use rocket::{options, post, response::status::NotFound, serde::json::Json, State};
use tauri::{AppHandle, Wry};

use rocket::serde::Deserialize;
use zip::ZipArchive;

pub mod debug;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct BoostInfo {
    url: String,
}

fn download_files<T: io::Seek + io::Read>(mut archive: ZipArchive<T>, root_path: &str) -> Result<()> {
  println!("Extracting to {}", root_path);
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = Path::new(root_path).join(file.name());
        println!("File {} extracted to \"{}\"", file.name(), outpath.display());

        if (&*file.name()).ends_with('/') {
            std::fs::create_dir_all(&outpath).unwrap();
        } else {
            std::fs::create_dir_all(outpath.parent().unwrap()).unwrap();
            let mut outfile = std::fs::File::create(&outpath).unwrap();
            std::io::copy(&mut file, &mut outfile).unwrap();
        }
    }

    Ok(())
}

#[post("/install", data = "<url_of_boost>", format = "application/json")]
pub async fn install_boost(
    app: &State<AppHandle<Wry>>,
    url_of_boost: Json<BoostInfo>,
) -> Result<(), NotFound<String>> {
    let boost_dir = shellexpand::tilde("~/Library/Application Support/Arc/boosts/");

    let file_bytes = reqwest::get(&url_of_boost.url)
        .await
        .map_err(|_| NotFound("Could not download file".to_string()))?
        .bytes()
        .await
        .map_err(|_| NotFound("Could not download file".to_string()))?;

    let zip = zip::ZipArchive::new(std::io::Cursor::new(file_bytes))
        .map_err(|_| NotFound("Could not decompress file".to_string()))?;

    download_files(zip, boost_dir.as_ref()).map_err(|e| NotFound(format!("Could not load decompressed file: {e}")))?;

    Ok(())
}

/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[options("/<_..>")]
pub fn all_options() {
    /* Intentionally left empty */
}
