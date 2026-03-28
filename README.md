This is a fork of the [moxcms](https://crates.io/crates/moxcms) crate. The git repository is located at https://github.com/awxkee/moxcms.

# Rust ICC Management

Fast and safe conversion between ICC profiles; in pure Rust.

Supports CMYK⬌RGBX, RGBX⬌RGBX, RGBX⬌GRAY, LAB⬌RGBX and CMYK⬌LAB, GRAY⬌RGB, any 3/4 color profiles to RGB and vice versa. Also supports almost any to any Display Class ICC profiles up to 16 inks.

## Example

```rust
let f_str = "./assets/dci_p3_profile.jpeg";
let file = File::open(f_str).expect("Failed to open file");

let img = image::ImageReader::open(f_str).unwrap().decode().unwrap();
let rgb = img.to_rgb8();

let mut decoder = JpegDecoder::new(BufReader::new(file)).unwrap();
let icc = decoder.icc_profile().unwrap().unwrap();
let color_profile = ColorProfile::new_from_slice(&icc).unwrap();
let dest_profile = ColorProfile::new_srgb();
let transform = color_profile
    .create_transform_8bit(&dest_profile, Layout::Rgb8, TransformOptions::default())
    .unwrap();
let mut dst = vec![0u8; rgb.len()];

for (src, dst) in rgb
    .chunks_exact(img.width() as usize * 3)
    .zip(dst.chunks_exact_mut(img.dimensions().0 as usize * 3))
{
    transform
        .transform(
            &src[..img.dimensions().0 as usize * 3],
            &mut dst[..img.dimensions().0 as usize * 3],
        )
        .unwrap();
}
image::save_buffer(
    "v1.jpg",
    &dst,
    img.dimensions().0,
    img.dimensions().1,
    image::ExtendedColorType::Rgb8,
)
    .unwrap();
```

## Benchmarks

### ICC Transform 8-Bit 

Tests were ran with a 1997×1331 resolution image.

| Conversion         | time(NEON) | Time(AVX2) |
|--------------------|:----------:|:----------:|
| moxcms RGB⮕RGB     |   2.68ms   |   4.52ms   |
| moxcms LUT RGB⮕RGB |   7.18ms   |  17.50ms   |
| moxcms RGBA⮕RGBA   |   2.96ms   |   4.83ms   |
| moxcms CMYK⮕RGBA   |  11.86ms   |  27.98ms   |
| lcms2 RGB⮕RGB      |   13.1ms   |  27.73ms   |
| lcms2 LUT RGB⮕RGB  |  27.60ms   |  58.26ms   |
| lcms2 RGBA⮕RGBA    |  21.97ms   |  35.70ms   |
| lcms2 CMYK⮕RGBA    |  39.71ms   |  79.40ms   |
| qcms RGB⮕RGB       |   6.47ms   |   4.59ms   |
| qcms LUT RGB⮕RGB   |  26.72ms   |  60.80ms   |
| qcms RGBA⮕RGBA     |   6.83ms   |   4.99ms   |
| qcms CMYK⮕RGBA     |  25.97ms   |  61.54ms   |

## License

This project is licensed under either of

- BSD-3-Clause License (see [LICENSE](LICENSE.md))
- Apache License, Version 2.0 (see [LICENSE](LICENSE-APACHE.md))

at your option.
