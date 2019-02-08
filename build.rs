use std::{env, error::Error, fs, path::PathBuf};

use mdbook::MDBook;

fn main() -> Result<(), Box<Error>> {
    let book_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("book");

    // build the book
    MDBook::load(&book_dir)?.build()?;

    let src = book_dir.join("book");

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    let mut path = out_dir.parent();
    let path = loop {
        let p = path.unwrap();

        if p.file_name().map(|f| f == "target") == Some(true) {
            break p;
        }

        path = p.parent();
    };

    let dest = path
        .join("doc")
        .join(env::var("CARGO_PKG_NAME")?.replace("-", "_"))
        .join("book");

    fs::create_dir_all(&dest).ok();
    fs::remove_dir_all(&dest).ok();
    fs::rename(src, dest)?;

    Ok(())
}
