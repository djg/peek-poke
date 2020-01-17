use peek_poke::{Peek, PeekPoke, Poke};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PeekPoke)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PeekPoke)]
pub struct Size {
    pub w: f32,
    pub h: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PeekPoke)]
pub struct Rect {
    pub point: Point,
    pub size: Size,
}

pub type PipelineSourceId = u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PeekPoke)]
pub struct PipelineId(pub PipelineSourceId, pub u32);

impl Default for PipelineId {
    fn default() -> Self {
        PipelineId(0, 0)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, PeekPoke)]
pub struct ClipChainId(pub u64, pub PipelineId);

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PeekPoke)]
pub enum ClipId {
    Clip(usize, PipelineId),
    ClipChain(ClipChainId),
}

impl Default for ClipId {
    fn default() -> Self {
        ClipId::Clip(!0, PipelineId::default())
    }
}

pub type ItemTag = (u64, u16);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PeekPoke)]
pub struct SpatialId(pub usize, PipelineId);

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PeekPoke)]
pub struct CommonItemProperties {
    pub clip_rect: Rect,
    pub clip_id: ClipId,
    pub spatial_id: SpatialId,
    pub hit_info: Option<ItemTag>,
    pub is_backface_visible: bool,
}

#[inline(never)]
unsafe fn test<T: Poke>(bytes: *mut u8, x: &T) -> *mut u8 {
    x.poke_into(bytes)
}

fn poke_into<T: Poke>(bytes: &mut Vec<u8>, x: &T) {
    bytes.reserve(<T>::MAX_SIZE);
    let ptr = bytes.as_mut_ptr();
    let new_ptr = unsafe { test(ptr, x) };
    let new_len = (new_ptr as usize) - (bytes.as_ptr() as usize);
    unsafe {
        bytes.set_len(new_len);
    }
}

fn peek_from<T: Copy + Peek>(bytes: &[u8]) -> T {
    assert!(bytes.len() >= <T>::MAX_SIZE);
    let ptr = bytes.as_ptr();
    let (result, new_ptr): (T, _) = unsafe { peek_poke::peek_from_uninit(bytes.as_ptr()) };
    let size = (new_ptr as usize) - (ptr as usize);
    assert!(size <= bytes.len());
    result
}

pub fn main() {
    let x = CommonItemProperties {
        clip_rect: Rect {
            point: Point { x: 1.0, y: 2.0 },
            size: Size { w: 4.0, h: 5.0 },
        },
        clip_id: ClipId::Clip(5, PipelineId(1, 2)),
        spatial_id: SpatialId(3, PipelineId(4, 5)),
        hit_info: Some((6, 7)),
        is_backface_visible: true,
    };
    let mut bytes = Vec::<u8>::new();
    poke_into(&mut bytes, &x);
    println!("{:?}", bytes);
    assert_eq!(
        bytes,
        vec![
            0u8, 0, 128, 63, 0, 0, 0, 64, 0, 0, 128, 64, 0, 0, 160, 64, 0, 5, 0, 0, 0, 0, 0, 0, 0,
            1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0, 1, 6, 0, 0, 0,
            0, 0, 0, 0, 7, 0, 1
        ]
    );
    let y: CommonItemProperties = peek_from(&bytes);
    println!("{:?}", y);
    assert_eq!(x, y);
}
