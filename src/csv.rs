use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn writeN(
        context: *mut WRITE_CONTEXT,
        filename: *mut libc::c_char,
        extension: *const libc::c_char,
        string: *mut libc::c_char,
        nchars: libc::c_int,
    );
    fn writeString(
        context: *mut WRITE_CONTEXT,
        filename: *mut libc::c_char,
        extension: *const libc::c_char,
        string: *mut libc::c_char,
    );
    fn writeChar(
        context: *mut WRITE_CONTEXT,
        filename: *mut libc::c_char,
        extension: *const libc::c_char,
        c: libc::c_char,
    );
}
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct field_info {
    pub num_commas: libc::c_int,
    pub num_quotes: libc::c_int,
}
pub type FIELD_INFO = field_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parse_context {
    pub line: *mut STRING,
    pub fieldInfo: *mut FIELD_INFO,
    pub position: libc::c_int,
    pub start: libc::c_int,
    pub end: libc::c_int,
    pub columnIndex: libc::c_int,
}
pub type PARSE_CONTEXT = parse_context;
#[no_mangle]
pub unsafe extern "C" fn processFieldChar(
    mut c: libc::c_char,
    mut info: *mut FIELD_INFO,
) {
    if !info.is_null() {
        if c as libc::c_int == '"' as i32 {
            let ref mut fresh0 = (*info).num_quotes;
            *fresh0 += 1;
        } else if c as libc::c_int == ',' as i32 {
            let ref mut fresh1 = (*info).num_commas;
            *fresh1 += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn writeDelimeter(
    mut context: *mut WRITE_CONTEXT,
    mut filename: *mut libc::c_char,
    mut extension: *const libc::c_char,
) {
    writeChar(context, filename, extension, ',' as i32 as libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn writeNewline(
    mut context: *mut WRITE_CONTEXT,
    mut filename: *mut libc::c_char,
    mut extension: *const libc::c_char,
) {
    writeChar(context, filename, extension, '\n' as i32 as libc::c_char);
}
#[inline]
unsafe extern "C" fn endOfField(mut c: libc::c_char) -> libc::c_int {
    return (c as libc::c_int == ',' as i32 || c as libc::c_int == '\n' as i32
        || c as libc::c_int == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn stripQuotes(mut parseContext: *mut PARSE_CONTEXT) {
    if (*parseContext).start != (*parseContext).end
        && *((*(*parseContext).line).str_0).offset((*parseContext).start as isize)
            as libc::c_int == '"' as i32
        && *((*(*parseContext).line).str_0)
            .offset(((*parseContext).end - 1 as libc::c_int) as isize) as libc::c_int
            == '"' as i32
    {
        let ref mut fresh2 = (*parseContext).start;
        *fresh2 += 1;
        let ref mut fresh3 = (*parseContext).end;
        *fresh3 -= 1;
        if !((*parseContext).fieldInfo).is_null() {
            (*(*parseContext).fieldInfo).num_quotes -= 2 as libc::c_int;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn readAscii28Field(mut parseContext: *mut PARSE_CONTEXT) {
    (*parseContext).start = (*parseContext).position;
    let mut c: libc::c_char = *((*(*parseContext).line).str_0)
        .offset((*parseContext).position as isize);
    processFieldChar(c, (*parseContext).fieldInfo);
    while c as libc::c_int != 0 as libc::c_int && c as libc::c_int != 28 as libc::c_int
        && c as libc::c_int != '\n' as i32
    {
        (*parseContext).position += 1 as libc::c_int;
        c = *((*(*parseContext).line).str_0).offset((*parseContext).position as isize);
        processFieldChar(c, (*parseContext).fieldInfo);
    }
    (*parseContext).end = (*parseContext).position;
    stripQuotes(parseContext);
}
#[no_mangle]
pub unsafe extern "C" fn readCsvSubfield(mut parseContext: *mut PARSE_CONTEXT) {
    let mut c: libc::c_char = *((*(*parseContext).line).str_0)
        .offset((*parseContext).position as isize);
    if endOfField(c) != 0 {
        (*parseContext).start = (*parseContext).position;
        (*parseContext).end = (*parseContext).position;
        return;
    }
    let mut escaped: libc::c_int = (c as libc::c_int == '"' as i32) as libc::c_int;
    let mut offset: libc::c_int = 0 as libc::c_int;
    if escaped != 0 {
        let ref mut fresh4 = (*parseContext).position;
        *fresh4 += 1;
    }
    (*parseContext).start = (*parseContext).position;
    loop {
        if offset != 0 as libc::c_int {
            *((*(*parseContext).line).str_0)
                .offset(
                    ((*parseContext).position - offset) as isize,
                ) = *((*(*parseContext).line).str_0)
                .offset((*parseContext).position as isize);
        }
        c = *((*(*parseContext).line).str_0).offset((*parseContext).position as isize);
        if c as libc::c_int == 0 as libc::c_int {
            (*parseContext).end = (*parseContext).position - offset;
            return;
        }
        if escaped == 0
            && (c as libc::c_int == ',' as i32 || c as libc::c_int == '\n' as i32)
        {
            (*parseContext).end = (*parseContext).position - offset;
            return;
        }
        processFieldChar(c, (*parseContext).fieldInfo);
        if escaped != 0 && c as libc::c_int == '"' as i32 {
            if *((*(*parseContext).line).str_0)
                .offset(((*parseContext).position + 1 as libc::c_int) as isize)
                as libc::c_int == '"' as i32
            {
                let ref mut fresh5 = (*parseContext).position;
                *fresh5 += 1;
                offset += 1;
            } else {
                (*parseContext).end = (*parseContext).position - offset;
                let ref mut fresh6 = (*parseContext).position;
                *fresh6 += 1;
                let ref mut fresh7 = (*(*parseContext).fieldInfo).num_quotes;
                *fresh7 -= 1;
                return;
            }
        }
        let ref mut fresh8 = (*parseContext).position;
        *fresh8 += 1;
    };
}
#[no_mangle]
pub unsafe extern "C" fn readCsvField(mut parseContext: *mut PARSE_CONTEXT) {
    readCsvSubfield(parseContext);
    stripQuotes(parseContext);
}
#[no_mangle]
pub unsafe extern "C" fn advanceField(mut context: *mut PARSE_CONTEXT) {
    let ref mut fresh9 = (*context).columnIndex;
    *fresh9 += 1;
    let ref mut fresh10 = (*context).position;
    *fresh10 += 1;
}
#[no_mangle]
pub unsafe extern "C" fn writeField(
    mut context: *mut WRITE_CONTEXT,
    mut filename: *mut libc::c_char,
    mut extension: *const libc::c_char,
    mut line: *mut STRING,
    mut start: libc::c_int,
    mut end: libc::c_int,
    mut info: *mut FIELD_INFO,
) {
    let mut escaped: libc::c_int = ((*info).num_commas > 0 as libc::c_int
        || (*info).num_quotes > 0 as libc::c_int) as libc::c_int;
    let mut copyDirectly: libc::c_int = !((*info).num_quotes > 0 as libc::c_int)
        as libc::c_int;
    if escaped != 0 {
        writeChar(context, filename, extension, '"' as i32 as libc::c_char);
    }
    if copyDirectly != 0 {
        writeN(
            context,
            filename,
            extension,
            ((*line).str_0).offset(start as isize),
            end - start,
        );
    } else {
        let mut i: libc::c_int = start;
        while i < end {
            let mut c: libc::c_char = *((*line).str_0).offset(i as isize);
            if c as libc::c_int == '"' as i32 {
                writeString(
                    context,
                    filename,
                    extension,
                    b"\"\"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            } else {
                writeChar(context, filename, extension, c);
            }
            i += 1;
        }
    }
    if escaped != 0 {
        writeChar(context, filename, extension, '"' as i32 as libc::c_char);
    }
}
#[no_mangle]
pub unsafe extern "C" fn isWhitespaceChar(mut c: libc::c_char) -> libc::c_int {
    return (c as libc::c_int == ' ' as i32 || c as libc::c_int == '\t' as i32
        || c as libc::c_int == '\n' as i32) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn isWhitespace(
    mut context: *mut PARSE_CONTEXT,
    mut position: libc::c_int,
) -> libc::c_int {
    return isWhitespaceChar(*((*(*context).line).str_0).offset(position as isize));
}
#[no_mangle]
pub unsafe extern "C" fn stripWhitespace(mut context: *mut PARSE_CONTEXT) {
    while isWhitespace(context, (*context).start) != 0
        && (*context).start < (*context).end
    {
        let ref mut fresh11 = (*context).start;
        *fresh11 += 1;
    }
    while isWhitespace(context, (*context).end - 1 as libc::c_int) != 0
        && (*context).end > (*context).start
    {
        let ref mut fresh12 = (*context).end;
        *fresh12 -= 1;
    }
}
