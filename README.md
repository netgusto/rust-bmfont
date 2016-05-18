# rust-bmfont

Bitmap font parser for rustlang. Currently supports:

* `.fnt` (xml flavour); example generator: <http://kvazars.com/littera/>

## Usage

Returns a structure containing the informations contained in a `.fnt` (xml format), ready to be processed by a renderer.

```rust
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
```

## Examples

### Parse example

```bash
$ cargo run --example parse
```

## Sample renderer

```rust
// file: text_renderer.rs

extern crate graphics;
extern crate bmfont;

use opengl_graphics::{ GlGraphics, Texture as GlTexture };
use piston_window::{ Context, Image };

use graphics::DrawState;

use bmfont::{ BmFont };

pub struct TextRenderer {
    pub texture: GlTexture,
    map: BmFont
}

impl TextRenderer {

    pub fn new(font: BmFont) -> Self {
        TextRenderer {
            texture: GlTexture::from_path(&(font.pages[0]).file).unwrap(),
            map: font
        }
    }

    pub fn render(
        &self,
        text: String,
        center_x: f64,
        center_y: f64,
        color: [f32; 4],
        c: &Context,
        gl: &mut GlGraphics
    ) {

        let mut x = center_x;
        let y = center_y;

        for character in text.chars() {

            let charcode = character as u32;
            //let bmchar: &BmChar;

            if let Some(bmchar) = self.map.chars.get(&charcode) {
                Image::new_color([color[0], color[1], color[2], color[3]])
                    .src_rect([bmchar.x as i32, bmchar.y as i32, bmchar.width as i32, bmchar.height as i32])
                    .rect([x as f64, y + bmchar.yoffset as f64, bmchar.width as f64, bmchar.height as f64])
                    .draw(
                        &self.texture,
                        &DrawState::default(),
                        c.transform,
                        gl
                    );

                x += bmchar.width as f64 + bmchar.xoffset as f64;
            }
        }
    }
}
```

In your program, use the bitmap font renderer like so:

```rust
// [...]
mod text_renderer;
use text_renderer::TextRenderer;

// [...] During init
let cwd: PathBuf = env::current_dir().unwrap();
let assetspath: PathBuf = PathBuf::from(cwd).join(Path::new("assets/font"));
let fontdescriptorpath: PathBuf = assetspath.join(Path::new("myfont.fnt"));

let textr = TextRenderer::new(bmparse(fontdescriptorpath));

// [...] In your render method
const BLACK:[f32; 4] = [0.0, 0.0, 0.0, 1.0];

textr.render(
  String::from("Hello, World !"),  // The string to render
  200.0, // posx
  200.0,// posy
  BLACK, // Color
  &c,    // Piston window Context; passed in gl.draw callback; may be built using let ref c = Context::new_abs(viewwidth as f64, viewheight as f64);
  &mut gl    // GlGraphics object
);
```
