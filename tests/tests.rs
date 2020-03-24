macro_rules! test {
    ($item:item) => {
        #[cfg(target_arch = "wasm32")]
        #[wasm_bindgen_test::wasm_bindgen_test]
        $item

        #[cfg(not(target_arch = "wasm32"))]
        #[test]
        $item
    }
}

test! {
    fn varargs() {
        let args = unsafe {
            varargs::varargs(
                2,
                "foo\0".as_ptr() as *const std::os::raw::c_char,
                "bar\0".as_ptr() as *const std::os::raw::c_char,
                "baz\0".as_ptr() as *const std::os::raw::c_char,
            )
        };

        let args = args
            .into_iter()
            .map(|arg| unsafe { std::ffi::CStr::from_ptr(arg).to_string_lossy() })
            .collect::<Vec<_>>();

        assert_eq!(args, vec!["foo", "bar", "baz"]);
    }
}
