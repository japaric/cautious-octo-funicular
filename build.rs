use std::{env, fs, path::PathBuf};

use mdbook::MDBook;

fn main() {
    let book_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("book");

    // build the book
    MDBook::load(&book_dir).unwrap().build().unwrap();

    let src = book_dir.join("book");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

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
        .join(env::var("CARGO_PKG_NAME").unwrap().replace("-", "_"))
        .join("book");

    fs::create_dir_all(&dest).ok();
    fs::remove_dir_all(&dest).ok();
    fs::rename(src, dest).unwrap();
}
