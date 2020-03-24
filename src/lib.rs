#![feature(c_variadic)]

pub unsafe extern "C" fn varargs(
    n: usize,
    string: *const std::os::raw::c_char,
    mut args: ...
) -> Vec<*const std::os::raw::c_char> {
    let mut result = vec![string];
    let mut varargs_list = args.as_va_list();

    for _ in 0..n {
        let arg = varargs_list.arg::<*const std::os::raw::c_char>();
        result.push(arg);
    }

    result
}
