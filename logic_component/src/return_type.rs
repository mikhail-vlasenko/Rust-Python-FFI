#[repr(C)]
pub struct ReturnType {
    pub array: [i32; 100],
    pub tuple: (i32, i32),
}
