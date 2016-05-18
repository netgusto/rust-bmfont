extern crate bmfont;

use bmfont::{ parse as bmparse };
use std::path::{ PathBuf, Path };
use std::env;

fn main() {
    let cwd: PathBuf = env::current_dir().unwrap();
    let assetspath: PathBuf = PathBuf::from(cwd).join(Path::new("examples/assets"));
    let fontdescriptorpath: PathBuf = assetspath.join(Path::new("font.fnt"));

    let res = bmparse(fontdescriptorpath);
    println!("{:?}", res);
}