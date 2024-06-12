use ravif::*;

use crate::bytebuffer::ByteBuffer;


#[no_mangle]
pub unsafe extern "C" fn ravif__encode_rgba(
    pixels_rgba: *const rgb::RGBA<u8>,
    width: usize,
    height: usize,
    speed: u8,
    quality: f32) -> *mut ByteBuffer
{
    let imgRef = std::slice::from_raw_parts(pixels_rgba, (width * height * 4) as usize);
    let img = Img::new(imgRef, width, height);

    let res = Encoder::new()
    .with_quality(quality)
    .with_speed(speed) 
    .encode_rgba(img);
 
    let buf = ByteBuffer::from_vec(res.unwrap().avif_file);
    Box::into_raw(Box::new(buf))
}