let AGHash: u32 = 0x0B000500u;
let AGMask: u32 = 0xFF00FF00u;
let BRHash: u32 = 0x00000003u;
let BRMask: u32 = 0x00FF00FFu;

struct Vector {
    data: array<u32>,
};

@group(0) @binding(0) var<storage, read>  in: Vector;
@group(0) @binding(1) var<storage, read_write> out: Vector;

//@compute @workgroup_size(1)
//fn main(@builtin(global_invocation_id) i: vec3<u32>) {
////    out.data[i.x] = 255u;
////& ((in.data[i.x] * BRHash) & BRMask);
//    out.data[i.x*5u] = ((in.data[i.x] & BRMask) * BRHash);
//    out.data[i.x*5u+1u] = in.data[i.x];
//}

@compute @workgroup_size(256)
fn main(@builtin(global_invocation_id) i: vec3<u32>) {
    out.data[i.x] = i.x;
//    out.data[i.x*2u] = ((in.data[i.x] * 3u) & (0xFFu << 24u)) |
//    (((in.data[i.x] >> 8u) * 5u) & (0xFFu << 16u)) |
//    (((in.data[i.x] >> 16u) * 7u) & (0xFFu << 8u)) |
//    (((in.data[i.x] >> 24u) * 11u) & 0xFFu);
//
//    out.data[i.x*2u+1u] = in.data[i.x];
}