/* automatically generated by rust-bindgen */
#![allow(unused)]
#![allow(non_camel_case_types)]

pub const _STDINT_H: u32 = 1;
pub const _FEATURES_H: u32 = 1;
pub const _DEFAULT_SOURCE: u32 = 1;
pub const __USE_ISOC11: u32 = 1;
pub const __USE_ISOC99: u32 = 1;
pub const __USE_ISOC95: u32 = 1;
pub const __USE_POSIX_IMPLICITLY: u32 = 1;
pub const _POSIX_SOURCE: u32 = 1;
pub const _POSIX_C_SOURCE: u32 = 200809;
pub const __USE_POSIX: u32 = 1;
pub const __USE_POSIX2: u32 = 1;
pub const __USE_POSIX199309: u32 = 1;
pub const __USE_POSIX199506: u32 = 1;
pub const __USE_XOPEN2K: u32 = 1;
pub const __USE_XOPEN2K8: u32 = 1;
pub const _ATFILE_SOURCE: u32 = 1;
pub const __USE_MISC: u32 = 1;
pub const __USE_ATFILE: u32 = 1;
pub const __USE_FORTIFY_LEVEL: u32 = 0;
pub const _STDC_PREDEF_H: u32 = 1;
pub const __STDC_IEC_559__: u32 = 1;
pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
pub const __STDC_ISO_10646__: u32 = 201505;
pub const __STDC_NO_THREADS__: u32 = 1;
pub const __GNU_LIBRARY__: u32 = 6;
pub const __GLIBC__: u32 = 2;
pub const __GLIBC_MINOR__: u32 = 23;
pub const _SYS_CDEFS_H: u32 = 1;
pub const __WORDSIZE: u32 = 64;
pub const __WORDSIZE_TIME64_COMPAT32: u32 = 1;
pub const __SYSCALL_WORDSIZE: u32 = 64;
pub const _BITS_WCHAR_H: u32 = 1;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i64 = -9223372036854775808;
pub const INT_FAST32_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u64 = 9223372036854775807;
pub const INT_FAST32_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: i32 = -1;
pub const UINT_FAST32_MAX: i32 = -1;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const UINTPTR_MAX: i32 = -1;
pub const PTRDIFF_MIN: i64 = -9223372036854775808;
pub const PTRDIFF_MAX: u64 = 9223372036854775807;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const SIZE_MAX: i32 = -1;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 4294967295;
pub const TSDB_DATA_TYPE_NULL: u32 = 0;
pub const TSDB_DATA_TYPE_BOOL: u32 = 1;
pub const TSDB_DATA_TYPE_TINYINT: u32 = 2;
pub const TSDB_DATA_TYPE_SMALLINT: u32 = 3;
pub const TSDB_DATA_TYPE_INT: u32 = 4;
pub const TSDB_DATA_TYPE_BIGINT: u32 = 5;
pub const TSDB_DATA_TYPE_FLOAT: u32 = 6;
pub const TSDB_DATA_TYPE_DOUBLE: u32 = 7;
pub const TSDB_DATA_TYPE_BINARY: u32 = 8;
pub const TSDB_DATA_TYPE_TIMESTAMP: u32 = 9;
pub const TSDB_DATA_TYPE_NCHAR: u32 = 10;
pub type int_least8_t = ::std::os::raw::c_schar;
pub type int_least16_t = ::std::os::raw::c_short;
pub type int_least32_t = ::std::os::raw::c_int;
pub type int_least64_t = ::std::os::raw::c_long;
pub type uint_least8_t = ::std::os::raw::c_uchar;
pub type uint_least16_t = ::std::os::raw::c_ushort;
pub type uint_least32_t = ::std::os::raw::c_uint;
pub type uint_least64_t = ::std::os::raw::c_ulong;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_long;
pub type int_fast32_t = ::std::os::raw::c_long;
pub type int_fast64_t = ::std::os::raw::c_long;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_ulong;
pub type uint_fast32_t = ::std::os::raw::c_ulong;
pub type uint_fast64_t = ::std::os::raw::c_ulong;
pub type intmax_t = ::std::os::raw::c_long;
pub type uintmax_t = ::std::os::raw::c_ulong;
pub const TSDB_OPTION_TSDB_OPTION_LOCALE: TSDB_OPTION = 0;
pub const TSDB_OPTION_TSDB_OPTION_CHARSET: TSDB_OPTION = 1;
pub const TSDB_OPTION_TSDB_OPTION_TIMEZONE: TSDB_OPTION = 2;
pub const TSDB_OPTION_TSDB_OPTION_CONFIGDIR: TSDB_OPTION = 3;
pub const TSDB_OPTION_TSDB_OPTION_SHELL_ACTIVITY_TIMER: TSDB_OPTION = 4;
pub const TSDB_OPTION_TSDB_MAX_OPTIONS: TSDB_OPTION = 5;
pub type TSDB_OPTION = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct taosField {
    pub name: [::std::os::raw::c_char; 64usize],
    pub bytes: ::std::os::raw::c_short,
    pub type_: ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_taosField() {
    assert_eq!(
        ::std::mem::size_of::<taosField>(),
        68usize,
        concat!("Size of: ", stringify!(taosField))
    );
    assert_eq!(
        ::std::mem::align_of::<taosField>(),
        2usize,
        concat!("Alignment of ", stringify!(taosField))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<taosField>())).name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(taosField),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<taosField>())).bytes as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(taosField),
            "::",
            stringify!(bytes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<taosField>())).type_ as *const _ as usize },
        66usize,
        concat!(
            "Offset of field: ",
            stringify!(taosField),
            "::",
            stringify!(type_)
        )
    );
}
pub type TAOS_FIELD = taosField;
extern "C" {
    pub fn taos_init();
}
extern "C" {
    pub fn taos_options(
        option: TSDB_OPTION,
        arg: *const ::std::os::raw::c_void,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn taos_connect(
        ip: *mut ::std::os::raw::c_char,
        user: *mut ::std::os::raw::c_char,
        pass: *mut ::std::os::raw::c_char,
        db: *mut ::std::os::raw::c_char,
        port: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn taos_close(taos: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn taos_query(
        taos: *mut ::std::os::raw::c_void,
        sqlstr: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn taos_use_result(taos: *mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn taos_fetch_row(res: *mut ::std::os::raw::c_void) -> *mut *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn taos_result_precision(res: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn taos_free_result(res: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn taos_field_count(taos: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn taos_num_fields(res: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn taos_affected_rows(taos: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn taos_fetch_fields(res: *mut ::std::os::raw::c_void) -> *mut TAOS_FIELD;
}
extern "C" {
    pub fn taos_select_db(
        taos: *mut ::std::os::raw::c_void,
        db: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn taos_print_row(
        str: *mut ::std::os::raw::c_char,
        row: *mut *mut ::std::os::raw::c_void,
        fields: *mut TAOS_FIELD,
        num_fields: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn taos_stop_query(res: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn taos_fetch_block(
        res: *mut ::std::os::raw::c_void,
        rows: *mut *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn taos_validate_sql(
        taos: *mut ::std::os::raw::c_void,
        sql: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn taos_get_server_info(taos: *mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn taos_get_client_info() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn taos_errstr(taos: *mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn taos_errno(taos: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn taos_query_a(
        taos: *mut ::std::os::raw::c_void,
        sqlstr: *mut ::std::os::raw::c_char,
        fp: ::std::option::Option<
            unsafe extern "C" fn(
                param: *mut ::std::os::raw::c_void,
                arg1: *mut ::std::os::raw::c_void,
                code: ::std::os::raw::c_int,
            ),
        >,
        param: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn taos_fetch_rows_a(
        res: *mut ::std::os::raw::c_void,
        fp: ::std::option::Option<
            unsafe extern "C" fn(
                param: *mut ::std::os::raw::c_void,
                arg1: *mut ::std::os::raw::c_void,
                numOfRows: ::std::os::raw::c_int,
            ),
        >,
        param: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn taos_fetch_row_a(
        res: *mut ::std::os::raw::c_void,
        fp: ::std::option::Option<
            unsafe extern "C" fn(
                param: *mut ::std::os::raw::c_void,
                arg1: *mut ::std::os::raw::c_void,
                row: *mut *mut ::std::os::raw::c_void,
            ),
        >,
        param: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn taos_subscribe(
        host: *mut ::std::os::raw::c_char,
        user: *mut ::std::os::raw::c_char,
        pass: *mut ::std::os::raw::c_char,
        db: *mut ::std::os::raw::c_char,
        table: *mut ::std::os::raw::c_char,
        time: i64,
        mseconds: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn taos_consume(tsub: *mut ::std::os::raw::c_void) -> *mut *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn taos_unsubscribe(tsub: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn taos_open_stream(
        taos: *mut ::std::os::raw::c_void,
        sqlstr: *mut ::std::os::raw::c_char,
        fp: ::std::option::Option<
            unsafe extern "C" fn(
                param: *mut ::std::os::raw::c_void,
                arg1: *mut ::std::os::raw::c_void,
                row: *mut *mut ::std::os::raw::c_void,
            ),
        >,
        stime: i64,
        param: *mut ::std::os::raw::c_void,
        callback: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn taos_close_stream(tstr: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub static mut configDir: [::std::os::raw::c_char; 0usize];
}