use libc::{c_char, c_int, c_void, c_ulong, size_t, ssize_t};

pub enum stream_t {}
pub enum block_t {}
pub enum vlc_object_t {}

extern "C" {

    pub fn vlc_stream_Read(s : *mut stream_t, buf: *mut c_void, len: size_t, ) -> ssize_t;

    pub fn vlc_stream_ReadBlock(s: *mut stream_t) -> *mut block_t;

    pub fn vlc_stream_Tell(s: *const stream_t) -> u64;

    pub fn vlc_stream_Eof(s: *const stream_t) -> bool;

    pub fn vlc_stream_Seek(s: *mut stream_t, offset: u64) -> c_int;

    pub fn vlc_stream_vaControl(s: *mut stream_t, query: c_int, ...) -> c_int;

    pub fn vlc_stream_ReadLine(s: *mut stream_t) -> *mut c_char;

    pub fn vlc_stream_Delete(s: *mut stream_t);

    pub fn vlc_stream_GetSize(s: *mut stream_t, size: *mut c_ulong) -> c_int;

    pub fn stream_HasExtension(s: *mut stream_t, extension: *const c_char) -> bool;

    pub fn vlc_stream_NewURL(obj: *mut vlc_object_t, url: *const c_char, ) -> *mut stream_t;
}
