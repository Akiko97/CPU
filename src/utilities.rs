pub struct Utilities {}

impl Utilities {
    pub fn f32_to_u32(f: f32) -> u32 {
        unsafe { std::mem::transmute::<f32, u32>(f) }
    }

    pub fn f64_to_u64(f: f64) -> u64 {
        unsafe { std::mem::transmute::<f64, u64>(f) }
    }

    pub fn u32_to_f32(u: u32) -> f32 {
        unsafe { std::mem::transmute::<u32, f32>(u) }
    }

    pub fn u64_to_f64(u: u64) -> f64 {
        unsafe { std::mem::transmute::<u64, f64>(u) }
    }

    pub fn f32vec_to_u32vec(f: Vec<f32>) -> Vec<u32> {
        f.into_iter().map(|x| Self::f32_to_u32(x)).collect()
    }

    pub fn f64vec_to_u64vec(f: Vec<f64>) -> Vec<u64> {
        f.into_iter().map(|x| Self::f64_to_u64(x)).collect()
    }

    pub fn u32vec_to_f32vec(u: Vec<u32>) -> Vec<f32> {
        u.into_iter().map(|x| Self::u32_to_f32(x)).collect()
    }

    pub fn u64vec_to_f64vec(u: Vec<u64>) -> Vec<f64> {
        u.into_iter().map(|x| Self::u64_to_f64(x)).collect()
    }
}
