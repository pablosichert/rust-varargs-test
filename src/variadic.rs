#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, register_tool)]
pub type __builtin_va_list = *mut libc::c_char;
pub type va_list = __builtin_va_list;
#[no_mangle]
pub unsafe extern "C" fn variadic_va_list(mut context: *mut libc::c_void,
                                          mut callback:
                                              Option<unsafe extern "C" fn(_:
                                                                              *mut libc::c_void,
                                                                          _:
                                                                              libc::c_int)
                                                         -> ()>,
                                          mut n: libc::c_int,
                                          mut arguments: va_list) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        let mut argument: libc::c_int = arguments.arg::<libc::c_int>();
        callback.expect("non-null function pointer")(context, argument);
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn variadic(mut context: *mut libc::c_void,
                                  mut callback:
                                      Option<unsafe extern "C" fn(_:
                                                                      *mut libc::c_void,
                                                                  _:
                                                                      libc::c_int)
                                                 -> ()>, mut n: libc::c_int,
                                  mut arguments: libc::c_int, mut args: ...) {
    if n < 1 as libc::c_int { return }
    callback.expect("non-null function pointer")(context, arguments);
    let mut list: va_list = 0 as *mut libc::c_char;
    list = args.clone();
    variadic_va_list(context, callback, n - 1 as libc::c_int, list);
}
