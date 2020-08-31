extern crate bindgen;

fn main() {
  //look for libvlccore in the current directory while in development
  println!("cargo:rustc-link-search=native=.");

  let _ = bindgen::builder()
      .layout_tests(false)
      .clang_arg("-Iinclude")
      .header("include/vlc_common.h")
      .header("include/vlc_block.h")
      .header("include/vlc_stream.h")
      .whitelist_function("vlc_stream_NewURL")
      .whitelist_function("vlc_stream_Delete")
      .whitelist_function("vlc_stream_ReadBlock")
      .whitelist_function("vlc_stream_Read")
      .whitelist_function("vlc_stream_ReadLine")
      .whitelist_function("vlc_stream_Seek")
      .whitelist_function("vlc_stream_Eof")
      .whitelist_function("vlc_stream_Tell")
      .whitelist_function("vlc_stream_vaControl")
      .whitelist_function("stream_MimeType")
      .whitelist_function("stream_ContentType")
      .whitelist_function("stream_HasExtension")
      .whitelist_function("vlc_stream_GetSize")
      .use_core()
      .generate()
      .unwrap()
      .write_to_file("src/ffi/stream.rs");
}