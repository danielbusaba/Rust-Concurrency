#![feature(test)]

use std::fs;
use rayon::prelude::*;

extern crate test;
use test::Bencher;

extern crate faster;
use faster::*;

const IMAGE_DIR: &str = "images";

fn div16_loop(image: &mut image::GrayImage)
{
    for i in 0 .. image.height()
    {
        for j in 0 .. image.width()
        {
            image.get_pixel_mut(j, i) [0] /= 16;
        }
    }
}

fn div16_iter(image: &mut image::GrayImage)
{
    image.iter_mut().for_each(
        | pixel |
        {
            *pixel /= 16;
        }
    );
}

fn div16_par(image: &mut image::GrayImage)
{
    image.par_iter_mut().for_each(
        | pixel |
        {
            *pixel /= 16;
        }
    );
}

fn div16_chnk(image: &mut image::GrayImage)
{
    let chunk_size = (image.width() * image.height() - 1) as usize / num_cpus::get() + 1;
    image.par_chunks_mut(chunk_size).for_each(
        | chunk |
        {
            chunk.iter_mut().for_each(
                | pixel |
                {
                    *pixel /= 16;
                }
            );
        }
    );
}

fn div16_simd(image: &mut image::GrayImage)
{
    image.simd_iter_mut(u8s(0)).simd_do_each(
        | mut pixel |
        {
            pixel /= 16;
        }
    );
}

fn div16_chnk_simd(image: &mut image::GrayImage)
{
    let chunk_size = (image.width() * image.height() - 1) as usize / num_cpus::get() + 1;
    image.par_chunks_mut(chunk_size).for_each(
        | chunk |
        {
            chunk.simd_iter_mut(u8s(0)).simd_do_each(
                | mut pixel |
                {
                    pixel /= 16;
                }
            );
        }
    );
}

#[bench]
fn bench_div16_loop(b: &mut Bencher) -> std::io::Result<()>
{
    for entry in fs::read_dir(IMAGE_DIR.to_owned() + &"/")?
    {
        let entry = entry?;
        let mut original = image::open(entry.path()).unwrap().to_luma();
        b.iter(|| div16_loop(&mut original));
    }

    Ok(())
}

#[bench]
fn bench_div16_iter(b: &mut Bencher) -> std::io::Result<()>
{
    for entry in fs::read_dir(IMAGE_DIR.to_owned() + &"/")?
    {
        let entry = entry?;
        let mut original = image::open(entry.path()).unwrap().to_luma();
        b.iter(|| div16_iter(&mut original));
    }

    Ok(())
}

#[bench]
fn bench_div16_par(b: &mut Bencher) -> std::io::Result<()>
{
    for entry in fs::read_dir(IMAGE_DIR.to_owned() + &"/")?
    {
        let entry = entry?;
        let mut original = image::open(entry.path()).unwrap().to_luma();
        b.iter(|| div16_par(&mut original));
    }

    Ok(())
}

#[bench]
fn bench_div16_chnk(b: &mut Bencher) -> std::io::Result<()>
{
    for entry in fs::read_dir(IMAGE_DIR.to_owned() + &"/")?
    {
        let entry = entry?;
        let mut original = image::open(entry.path()).unwrap().to_luma();
        b.iter(|| div16_chnk(&mut original));
    }

    Ok(())
}

#[bench]
fn bench_div16_simd(b: &mut Bencher) -> std::io::Result<()>
{
    for entry in fs::read_dir(IMAGE_DIR.to_owned() + &"/")?
    {
        let entry = entry?;
        let mut original = image::open(entry.path()).unwrap().to_luma();
        b.iter(|| div16_simd(&mut original));
    }

    Ok(())
}

#[bench]
fn bench_div16_chnk_simd(b: &mut Bencher) -> std::io::Result<()>
{
    for entry in fs::read_dir(IMAGE_DIR.to_owned() + &"/")?
    {
        let entry = entry?;
        let mut original = image::open(entry.path()).unwrap().to_luma();
        b.iter(|| div16_chnk_simd(&mut original));
    }

    Ok(())
}

fn main() -> std::io::Result<()>
{
    Ok(())
}