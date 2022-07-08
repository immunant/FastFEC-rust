#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
use ::rust::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type real_pcre8_or_16;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn url_readBuffer(
        buffer: *mut libc::c_char,
        want: libc::c_int,
        file: *mut URL_FILE,
    ) -> size_t;
    fn url_fclose(file: *mut URL_FILE) -> libc::c_int;
    fn url_fopen_stdin() -> *mut URL_FILE;
    fn url_fopen(
        url: *const libc::c_char,
        operation: *const libc::c_char,
        override_0: *mut FILE,
    ) -> *mut URL_FILE;
    fn freePersistentMemoryContext(context: *mut PERSISTENT_MEMORY_CONTEXT);
    fn newPersistentMemoryContext() -> *mut PERSISTENT_MEMORY_CONTEXT;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    static mut pcre_free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
    fn pcre_compile(
        _: *const libc::c_char,
        _: libc::c_int,
        _: *mut *const libc::c_char,
        _: *mut libc::c_int,
        _: *const libc::c_uchar,
    ) -> *mut pcre;
    fn pcre_exec(
        _: *const pcre,
        _: *const pcre_extra,
        _: *const libc::c_char,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn newFecContext(
        persistentMemory: *mut PERSISTENT_MEMORY_CONTEXT,
        bufferRead: BufferRead,
        inputBufferSize: libc::c_int,
        customWriteFunction: CustomWriteFunction,
        outputBufferSize: libc::c_int,
        customLineFunction: CustomLineFunction,
        writeToFile: libc::c_int,
        file: *mut libc::c_void,
        filingId: *mut libc::c_char,
        outputDirectory: *mut libc::c_char,
        includeFilingId: libc::c_int,
        silent: libc::c_int,
        warn: libc::c_int,
    ) -> *mut FEC_CONTEXT;
    fn freeFecContext(context: *mut FEC_CONTEXT);
    fn parseFec(ctx: *mut FEC_CONTEXT) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
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
pub type CURL = ();
pub type pcre = real_pcre8_or_16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pcre_extra {
    pub flags: libc::c_ulong,
    pub study_data: *mut libc::c_void,
    pub match_limit: libc::c_ulong,
    pub callout_data: *mut libc::c_void,
    pub tables: *const libc::c_uchar,
    pub match_limit_recursion: libc::c_ulong,
    pub mark: *mut *mut libc::c_uchar,
    pub executable_jit: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct string_type {
    pub str_0: *mut libc::c_char,
    pub n: size_t,
}
pub type STRING = string_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct persistent_memory_context {
    pub rawLine: *mut STRING,
    pub line: *mut STRING,
    pub bufferLine: *mut STRING,
    pub headerVersions: *mut *mut pcre,
    pub headerFormTypes: *mut *mut pcre,
    pub typeVersions: *mut *mut pcre,
    pub typeFormTypes: *mut *mut pcre,
    pub typeHeaders: *mut *mut pcre,
}
pub type PERSISTENT_MEMORY_CONTEXT = persistent_memory_context;
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
pub type fcurl_type_e = libc::c_uint;
pub const CFTYPE_CURL: fcurl_type_e = 2;
pub const CFTYPE_FILE: fcurl_type_e = 1;
pub const CFTYPE_NONE: fcurl_type_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fcurl_data {
    pub type_0: fcurl_type_e,
    pub handle: C2RustUnnamed,
    pub buffer: *mut libc::c_char,
    pub buffer_len: size_t,
    pub buffer_pos: size_t,
    pub still_running: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub curl: *mut libc::c_void,
    pub file: *mut FILE,
}
pub type URL_FILE = fcurl_data;
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
pub struct fec_context {
    pub buffer: *mut BUFFER,
    pub file: *mut libc::c_void,
    pub writeContext: *mut WRITE_CONTEXT,
    pub filingId: *mut libc::c_char,
    pub version: *mut libc::c_char,
    pub versionLength: libc::c_int,
    pub useAscii28: libc::c_int,
    pub summary: libc::c_int,
    pub f99Text: *mut libc::c_char,
    pub persistentMemory: *mut PERSISTENT_MEMORY_CONTEXT,
    pub currentLineHasAscii28: libc::c_int,
    pub currentLineLength: libc::c_int,
    pub includeFilingId: libc::c_int,
    pub silent: libc::c_int,
    pub warn: libc::c_int,
    pub formType: *mut libc::c_char,
    pub numFields: libc::c_int,
    pub headers: *mut libc::c_char,
    pub types: *mut libc::c_char,
    pub f99TextStart: *mut pcre,
    pub f99TextEnd: *mut pcre,
}
pub type FEC_CONTEXT = fec_context;
#[no_mangle]
pub static mut FLAG_FILING_ID: *const libc::c_char = b"--include-filing-id\0"
    as *const u8 as *const libc::c_char;
#[no_mangle]
pub static mut FLAG_FILING_ID_SHORT: libc::c_char = 'i' as i32 as libc::c_char;
#[no_mangle]
pub static mut FLAG_SILENT: *const libc::c_char = b"--silent\0" as *const u8
    as *const libc::c_char;
#[no_mangle]
pub static mut FLAG_SILENT_SHORT: libc::c_char = 's' as i32 as libc::c_char;
#[no_mangle]
pub static mut FLAG_WARN: *const libc::c_char = b"--warn\0" as *const u8
    as *const libc::c_char;
#[no_mangle]
pub static mut FLAG_WARN_SHORT: libc::c_char = 'w' as i32 as libc::c_char;
#[no_mangle]
pub static mut FLAG_DISABLE_STDIN: *const libc::c_char = b"--no-stdin\0" as *const u8
    as *const libc::c_char;
#[no_mangle]
pub static mut FLAG_DISABLE_STDIN_SHORT: libc::c_char = 'x' as i32 as libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn printUsage(mut argv: *mut *mut libc::c_char) {
    fprintf(
        stderr,
        b"\nUsage:\n    %s [flags] <id, file, or url> [output directory=output] [override id]\nor: [some command] | %s [flags] <id> [output directory=output]\n\0"
            as *const u8 as *const libc::c_char,
        *argv.offset(0 as libc::c_int as isize),
        *argv.offset(0 as libc::c_int as isize),
    );
    fprintf(stderr, b"\nOptional flags:\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        stderr,
        b"  %s, -%c: include a filing_id column at the beginning of\n                        every output CSV\n\0"
            as *const u8 as *const libc::c_char,
        FLAG_FILING_ID,
        FLAG_FILING_ID_SHORT as libc::c_int,
    );
    fprintf(
        stderr,
        b"  %s, -%c        : suppress all stdout messages\n\n\0" as *const u8
            as *const libc::c_char,
        FLAG_SILENT,
        FLAG_SILENT_SHORT as libc::c_int,
    );
    fprintf(
        stderr,
        b"  %s, -%c        : show warning messages\n\n\0" as *const u8
            as *const libc::c_char,
        FLAG_WARN,
        FLAG_WARN_SHORT as libc::c_int,
    );
    fprintf(
        stderr,
        b"  %s, -%c        : disable piped input\n\n\0" as *const u8
            as *const libc::c_char,
        FLAG_DISABLE_STDIN,
        FLAG_DISABLE_STDIN_SHORT as libc::c_int,
    );
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut piped: libc::c_int = (isatty(fileno(stdin)) == 0) as libc::c_int;
    let mut flagOffset: libc::c_int = 0 as libc::c_int;
    let mut handle: *mut URL_FILE = 0 as *mut URL_FILE;
    let mut url: *const libc::c_char = 0 as *const libc::c_char;
    if argc < 2 as libc::c_int {
        printUsage(argv);
        exit(1 as libc::c_int);
    }
    let mut error: *const libc::c_char = 0 as *const libc::c_char;
    let mut errorOffset: libc::c_int = 0;
    let mut filingIdOnly: *mut pcre = pcre_compile(
        b"^\\s*([0-9]+)\\s*$\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        &mut error,
        &mut errorOffset,
        0 as *const libc::c_uchar,
    );
    if filingIdOnly.is_null() {
        fprintf(
            stderr,
            b"PCRE filing ID compilation failed at offset %d: %s\n\0" as *const u8
                as *const libc::c_char,
            errorOffset,
            error,
        );
        exit(1 as libc::c_int);
    }
    let mut extractNumber: *mut pcre = pcre_compile(
        b"^.*?([0-9]+)(\\.[^\\.]+)?\\s*$\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        &mut error,
        &mut errorOffset,
        0 as *const libc::c_uchar,
    );
    if extractNumber.is_null() {
        fprintf(
            stderr,
            b"PCRE number extraction compilation failed at offset %d: %s\n\0"
                as *const u8 as *const libc::c_char,
            errorOffset,
            error,
        );
        exit(1 as libc::c_int);
    }
    let mut includeFilingId: libc::c_int = 0 as libc::c_int;
    let mut silent: libc::c_int = 0 as libc::c_int;
    let mut warn: libc::c_int = 0 as libc::c_int;
    while *(*argv.offset((1 as libc::c_int + flagOffset) as isize))
        .offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
    {
        if strcmp(*argv.offset((1 as libc::c_int + flagOffset) as isize), FLAG_FILING_ID)
            == 0 as libc::c_int
        {
            includeFilingId = 1 as libc::c_int;
            flagOffset += 1;
        } else if strcmp(
                *argv.offset((1 as libc::c_int + flagOffset) as isize),
                FLAG_SILENT,
            ) == 0 as libc::c_int
            {
            silent = 1 as libc::c_int;
            flagOffset += 1;
        } else if strcmp(
                *argv.offset((1 as libc::c_int + flagOffset) as isize),
                FLAG_WARN,
            ) == 0 as libc::c_int
            {
            warn = 1 as libc::c_int;
            flagOffset += 1;
        } else if strcmp(
                *argv.offset((1 as libc::c_int + flagOffset) as isize),
                FLAG_DISABLE_STDIN,
            ) == 0 as libc::c_int
            {
            piped = 0 as libc::c_int;
            flagOffset += 1;
        } else {
            let mut matched: libc::c_int = 0 as libc::c_int;
            let mut i: libc::c_int = 1 as libc::c_int;
            while (i as libc::c_ulong)
                < strlen(*argv.offset((1 as libc::c_int + flagOffset) as isize))
            {
                if *(*argv.offset((1 as libc::c_int + flagOffset) as isize))
                    .offset(i as isize) as libc::c_int
                    == FLAG_FILING_ID_SHORT as libc::c_int
                {
                    includeFilingId = 1 as libc::c_int;
                    matched = 1 as libc::c_int;
                } else if *(*argv.offset((1 as libc::c_int + flagOffset) as isize))
                        .offset(i as isize) as libc::c_int
                        == FLAG_SILENT_SHORT as libc::c_int
                    {
                    silent = 1 as libc::c_int;
                    matched = 1 as libc::c_int;
                } else if *(*argv.offset((1 as libc::c_int + flagOffset) as isize))
                        .offset(i as isize) as libc::c_int
                        == FLAG_WARN_SHORT as libc::c_int
                    {
                    warn = 1 as libc::c_int;
                    matched = 1 as libc::c_int;
                } else if *(*argv.offset((1 as libc::c_int + flagOffset) as isize))
                        .offset(i as isize) as libc::c_int
                        == FLAG_DISABLE_STDIN_SHORT as libc::c_int
                    {
                    piped = 0 as libc::c_int;
                    matched = 1 as libc::c_int;
                } else {
                    printUsage(argv);
                    exit(1 as libc::c_int);
                }
                i += 1;
            }
            if matched != 0 {
                flagOffset += 1;
            } else {
                printUsage(argv);
                exit(1 as libc::c_int);
            }
        }
    }
    let mut docqueryUrl: *const libc::c_char = b"https://docquery.fec.gov/dcdev/posted/\0"
        as *const u8 as *const libc::c_char;
    let mut docqueryUrlAlt: *const libc::c_char = b"https://docquery.fec.gov/paper/posted/\0"
        as *const u8 as *const libc::c_char;
    url = if piped != 0 {
        0 as *mut libc::c_char
    } else {
        *argv.offset((1 as libc::c_int + flagOffset) as isize)
    };
    let mut fecUrl: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fecBackupUrl: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fecId: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut outputDirectory: *mut libc::c_char = malloc(
        8 as libc::c_int as libc::c_ulong,
    ) as *mut libc::c_char;
    strcpy(outputDirectory, b"output/\0" as *const u8 as *const libc::c_char);
    let mut fecExtension: *mut libc::c_char = b".fec\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    if argc > 2 as libc::c_int + flagOffset {
        outputDirectory = realloc(
            outputDirectory as *mut libc::c_void,
            (strlen(*argv.offset((2 as libc::c_int + flagOffset) as isize)))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        strcpy(outputDirectory, *argv.offset((2 as libc::c_int + flagOffset) as isize));
        if *outputDirectory
            .offset(
                (strlen(outputDirectory)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as isize,
            ) as libc::c_int != '/' as i32
        {
            outputDirectory = realloc(
                outputDirectory as *mut libc::c_void,
                (strlen(outputDirectory)).wrapping_add(2 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            strcat(outputDirectory, b"/\0" as *const u8 as *const libc::c_char);
        }
    }
    if piped != 0 {
        if argc > 1 as libc::c_int + flagOffset {
            fecId = malloc(
                (strlen(*argv.offset((1 as libc::c_int + flagOffset) as isize)))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            strcpy(fecId, *argv.offset((1 as libc::c_int + flagOffset) as isize));
        } else {
            printUsage(argv);
            exit(1 as libc::c_int);
        }
    } else if argc > 3 as libc::c_int + flagOffset {
        fecId = malloc(
            (strlen(*argv.offset((3 as libc::c_int + flagOffset) as isize)))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        strcpy(fecId, *argv.offset((3 as libc::c_int + flagOffset) as isize));
    }
    let mut matches: [libc::c_int; 6] = [0; 6];
    if piped == 0
        && pcre_exec(
            filingIdOnly,
            0 as *const pcre_extra,
            url,
            strlen(url) as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            matches.as_mut_ptr(),
            3 as libc::c_int,
        ) >= 0 as libc::c_int
    {
        let mut start: libc::c_int = matches[0 as libc::c_int as usize];
        let mut end: libc::c_int = matches[1 as libc::c_int as usize];
        fecId = malloc((end - start + 1 as libc::c_int) as libc::c_ulong)
            as *mut libc::c_char;
        strncpy(fecId, url.offset(start as isize), (end - start) as libc::c_ulong);
        *fecId.offset((end - start) as isize) = '\u{0}' as i32 as libc::c_char;
        fecUrl = malloc(
            (strlen(docqueryUrl))
                .wrapping_add(strlen(fecId))
                .wrapping_add(strlen(fecExtension))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        strcpy(fecUrl, docqueryUrl);
        strcat(fecUrl, fecId);
        strcat(fecUrl, fecExtension);
        *fecUrl.offset(strlen(fecUrl) as isize) = '\u{0}' as i32 as libc::c_char;
        fecBackupUrl = malloc(
            (strlen(docqueryUrlAlt))
                .wrapping_add(strlen(fecId))
                .wrapping_add(strlen(fecExtension))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        strcpy(fecBackupUrl, docqueryUrlAlt);
        strcat(fecBackupUrl, fecId);
        strcat(fecBackupUrl, fecExtension);
    } else if piped == 0 {
        fecUrl = malloc((strlen(url)).wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        strcpy(fecUrl, url);
        *fecUrl.offset(strlen(fecUrl) as isize) = '\u{0}' as i32 as libc::c_char;
        if fecId.is_null()
            && pcre_exec(
                extractNumber,
                0 as *const pcre_extra,
                fecUrl,
                strlen(fecUrl) as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                matches.as_mut_ptr(),
                6 as libc::c_int,
            ) >= 0 as libc::c_int
        {
            let mut start_0: libc::c_int = matches[2 as libc::c_int as usize];
            let mut end_0: libc::c_int = matches[3 as libc::c_int as usize];
            fecId = malloc((end_0 - start_0 + 1 as libc::c_int) as libc::c_ulong)
                as *mut libc::c_char;
            strncpy(
                fecId,
                url.offset(start_0 as isize),
                (end_0 - start_0) as libc::c_ulong,
            );
            *fecId.offset((end_0 - start_0) as isize) = '\u{0}' as i32 as libc::c_char;
        }
    }
    if fecId.is_null() {
        fprintf(
            stderr,
            b"Please specify a filing ID manually\n\0" as *const u8
                as *const libc::c_char,
        );
        printUsage(argv);
        free(outputDirectory as *mut libc::c_void);
        if !fecUrl.is_null() {
            free(fecUrl as *mut libc::c_void);
        }
        if !fecBackupUrl.is_null() {
            free(fecBackupUrl as *mut libc::c_void);
        }
        pcre_free.expect("non-null function pointer")(filingIdOnly as *mut libc::c_void);
        pcre_free
            .expect("non-null function pointer")(extractNumber as *mut libc::c_void);
        exit(1 as libc::c_int);
    }
    if silent == 0 {
        printf(
            b"About to parse filing ID %s\n\0" as *const u8 as *const libc::c_char,
            fecId,
        );
    }
    if piped != 0 {
        if silent == 0 {
            printf(b"Using stdin\n\0" as *const u8 as *const libc::c_char);
        }
    } else if silent == 0 {
        printf(b"Trying URL: %s\n\0" as *const u8 as *const libc::c_char, fecUrl);
    }
    handle = if piped != 0 {
        url_fopen_stdin()
    } else {
        url_fopen(fecUrl, b"r\0" as *const u8 as *const libc::c_char, 0 as *mut FILE)
    };
    if handle.is_null() {
        if !fecBackupUrl.is_null() {
            if silent == 0 {
                printf(
                    b"Couldn't open URL %s\n\0" as *const u8 as *const libc::c_char,
                    fecUrl,
                );
                printf(
                    b"Trying backup URL: %s\n\0" as *const u8 as *const libc::c_char,
                    fecBackupUrl,
                );
            }
            handle = url_fopen(
                fecBackupUrl,
                b"r\0" as *const u8 as *const libc::c_char,
                0 as *mut FILE,
            );
            if handle.is_null() {
                fprintf(
                    stderr,
                    b"Couldn't open URL: %s\n\0" as *const u8 as *const libc::c_char,
                    fecBackupUrl,
                );
                return 2 as libc::c_int;
            }
        } else {
            fprintf(
                stderr,
                b"Couldn't open URL %s\n\0" as *const u8 as *const libc::c_char,
                fecUrl,
            );
            return 2 as libc::c_int;
        }
    }
    let mut persistentMemory: *mut PERSISTENT_MEMORY_CONTEXT = newPersistentMemoryContext();
    let mut fec: *mut FEC_CONTEXT = newFecContext(
        persistentMemory,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_char,
                    libc::c_int,
                    *mut URL_FILE,
                ) -> size_t,
            >,
            BufferRead,
        >(
            Some(
                url_readBuffer
                    as unsafe extern "C" fn(
                        *mut libc::c_char,
                        libc::c_int,
                        *mut URL_FILE,
                    ) -> size_t,
            ),
        ),
        65536 as libc::c_int,
        None,
        65536 as libc::c_int,
        None,
        1 as libc::c_int,
        handle as *mut libc::c_void,
        fecId,
        outputDirectory,
        includeFilingId,
        silent,
        warn,
    );
    let mut fecParseResult: libc::c_int = parseFec(fec);
    freeFecContext(fec);
    freePersistentMemoryContext(persistentMemory);
    pcre_free.expect("non-null function pointer")(filingIdOnly as *mut libc::c_void);
    pcre_free.expect("non-null function pointer")(extractNumber as *mut libc::c_void);
    free(outputDirectory as *mut libc::c_void);
    if !fecUrl.is_null() {
        free(fecUrl as *mut libc::c_void);
    }
    if !fecBackupUrl.is_null() {
        free(fecBackupUrl as *mut libc::c_void);
    }
    if !fecId.is_null() {
        free(fecId as *mut libc::c_void);
    }
    url_fclose(handle);
    if fecParseResult == 0 {
        fprintf(stderr, b"Parsing FEC failed\n\0" as *const u8 as *const libc::c_char);
        return 3 as libc::c_int;
    }
    if silent == 0 {
        printf(b"Done; parsing successful!\n\0" as *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
