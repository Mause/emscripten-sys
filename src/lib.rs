extern "C" {
    pub fn emscripten_asm_const_int(
        code: *const u8,
        sigPtr: *const u8,
        argBuf: *const u8,
    ) -> *mut u8;
}

#[test]
fn it_works() {
    const SNIPPET: &'static [u8] = b"{ console.log(\"hello\") }\x00";
    let empty_sig = unsafe { std::ffi::CString::from_vec_unchecked(vec![]) };

    unsafe {
        emscripten_asm_const_int(
            SNIPPET as *const _ as *const u8,
            empty_sig.as_ptr() as *const u8,
            std::ptr::null() as *const u8,
        )
    };

    assert_eq!(2 + 2, 4);
}
