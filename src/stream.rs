use ffi;
use ffi::stream::vlc_stream_vaControl;
use std::os::raw::c_char;
use std::ptr::null_mut;

struct Stream {
    pub stream : *mut ffi::stream::stream_t
}

pub fn vlc_stream_mime_type(s: &Stream) -> &str {
    
}

pub fn vlc_stream_content_type(s: &Stream) -> &str {
    const STREAM_GET_CONTENT_TYPE : i32 = 0;
    let result : &str = "";
    unsafe {
        let res : *mut c_char = null_mut();
        if vlc_stream_vaControl(s.stream,  STREAM_GET_CONTENT_TYPE, res) {
            return result
        }
        
    }
    ""
}