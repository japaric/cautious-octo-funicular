use std::{env, error::Error, fs, path::PathBuf};

use mdbook::MDBook;

fn main() -> Result<(), Box<dyn Error>> {
    let book_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("book");

    // build the book
    let mut mdbook = MDBook::load(&book_dir)?;

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    let mut path = out_dir.parent();
    let path = loop {
        let p = if let Some(p) = path {
            p
        } else {
            break PathBuf::from(env::var("CARGO_TARGET_DIR")?);
        };

        if p.file_name().map(|f| f == "debug") == Some(true) {
            break p.parent().unwrap().to_owned();
        }

        path = p.parent();
    };

    let dest = path
        .join("doc")
        .join(env::var("CARGO_PKG_NAME")?.replace("-", "_"))
        .join("book");

    fs::create_dir_all(&dest).ok();
    fs::remove_dir_all(&dest).ok();

    mdbook.config.build.build_dir = dest;
    mdbook.build()?;

    Ok(())
}
