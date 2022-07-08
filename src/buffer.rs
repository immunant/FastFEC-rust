use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn growString(str: *mut STRING) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct string_type {
    pub str_0: *mut libc::c_char,
    pub n: size_t,
}
pub type STRING = string_type;
pub type BufferRead = Option::<
    unsafe extern "C" fn(*mut libc::c_char, libc::c_int, *mut libc::c_void) -> size_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer {
    pub buffer: *mut libc::c_char,
    pub bufferSize: libc::c_int,
    pub bufferPos: libc::c_int,
    pub streamStarted: libc::c_int,
    pub bufferRead: BufferRead,
}
pub type BUFFER = buffer;
#[no_mangle]
pub unsafe extern "C" fn newBuffer(
    mut bufferSize: libc::c_int,
    mut bufferRead: BufferRead,
) -> *mut BUFFER {
    let mut buffer: *mut BUFFER = malloc(
        ::std::mem::size_of::<BUFFER>() as libc::c_ulong,
    ) as *mut BUFFER;
    (*buffer).bufferSize = bufferSize;
    (*buffer).bufferPos = 0 as libc::c_int;
    let ref mut fresh0 = (*buffer).buffer;
    *fresh0 = malloc(bufferSize as libc::c_ulong) as *mut libc::c_char;
    (*buffer).streamStarted = 0 as libc::c_int;
    let ref mut fresh1 = (*buffer).bufferRead;
    *fresh1 = bufferRead;
    return buffer;
}
#[no_mangle]
pub unsafe extern "C" fn freeBuffer(mut buffer: *mut BUFFER) {
    free((*buffer).buffer as *mut libc::c_void);
    free(buffer as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn fillBuffer(
    mut buffer: *mut BUFFER,
    mut data: *mut libc::c_void,
) -> size_t {
    (*buffer).bufferPos = 0 as libc::c_int;
    let mut bytesRead: libc::c_int = ((*buffer).bufferRead)
        .expect(
            "non-null function pointer",
        )((*buffer).buffer, (*buffer).bufferSize, data) as libc::c_int;
    (*buffer).bufferSize = bytesRead;
    return bytesRead as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn readLine(
    mut buffer: *mut BUFFER,
    mut string: *mut STRING,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut eof: libc::c_int = 0 as libc::c_int;
    if (*buffer).streamStarted == 0 {
        if fillBuffer(buffer, data) == 0 as libc::c_int as libc::c_ulong {
            eof = 1 as libc::c_int;
        }
        (*buffer).streamStarted = 1 as libc::c_int;
    }
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut stringStart: libc::c_int = 0 as libc::c_int;
    let mut start: libc::c_int = (*buffer).bufferPos;
    loop {
        if (*buffer).bufferPos >= (*buffer).bufferSize {
            memcpy(
                ((*string).str_0).offset(stringStart as isize) as *mut libc::c_void,
                ((*buffer).buffer).offset(start as isize) as *const libc::c_void,
                ((*buffer).bufferPos - start) as libc::c_ulong,
            );
            stringStart += (*buffer).bufferPos - start;
            if fillBuffer(buffer, data) == 0 as libc::c_int as libc::c_ulong {
                eof = 1 as libc::c_int;
            }
            start = (*buffer).bufferPos;
        }
        let mut c: libc::c_char = (if eof != 0 {
            '\u{0}' as i32
        } else {
            *((*buffer).buffer).offset((*buffer).bufferPos as isize) as libc::c_int
        }) as libc::c_char;
        let ref mut fresh2 = (*buffer).bufferPos;
        *fresh2 += 1;
        while (n + 2 as libc::c_int) as libc::c_ulong > (*string).n {
            growString(string);
        }
        let mut end: libc::c_int = (c as libc::c_int == '\n' as i32) as libc::c_int;
        if end != 0 {
            n += 1;
        }
        if end != 0 || eof != 0 {
            *((*string).str_0).offset(n as isize) = '\u{0}' as i32 as libc::c_char;
        }
        n += 1;
        if !(end != 0 || eof != 0) {
            continue;
        }
        memcpy(
            ((*string).str_0).offset(stringStart as isize) as *mut libc::c_void,
            ((*buffer).buffer).offset(start as isize) as *const libc::c_void,
            ((*buffer).bufferPos - start - 1 as libc::c_int
                + (if end != 0 { 1 as libc::c_int } else { 0 as libc::c_int }))
                as libc::c_ulong,
        );
        break;
    }
    return n - 1 as libc::c_int;
}
