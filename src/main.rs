mod cga;

use crate::cga::*;

fn main() {
    let px = &CGA::up(1.0, 2.0, 3.0);
    let line = px ^ CGA::eo() ^ CGA::ei();
    let sphere = (CGA::eo() - CGA::ei()).Dual();
    println!("a point       : {}", px);
    println!("a line        : {}", line);
    println!("a sphere      : {}", sphere);
}
