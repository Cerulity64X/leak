#![feature(box_syntax)]
fn main() {
    loop {
        Box::leak(box[0u8;1024*1024*1024]);
    }
}