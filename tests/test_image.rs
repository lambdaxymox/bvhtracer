extern crate bvhtracer;


use bvhtracer::{
    ImageBuffer,
    Rgba,
};


#[test]
fn test_image_buffer1() {
    let data: Vec<u8> = vec![
        0, 0, 0, 255, 1, 1, 1, 255, 2, 2, 2, 255, 3, 3, 3, 255, 4, 4, 4, 255, 5, 5, 5, 255, 6, 6, 6, 255
    ];
    let buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(7, 1, data).unwrap();

    assert_eq!(buffer.get_pixel_unchecked(0, 0), &Rgba::new(0, 0, 0, 255));
    assert_eq!(buffer.get_pixel_unchecked(1, 0), &Rgba::new(1, 1, 1, 255));
    assert_eq!(buffer.get_pixel_unchecked(2, 0), &Rgba::new(2, 2, 2, 255));
    assert_eq!(buffer.get_pixel_unchecked(3, 0), &Rgba::new(3, 3, 3, 255));
    assert_eq!(buffer.get_pixel_unchecked(4, 0), &Rgba::new(4, 4, 4, 255));
    assert_eq!(buffer.get_pixel_unchecked(5, 0), &Rgba::new(5, 5, 5, 255));
    assert_eq!(buffer.get_pixel_unchecked(6, 0), &Rgba::new(6, 6, 6, 255));
}

#[test]
fn test_image_buffer2() {
    let data: Vec<u8> = vec![
        0, 0, 0, 255, 1, 1, 1, 255, 2, 2, 2, 255, 3,  3,  3, 255,  4,  4,  4,  255, 5,  5,  5,  255, 6,  6,  6,  255,
        7, 7, 7, 255, 8, 8, 8, 255, 9, 9, 9, 255, 10, 10, 10, 255, 11, 11, 11, 255, 12, 12, 12, 255, 13, 13, 13, 255
    ];
    let buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(7, 2, data).unwrap();

    assert_eq!(buffer.get_pixel_unchecked(0, 0), &Rgba::new(0,  0,  0,  255));
    assert_eq!(buffer.get_pixel_unchecked(1, 0), &Rgba::new(1,  1,  1,  255));
    assert_eq!(buffer.get_pixel_unchecked(2, 0), &Rgba::new(2,  2,  2,  255));
    assert_eq!(buffer.get_pixel_unchecked(3, 0), &Rgba::new(3,  3,  3,  255));
    assert_eq!(buffer.get_pixel_unchecked(4, 0), &Rgba::new(4,  4,  4,  255));
    assert_eq!(buffer.get_pixel_unchecked(5, 0), &Rgba::new(5,  5,  5,  255));
    assert_eq!(buffer.get_pixel_unchecked(6, 0), &Rgba::new(6,  6,  6,  255));
    assert_eq!(buffer.get_pixel_unchecked(0, 1), &Rgba::new(7,  7,  7,  255));
    assert_eq!(buffer.get_pixel_unchecked(1, 1), &Rgba::new(8,  8,  8,  255));
    assert_eq!(buffer.get_pixel_unchecked(2, 1), &Rgba::new(9,  9,  9,  255));
    assert_eq!(buffer.get_pixel_unchecked(3, 1), &Rgba::new(10, 10, 10, 255));
    assert_eq!(buffer.get_pixel_unchecked(4, 1), &Rgba::new(11, 11, 11, 255));
    assert_eq!(buffer.get_pixel_unchecked(5, 1), &Rgba::new(12, 12, 12, 255));
    assert_eq!(buffer.get_pixel_unchecked(6, 1), &Rgba::new(13, 13, 13, 255));
}

#[test]
fn test_image_buffer3() {
    let data: Vec<u8> = vec![
        255, 0, 0,   255, 0,   255, 0, 255, 
        0,   0, 255, 255, 255, 255, 0, 255,
    ];
    let buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(2, 2, data).unwrap();

    assert_eq!(buffer.get_pixel_unchecked(0, 0), &Rgba::new(255, 0,   0,   255));
    assert_eq!(buffer.get_pixel_unchecked(1, 0), &Rgba::new(0,   255, 0,   255));
    assert_eq!(buffer.get_pixel_unchecked(0, 1), &Rgba::new(0,   0,   255, 255));
    assert_eq!(buffer.get_pixel_unchecked(1, 1), &Rgba::new(255, 255, 0,   255));
}

#[test]
fn test_image_buffer4() {
    let data: Vec<u8> = vec![
        255, 0,   0, 255, 0,   255, 0,   255, 0, 0, 255, 255,
        255, 255, 0, 255, 255, 255, 255, 255, 0, 0, 0,   255
    ];
    let buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(3, 2, data).unwrap();

    assert_eq!(buffer.get_pixel_unchecked(0, 0), &Rgba::new(255, 0,   0,   255));
    assert_eq!(buffer.get_pixel_unchecked(1, 0), &Rgba::new(0,   255, 0,   255));
    assert_eq!(buffer.get_pixel_unchecked(2, 0), &Rgba::new(0,   0,   255, 255));
    assert_eq!(buffer.get_pixel_unchecked(0, 1), &Rgba::new(255, 255, 0,   255));
    assert_eq!(buffer.get_pixel_unchecked(1, 1), &Rgba::new(255, 255, 255, 255));
    assert_eq!(buffer.get_pixel_unchecked(2, 1), &Rgba::new(0,   0,   0,   255));
}

