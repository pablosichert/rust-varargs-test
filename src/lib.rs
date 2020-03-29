#![feature(c_variadic)]

pub unsafe extern "C" fn varargs(arg: u32, mut args: ...) -> [u32; 4] {
    let arg1 = arg;
    let arg2 = args.arg::<u32>();
    let arg3 = args.as_va_list().arg::<u32>();
    let arg4 = args.arg::<u32>();

    [arg1, arg2, arg3, arg4]
}
