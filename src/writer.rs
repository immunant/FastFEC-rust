use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn growStringTo(str: *mut STRING, newSize: size_t) -> libc::c_int;
    fn freeString(s: *mut STRING);
    fn newString(size: size_t) -> *mut STRING;
    static DEFAULT_STRING_SIZE: size_t;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct string_type {
    pub str_0: *mut libc::c_char,
    pub n: size_t,
}
pub type STRING = string_type;
pub type CustomWriteFunction = Option::<
    unsafe extern "C" fn(
        *mut libc::c_char,
        *mut libc::c_char,
        *mut libc::c_char,
        libc::c_int,
    ) -> (),
>;
pub type CustomLineFunction = Option::<
    unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_char, *mut libc::c_char) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer_file {
    pub buffer: *mut libc::c_char,
    pub bufferPos: libc::c_int,
    pub bufferSize: libc::c_int,
}
pub type BUFFER_FILE = buffer_file;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct write_context {
    pub bufferSize: libc::c_int,
    pub outputDirectory: *mut libc::c_char,
    pub filingId: *mut libc::c_char,
    pub filenames: *mut *mut libc::c_char,
    pub extensions: *mut *mut libc::c_char,
    pub bufferFiles: *mut *mut BUFFER_FILE,
    pub files: *mut *mut FILE,
    pub nfiles: libc::c_int,
    pub lastname: *mut libc::c_char,
    pub lastBufferFile: *mut BUFFER_FILE,
    pub lastfile: *mut FILE,
    pub local: libc::c_int,
    pub localBuffer: *mut STRING,
    pub localBufferPosition: libc::c_int,
    pub useCustomLine: libc::c_int,
    pub customLineBuffer: *mut STRING,
    pub customLineBufferPosition: libc::c_int,
    pub writeToFile: libc::c_int,
    pub customWriteFunction: CustomWriteFunction,
    pub customLineFunction: CustomLineFunction,
}
pub type WRITE_CONTEXT = write_context;
#[no_mangle]
pub static mut NUMBER_FORMAT: *const libc::c_char = b"%.2f\0" as *const u8
    as *const libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn mkdir_p(mut path: *const libc::c_char) -> libc::c_int {
    let len: size_t = strlen(path);
    let mut _path: [libc::c_char; 4096] = [0; 4096];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    *__errno_location() = 0 as libc::c_int;
    if len
        > (::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        *__errno_location() = 36 as libc::c_int;
        return -(1 as libc::c_int);
    }
    strcpy(_path.as_mut_ptr(), path);
    p = _path.as_mut_ptr().offset(1 as libc::c_int as isize);
    while *p != 0 {
        if *p as libc::c_int == '/' as i32 {
            *p = '\u{0}' as i32 as libc::c_char;
            let mut mkdirResult: libc::c_int = mkdir(
                _path.as_mut_ptr(),
                (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                    as __mode_t,
            );
            if mkdirResult != 0 as libc::c_int {
                if *__errno_location() != 17 as libc::c_int {
                    return -(1 as libc::c_int);
                }
            }
            *p = '/' as i32 as libc::c_char;
        }
        p = p.offset(1);
    }
    let mut mkdirResult_0: libc::c_int = mkdir(
        _path.as_mut_ptr(),
        (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int) as __mode_t,
    );
    if mkdirResult_0 != 0 as libc::c_int {
        if *__errno_location() != 17 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn normalize_filename(mut filename: *mut libc::c_char) {
    let mut current_pos: *mut libc::c_char = strchr(filename, '/' as i32);
    while !current_pos.is_null() {
        *current_pos = '-' as i32 as libc::c_char;
        current_pos = strchr(current_pos, '/' as i32);
    }
}
#[no_mangle]
pub unsafe extern "C" fn newBufferFile(mut bufferSize: libc::c_int) -> *mut BUFFER_FILE {
    let mut bufferFile: *mut BUFFER_FILE = malloc(
        ::std::mem::size_of::<BUFFER_FILE>() as libc::c_ulong,
    ) as *mut BUFFER_FILE;
    let ref mut fresh0 = (*bufferFile).buffer;
    *fresh0 = malloc(bufferSize as libc::c_ulong) as *mut libc::c_char;
    (*bufferFile).bufferPos = 0 as libc::c_int;
    (*bufferFile).bufferSize = bufferSize;
    return bufferFile;
}
#[no_mangle]
pub unsafe extern "C" fn freeBufferFile(mut bufferFile: *mut BUFFER_FILE) {
    free((*bufferFile).buffer as *mut libc::c_void);
    free(bufferFile as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn newWriteContext(
    mut outputDirectory: *mut libc::c_char,
    mut filingId: *mut libc::c_char,
    mut writeToFile: libc::c_int,
    mut bufferSize: libc::c_int,
    mut customWriteFunction: CustomWriteFunction,
    mut customLineFunction: CustomLineFunction,
) -> *mut WRITE_CONTEXT {
    let mut context: *mut WRITE_CONTEXT = malloc(
        ::std::mem::size_of::<WRITE_CONTEXT>() as libc::c_ulong,
    ) as *mut WRITE_CONTEXT;
    let ref mut fresh1 = (*context).outputDirectory;
    *fresh1 = outputDirectory;
    let ref mut fresh2 = (*context).filingId;
    *fresh2 = filingId;
    (*context).writeToFile = writeToFile;
    (*context).bufferSize = bufferSize;
    let ref mut fresh3 = (*context).filenames;
    *fresh3 = 0 as *mut *mut libc::c_char;
    let ref mut fresh4 = (*context).extensions;
    *fresh4 = 0 as *mut *mut libc::c_char;
    let ref mut fresh5 = (*context).bufferFiles;
    *fresh5 = 0 as *mut *mut BUFFER_FILE;
    let ref mut fresh6 = (*context).files;
    *fresh6 = 0 as *mut *mut FILE;
    (*context).nfiles = 0 as libc::c_int;
    let ref mut fresh7 = (*context).lastname;
    *fresh7 = 0 as *mut libc::c_char;
    let ref mut fresh8 = (*context).lastBufferFile;
    *fresh8 = 0 as *mut BUFFER_FILE;
    let ref mut fresh9 = (*context).lastfile;
    *fresh9 = 0 as *mut FILE;
    (*context).local = 0 as libc::c_int;
    let ref mut fresh10 = (*context).localBuffer;
    *fresh10 = 0 as *mut STRING;
    (*context).useCustomLine = customLineFunction.is_some() as libc::c_int;
    let ref mut fresh11 = (*context).customLineBuffer;
    *fresh11 = if (*context).useCustomLine != 0 {
        newString(DEFAULT_STRING_SIZE)
    } else {
        0 as *mut STRING
    };
    let ref mut fresh12 = (*context).customWriteFunction;
    *fresh12 = customWriteFunction;
    let ref mut fresh13 = (*context).customLineFunction;
    *fresh13 = customLineFunction;
    initializeCustomWriteContext(context);
    return context;
}
#[no_mangle]
pub unsafe extern "C" fn initializeLocalWriteContext(
    mut writeContext: *mut WRITE_CONTEXT,
    mut line: *mut STRING,
) {
    (*writeContext).local = 1 as libc::c_int;
    let ref mut fresh14 = (*writeContext).localBuffer;
    *fresh14 = line;
    (*writeContext).localBufferPosition = 0 as libc::c_int;
    *((*(*writeContext).localBuffer).str_0)
        .offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn initializeCustomWriteContext(
    mut writeContext: *mut WRITE_CONTEXT,
) {
    if (*writeContext).useCustomLine == 0 {
        return;
    }
    (*writeContext).customLineBufferPosition = 0 as libc::c_int;
    *((*(*writeContext).customLineBuffer).str_0)
        .offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn endLine(
    mut writeContext: *mut WRITE_CONTEXT,
    mut types: *mut libc::c_char,
) {
    if (*writeContext).useCustomLine == 0 {
        return;
    }
    ((*writeContext).customLineFunction)
        .expect(
            "non-null function pointer",
        )((*writeContext).lastname, (*(*writeContext).customLineBuffer).str_0, types);
    (*writeContext).customLineBufferPosition = 0 as libc::c_int;
    *((*(*writeContext).customLineBuffer).str_0)
        .offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn getFile(
    mut context: *mut WRITE_CONTEXT,
    mut filename: *mut libc::c_char,
    mut extension: *const libc::c_char,
) -> libc::c_int {
    if !((*context).lastname).is_null()
        && strcmp((*context).lastname, filename) == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if ((*context).filenames).is_null() {
        let ref mut fresh15 = (*context).filenames;
        *fresh15 = malloc(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            as *mut *mut libc::c_char;
        let ref mut fresh16 = (*context).extensions;
        *fresh16 = malloc(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            as *mut *mut libc::c_char;
        let ref mut fresh17 = (*context).bufferFiles;
        *fresh17 = malloc(::std::mem::size_of::<*mut BUFFER_FILE>() as libc::c_ulong)
            as *mut *mut BUFFER_FILE;
        if (*context).writeToFile != 0 {
            let ref mut fresh18 = (*context).files;
            *fresh18 = malloc(::std::mem::size_of::<*mut FILE>() as libc::c_ulong)
                as *mut *mut FILE;
        }
    } else {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < (*context).nfiles {
            if strcmp(*((*context).filenames).offset(i as isize), filename)
                == 0 as libc::c_int
            {
                let ref mut fresh19 = (*context).lastname;
                *fresh19 = *((*context).filenames).offset(i as isize);
                let ref mut fresh20 = (*context).lastBufferFile;
                *fresh20 = *((*context).bufferFiles).offset(i as isize);
                if (*context).writeToFile != 0 {
                    let ref mut fresh21 = (*context).lastfile;
                    *fresh21 = *((*context).files).offset(i as isize);
                }
                return 0 as libc::c_int;
            }
            i += 1;
        }
        let ref mut fresh22 = (*context).filenames;
        *fresh22 = realloc(
            (*context).filenames as *mut libc::c_void,
            (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                .wrapping_mul(((*context).nfiles + 1 as libc::c_int) as libc::c_ulong),
        ) as *mut *mut libc::c_char;
        let ref mut fresh23 = (*context).extensions;
        *fresh23 = realloc(
            (*context).extensions as *mut libc::c_void,
            (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                .wrapping_mul(((*context).nfiles + 1 as libc::c_int) as libc::c_ulong),
        ) as *mut *mut libc::c_char;
        let ref mut fresh24 = (*context).bufferFiles;
        *fresh24 = realloc(
            (*context).bufferFiles as *mut libc::c_void,
            (::std::mem::size_of::<*mut BUFFER_FILE>() as libc::c_ulong)
                .wrapping_mul(((*context).nfiles + 1 as libc::c_int) as libc::c_ulong),
        ) as *mut *mut BUFFER_FILE;
        if (*context).writeToFile != 0 {
            let ref mut fresh25 = (*context).files;
            *fresh25 = realloc(
                (*context).files as *mut libc::c_void,
                (::std::mem::size_of::<*mut FILE>() as libc::c_ulong)
                    .wrapping_mul(
                        ((*context).nfiles + 1 as libc::c_int) as libc::c_ulong,
                    ),
            ) as *mut *mut FILE;
        }
    }
    let ref mut fresh26 = *((*context).filenames).offset((*context).nfiles as isize);
    *fresh26 = malloc((strlen(filename)).wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    let ref mut fresh27 = *((*context).extensions).offset((*context).nfiles as isize);
    *fresh27 = malloc(
        (strlen(extension)).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    let ref mut fresh28 = *((*context).bufferFiles).offset((*context).nfiles as isize);
    *fresh28 = newBufferFile((*context).bufferSize);
    strcpy(*((*context).filenames).offset((*context).nfiles as isize), filename);
    strcpy(*((*context).extensions).offset((*context).nfiles as isize), extension);
    if (*context).writeToFile != 0 {
        let mut fullpath: *mut libc::c_char = malloc(
            (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul(
                    (strlen((*context).outputDirectory))
                        .wrapping_add(strlen(filename))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(strlen((*context).filingId))
                        .wrapping_add(strlen(extension))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ),
        ) as *mut libc::c_char;
        strcpy(fullpath, (*context).outputDirectory);
        strcat(fullpath, (*context).filingId);
        mkdir_p(fullpath);
        strcat(fullpath, b"/\0" as *const u8 as *const libc::c_char);
        let mut normalizedFilename: *mut libc::c_char = malloc(
            (strlen(filename)).wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        strcpy(normalizedFilename, filename);
        normalize_filename(normalizedFilename);
        strcat(fullpath, normalizedFilename);
        strcat(fullpath, extension);
        let ref mut fresh29 = *((*context).files).offset((*context).nfiles as isize);
        *fresh29 = fopen(fullpath, b"w\0" as *const u8 as *const libc::c_char);
        free(normalizedFilename as *mut libc::c_void);
        free(fullpath as *mut libc::c_void);
    }
    let ref mut fresh30 = (*context).lastname;
    *fresh30 = *((*context).filenames).offset((*context).nfiles as isize);
    let ref mut fresh31 = (*context).lastBufferFile;
    *fresh31 = *((*context).bufferFiles).offset((*context).nfiles as isize);
    if (*context).writeToFile != 0 {
        let ref mut fresh32 = (*context).lastfile;
        *fresh32 = *((*context).files).offset((*context).nfiles as isize);
    }
    let ref mut fresh33 = (*context).nfiles;
    *fresh33 += 1;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn bufferFlush(
    mut context: *mut WRITE_CONTEXT,
    mut filename: *mut libc::c_char,
    mut extension: *const libc::c_char,
    mut file: *mut FILE,
    mut bufferFile: *mut BUFFER_FILE,
) {
    if (*bufferFile).bufferPos == 0 as libc::c_int {
        return;
    }
    if ((*context).customWriteFunction).is_some() {
        ((*context).customWriteFunction)
            .expect(
                "non-null function pointer",
            )(
            filename,
            extension as *mut libc::c_char,
            (*bufferFile).buffer,
            (*bufferFile).bufferPos,
        );
    }
    if (*context).writeToFile != 0 {
        fwrite(
            (*bufferFile).buffer as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            (*bufferFile).bufferPos as libc::c_ulong,
            file,
        );
    }
    (*bufferFile).bufferPos = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn bufferWrite(
    mut context: *mut WRITE_CONTEXT,
    mut filename: *mut libc::c_char,
    mut extension: *const libc::c_char,
    mut file: *mut FILE,
    mut bufferFile: *mut BUFFER_FILE,
    mut string: *mut libc::c_char,
    mut nchars: libc::c_int,
) {
    let mut offset: libc::c_int = 0 as libc::c_int;
    while nchars > 0 as libc::c_int {
        let mut bytesToWrite: libc::c_int = nchars;
        let mut remaining: libc::c_int = (*bufferFile).bufferSize
            - (*bufferFile).bufferPos;
        if bytesToWrite > remaining {
            bytesToWrite = remaining;
        }
        memcpy(
            ((*bufferFile).buffer).offset((*bufferFile).bufferPos as isize)
                as *mut libc::c_void,
            string.offset(offset as isize) as *const libc::c_void,
            bytesToWrite as libc::c_ulong,
        );
        (*bufferFile).bufferPos += bytesToWrite;
        if (*bufferFile).bufferPos >= (*bufferFile).bufferSize {
            bufferFlush(context, filename, extension, file, bufferFile);
        }
        nchars -= bytesToWrite;
        offset += bytesToWrite;
    }
}
#[no_mangle]
pub unsafe extern "C" fn writeN(
    mut context: *mut WRITE_CONTEXT,
    mut filename: *mut libc::c_char,
    mut extension: *const libc::c_char,
    mut string: *mut libc::c_char,
    mut nchars: libc::c_int,
) {
    if (*context).local == 0 as libc::c_int {
        getFile(context, filename, extension);
        bufferWrite(
            context,
            filename,
            extension,
            (*context).lastfile,
            (*context).lastBufferFile,
            string,
            nchars,
        );
        if (*context).useCustomLine != 0 {
            let mut newPosition: libc::c_int = (*context).customLineBufferPosition
                + nchars;
            if (newPosition + 1 as libc::c_int) as libc::c_ulong
                > (*(*context).customLineBuffer).n
            {
                growStringTo(
                    (*context).customLineBuffer,
                    (newPosition + 1 as libc::c_int) as size_t,
                );
            }
            memcpy(
                ((*(*context).customLineBuffer).str_0)
                    .offset((*context).customLineBufferPosition as isize)
                    as *mut libc::c_void,
                string as *const libc::c_void,
                nchars as libc::c_ulong,
            );
            (*context).customLineBufferPosition = newPosition;
            *((*(*context).customLineBuffer).str_0)
                .offset(
                    (*context).customLineBufferPosition as isize,
                ) = 0 as libc::c_int as libc::c_char;
        }
    } else {
        let mut newPosition_0: libc::c_int = (*context).localBufferPosition + nchars;
        if (newPosition_0 + 1 as libc::c_int) as libc::c_ulong
            > (*(*context).localBuffer).n
        {
            growStringTo(
                (*context).localBuffer,
                (newPosition_0 + 1 as libc::c_int) as size_t,
            );
        }
        memcpy(
            ((*(*context).localBuffer).str_0)
                .offset((*context).localBufferPosition as isize) as *mut libc::c_void,
            string as *const libc::c_void,
            nchars as libc::c_ulong,
        );
        (*context).localBufferPosition = newPosition_0;
        *((*(*context).localBuffer).str_0)
            .offset(
                (*context).localBufferPosition as isize,
            ) = 0 as libc::c_int as libc::c_char;
    };
}
#[no_mangle]
pub unsafe extern "C" fn writeString(
    mut context: *mut WRITE_CONTEXT,
    mut filename: *mut libc::c_char,
    mut extension: *const libc::c_char,
    mut string: *mut libc::c_char,
) {
    writeN(context, filename, extension, string, strlen(string) as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn writeChar(
    mut context: *mut WRITE_CONTEXT,
    mut filename: *mut libc::c_char,
    mut extension: *const libc::c_char,
    mut c: libc::c_char,
) {
    if (*context).local == 0 as libc::c_int && (*context).useCustomLine == 0 {
        getFile(context, filename, extension);
        bufferWrite(
            context,
            filename,
            extension,
            (*context).lastfile,
            (*context).lastBufferFile,
            &mut c,
            1 as libc::c_int,
        );
    } else {
        let mut str: [libc::c_char; 1] = [c];
        writeN(context, filename, extension, str.as_mut_ptr(), 1 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn writeDouble(
    mut context: *mut WRITE_CONTEXT,
    mut filename: *mut libc::c_char,
    mut extension: *const libc::c_char,
    mut d: libc::c_double,
) {
    let mut str: [libc::c_char; 100] = [0; 100];
    sprintf(str.as_mut_ptr(), NUMBER_FORMAT, d);
    writeString(context, filename, extension, str.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn freeWriteContext(mut context: *mut WRITE_CONTEXT) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*context).nfiles {
        bufferFlush(
            context,
            *((*context).filenames).offset(i as isize),
            *((*context).extensions).offset(i as isize),
            if (*context).writeToFile != 0 {
                *((*context).files).offset(i as isize)
            } else {
                0 as *mut FILE
            },
            *((*context).bufferFiles).offset(i as isize),
        );
        free(*((*context).filenames).offset(i as isize) as *mut libc::c_void);
        free(*((*context).extensions).offset(i as isize) as *mut libc::c_void);
        freeBufferFile(*((*context).bufferFiles).offset(i as isize));
        if (*context).writeToFile != 0 {
            fclose(*((*context).files).offset(i as isize));
        }
        i += 1;
    }
    if !((*context).filenames).is_null() {
        free((*context).filenames as *mut libc::c_void);
    }
    if !((*context).files).is_null() {
        free((*context).files as *mut libc::c_void);
    }
    if !((*context).bufferFiles).is_null() {
        free((*context).bufferFiles as *mut libc::c_void);
    }
    if !((*context).extensions).is_null() {
        free((*context).extensions as *mut libc::c_void);
    }
    if !((*context).customLineBuffer).is_null() {
        freeString((*context).customLineBuffer);
    }
    free(context as *mut libc::c_void);
}
