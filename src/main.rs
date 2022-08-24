use std::fs;

use gpgpu::*;
use image::{EncodableLayout, RgbaImage};

use rapid_qoi::Qoi;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("hello");
    let b = fs::read("test_images/place.qoi")?;
    println!("{}", b.len());
    let (header, b) = Qoi::decode_alloc(b.as_bytes())?;
    println!("{}", header.height);
    let img = RgbaImage::from_raw(header.width, header.height, b).unwrap();
    println!("{}", img.height());
    // let img = [0, 0, 0, 255, 255, 255, 255, 255, 123, 231, 101, 255];
    let hashed = hash(&[170, 187, 204, 221]);
    let hashes = hashes(img.as_bytes());
    let a = (255 as u8).wrapping_mul(11);
    let b = (255 as u8).wrapping_mul(7);
    let g = (255 as u8).wrapping_mul(5);
    let r = (255 as u8).wrapping_mul(3);
    let hash = (a.wrapping_add(b).wrapping_add(g).wrapping_add(r)) % 64;
    Ok(())
}

fn hashes(img: &[u8]) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    // Framework initialization
    let fw = Framework::default();

    // Original CPU data
    // let cpu_data = img.as_bytes();
    let cpu_data: &[u32] = bytemuck::cast_slice(img.as_bytes());
    let _ayo = Vec::from(img);
    let _cpu_data = Vec::from(cpu_data);
    // GPU buffer creation
    let buf_a = GpuBuffer::from_slice(&fw, &cpu_data); // Input
    let buf_c = GpuBuffer::<u32>::with_capacity(&fw, (cpu_data.len() * 1) as u64); // Output

    //  or from a WGSL source file
    let shader = Shader::from_wgsl_file(&fw, "test_shaders/hashes.wgsl")?;

    // Descriptor set and program creation
    let desc = DescriptorSet::default()
        .bind_buffer(&buf_a, GpuBufferUsage::ReadOnly)
        .bind_buffer(&buf_c, GpuBufferUsage::ReadWrite);
    let program = Program::new(&shader, "main").add_descriptor_set(desc); // Entry point

    // Kernel creation and enqueuing
    Kernel::new(&fw, program).enqueue(256, 1, 1); // Enqueuing, not very optimus 😅

    let output = buf_c.read_vec_blocking()?; // Read back C from GPU
                                             // for (a, b) in cpu_data.into_iter().zip(output) {
                                             //     assert_eq!(a.pow(2), b);
                                             // }
    Ok(output)
}

fn hash(b: &[u8; 4]) -> u64 {
    let v = u32::from_ne_bytes(*b);
    let s = (((v as u64) << 32) | (v as u64)) & 0xFF00FF0000FF00FF;
    let le = 0x030007000005000Bu64.to_le();
    let multiplied = s.wrapping_mul(le);
    let swapped = multiplied.swap_bytes();
    return swapped & 63;
}
