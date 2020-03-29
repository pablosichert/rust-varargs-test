#![feature(rustc_private)]

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
    fn variadic() {
        let mut arguments = Vec::new();

        unsafe extern "C" fn callback(context: *mut libc::c_void, number: libc::c_int) {
            let arguments = &mut *(context as *mut std::vec::Vec<libc::c_int>);
            arguments.push(number);
        }

        unsafe {
            variadic::variadic(
                &mut arguments as *mut Vec<libc::c_int> as *mut libc::c_void,
                Some(callback),
                4,
                0,
                1,
                2,
                3
            )
        };

        assert_eq!(arguments, [0, 1, 2, 3]);
    }
}
