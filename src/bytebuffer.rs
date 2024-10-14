#[repr(C)]
pub struct ByteBuffer {
    pub ptr: *mut u8,
    pub length: i32,
    pub capacity: i32,
}

impl ByteBuffer {
    pub fn len(&self) -> usize {
        self.length
            .try_into()
            .expect("buffer length negative or overflowed")
    }

    pub fn from_vec(bytes: Vec<u8>) -> Self {
        let length = i32::try_from(bytes.len()).expect("buffer length cannot fit into a i32.");
        let capacity =
            i32::try_from(bytes.capacity()).expect("buffer capacity cannot fit into a i32.");

        // keep memory until call delete
        let mut v = std::mem::ManuallyDrop::new(bytes);

        Self {
            ptr: v.as_mut_ptr(),
            length,
            capacity,
        }
    }

    pub fn from_vec_struct<T: Sized>(bytes: Vec<T>) -> Self {
        let element_size = std::mem::size_of::<T>() as i32;

        let length = (bytes.len() as i32) * element_size;
        let capacity = (bytes.capacity() as i32) * element_size;

        let mut v = std::mem::ManuallyDrop::new(bytes);

        Self {
            ptr: v.as_mut_ptr() as *mut u8,
            length,
            capacity,
        }
    }

    pub fn destroy_into_vec(self) -> Vec<u8> {
        if self.ptr.is_null() {
            vec![]
        } else {
            let capacity: usize = self
                .capacity
                .try_into()
                .expect("buffer capacity negative or overflowed");
            let length: usize = self
                .length
                .try_into()
                .expect("buffer length negative or overflowed");

            unsafe { Vec::from_raw_parts(self.ptr, length, capacity) }
        }
    }

    pub fn destroy_into_vec_struct<T: Sized>(self) -> Vec<T> {
        if self.ptr.is_null() {
            vec![]
        } else {
            let element_size = std::mem::size_of::<T>() as i32;
            let length = (self.length * element_size) as usize;
            let capacity = (self.capacity * element_size) as usize;

            unsafe { Vec::from_raw_parts(self.ptr as *mut T, length, capacity) }
        }
    }

    pub fn destroy(self) {
        drop(self.destroy_into_vec());
    }
}

#[no_mangle]
pub unsafe extern "C" fn bytebuffer_free(buffer: *mut ByteBuffer) {
    let buf = Box::from_raw(buffer);
    buf.destroy();
}
/*

typedef void* EGLDisplay;
typedef int EGLenum;
typedef void* EGLAttrib
static inline EGLDisplay eglGetPlatformDisplay(EGLenum platform,
                          void *display_id, const EGLAttrib *attrib_list){

}

 */

#[no_mangle]
pub unsafe extern "C" fn eglGetPlatformDisplay(platform: u32, display_id: *mut std::ffi::c_void, attrib_list: *const std::ffi::c_void) -> *mut std::ffi::c_void
{
    std::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn main() -> i32
{
    0
}
