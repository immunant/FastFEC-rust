use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type real_pcre8_or_16;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
pub type pcre = real_pcre8_or_16;
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
static mut headers: [[*const libc::c_char; 3]; 282] = [
    [
        b"^(P3|P2.6)\0" as *const u8 as *const libc::c_char,
        b"^hdr$\0" as *const u8 as *const libc::c_char,
        b"record_type,fec_version,soft_name,batch_number,received_date,report_id\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(P2.2|P2.3|P2.4)\0" as *const u8 as *const libc::c_char,
        b"^hdr$\0" as *const u8 as *const libc::c_char,
        b"record_type,fec_version,soft_name,batch_number,report_id\0" as *const u8
            as *const libc::c_char,
    ],
    [
        b"^P1\0" as *const u8 as *const libc::c_char,
        b"^hdr$\0" as *const u8 as *const libc::c_char,
        b"record_type,fec_version,soft_name,batch_number\0" as *const u8
            as *const libc::c_char,
    ],
    [
        b"^[6-8]\0" as *const u8 as *const libc::c_char,
        b"^hdr$\0" as *const u8 as *const libc::c_char,
        b"record_type,ef_type,fec_version,soft_name,soft_ver,report_id,report_number,comment\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^[3-5]\0" as *const u8 as *const libc::c_char,
        b"^hdr$\0" as *const u8 as *const libc::c_char,
        b"record_type,ef_type,fec_version,soft_name,soft_ver,name_delim,report_id,report_number,comment\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P3.2|^P3.3|^P3.4\0" as *const u8 as *const libc::c_char,
        b"^f1[an]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,change_of_committee_name,committee_name,change_of_address,street_1,street_2,city,state,zip_code,change_of_committee_email,committee_email,change_of_committee_url,committee_url,effective_date,signature_last_name,signature_first_name,signature_middle_name,signature_prefix,signature_suffix,date_signed,committee_type,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix,candidate_office,candidate_state,candidate_district,party_code,party_type,organization_type,lobbyist_registrant_pac,lobbyist_registrant_pac_2,leadership_pac,affiliated_committee_name,affiliated_last_name,affiliated_first_name,affiliated_middle_name,affiliated_prefix,affiliated_suffix,affiliated_street_1,affiliated_street_2,affiliated_city,affiliated_state,affiliated_zip_code,affiliated_relationship_code,custodian_last_name,custodian_first_name,custodian_middle_name,custodian_prefix,custodian_suffix,custodian_street_1,custodian_street_2,custodian_city,custodian_state,custodian_zip_code,custodian_title,custodian_telephone,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,treasurer_street_1,treasurer_street_2,treasurer_city,treasurer_state,treasurer_zip_code,treasurer_title,treasurer_telephone,agent_last_name,agent_first_name,agent_middle_name,agent_prefix,agent_suffix,agent_street_1,agent_street_2,agent_city,agent_state,agent_zip_code,agent_title,agent_telephone,bank_name,bank_street_1,bank_street_2,bank_city,bank_state,bank_zip_code,bank2_name,bank2_street_1,bank2_street_2,bank2_city,bank2_state,bank2_zip_code,beginning_image_number,end_image_number,receipt_date\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P2.6|^P3.0|^P3.1\0" as *const u8 as *const libc::c_char,
        b"^f1[an]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,change_of_committee_name,committee_name,change_of_address,street_1,street_2,city,state,zip_code,change_of_committee_email,committee_email,change_of_committee_url,committee_url,effective_date,signature_last_name,signature_first_name,signature_middle_name,signature_prefix,signature_suffix,date_signed,committee_type,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix,candidate_office,candidate_state,candidate_district,party_code,party_type,organization_type,lobbyist_registrant_pac,lobbyist_registrant_pac_2,leadership_pac,affiliated_committee_name,affiliated_last_name,affiliated_first_name,affiliated_middle_name,affiliated_prefix,affiliated_suffix,affiliated_street_1,affiliated_street_2,affiliated_city,affiliated_state,affiliated_zip_code,affiliated_relationship_code,custodian_last_name,custodian_first_name,custodian_middle_name,custodian_prefix,custodian_suffix,custodian_street_1,custodian_street_2,custodian_city,custodian_state,custodian_zip_code,custodian_title,custodian_telephone,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,treasurer_street_1,treasurer_street_2,treasurer_city,treasurer_state,treasurer_zip_code,treasurer_title,treasurer_telephone,agent_last_name,agent_first_name,agent_middle_name,agent_prefix,agent_suffix,agent_street_1,agent_street_2,agent_city,agent_state,agent_zip_code,agent_title,agent_telephone,bank_name,bank_street_1,bank_street_2,bank_city,bank_state,bank_zip_code,bank2_name,bank2_street_1,bank2_street_2,bank2_city,bank2_state,bank2_zip_code,beginning_image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P2.4\0" as *const u8 as *const libc::c_char,
        b"^f1[an]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,change_of_committee_name,committee_name,change_of_address,street_1,street_2,city,state,zip_code,committee_email,committee_url,committee_fax_number,effective_date,signature_last_name,signature_first_name,signature_middle_name,signature_prefix,signature_suffix,date_signed,committee_type,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix,candidate_office,candidate_state,candidate_district,party_code,party_type,organization_type,leadership_pac,affiliated_committee_name,affiliated_street_1,affiliated_street_2,affiliated_city,affiliated_state,affiliated_zip_code,affiliated_relationship_code,custodian_last_name,custodian_first_name,custodian_middle_name,custodian_prefix,custodian_suffix,custodian_street_1,custodian_street_2,custodian_city,custodian_state,custodian_zip_code,custodian_title,custodian_telephone,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,treasurer_street_1,treasurer_street_2,treasurer_city,treasurer_state,treasurer_zip_code,treasurer_title,treasurer_telephone,agent_last_name,agent_first_name,agent_middle_name,agent_prefix,agent_suffix,agent_street_1,agent_street_2,agent_city,agent_state,agent_zip_code,agent_title,agent_telephone,bank_name,bank_street_1,bank_street_2,bank_city,bank_state,bank_zip_code,bank2_name,bank2_street_1,bank2_street_2,bank2_city,bank2_state,bank2_zip_code,beginning_image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P1.0|^P2.2|^P2.3\0" as *const u8 as *const libc::c_char,
        b"^f1[an]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,change_of_committee_name,committee_name,change_of_address,street_1,street_2,city,state,zip_code,committee_email,committee_url,committee_fax_number,effective_date,signature_last_name,signature_first_name,signature_middle_name,signature_prefix,signature_suffix,date_signed,committee_type,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix,candidate_office,candidate_state,candidate_district,party_code,party_type,affiliated_committee_name,affiliated_street_1,affiliated_street_2,affiliated_city,affiliated_state,affiliated_zip_code,affiliated_relationship_code,organization_type,custodian_last_name,custodian_first_name,custodian_middle_name,custodian_prefix,custodian_suffix,custodian_street_1,custodian_street_2,custodian_city,custodian_state,custodian_zip_code,custodian_title,custodian_telephone,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,treasurer_street_1,treasurer_street_2,treasurer_city,treasurer_state,treasurer_zip_code,treasurer_title,treasurer_telephone,agent_last_name,agent_first_name,agent_middle_name,agent_prefix,agent_suffix,agent_street_1,agent_street_2,agent_city,agent_state,agent_zip_code,agent_title,agent_telephone,bank_name,bank_street_1,bank_street_2,bank_city,bank_state,bank_zip_code,beginning_image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4\0" as *const u8 as *const libc::c_char,
        b"^f1[an]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,change_of_committee_name,committee_name,change_of_address,street_1,street_2,city,state,zip_code,change_of_committee_email,committee_email,change_of_committee_url,committee_url,effective_date,signature_last_name,signature_first_name,signature_middle_name,signature_prefix,signature_suffix,date_signed,committee_type,candidate_id_number,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix,candidate_office,candidate_state,candidate_district,party_code,party_type,organization_type,lobbyist_registrant_pac,lobbyist_registrant_pac_2,leadership_pac,lobbyist_registrant_pac_3,lobbyist_registrant_pac_4,affiliated_committee_id_number,affiliated_committee_name,affiliated_candidate_id_number,affiliated_last_name,affiliated_first_name,affiliated_middle_name,affiliated_prefix,affiliated_suffix,affiliated_street_1,affiliated_street_2,affiliated_city,affiliated_state,affiliated_zip_code,affiliated_relationship_code,custodian_last_name,custodian_first_name,custodian_middle_name,custodian_prefix,custodian_suffix,custodian_street_1,custodian_street_2,custodian_city,custodian_state,custodian_zip_code,custodian_title,custodian_telephone,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,treasurer_street_1,treasurer_street_2,treasurer_city,treasurer_state,treasurer_zip_code,treasurer_title,treasurer_telephone,agent_last_name,agent_first_name,agent_middle_name,agent_prefix,agent_suffix,agent_street_1,agent_street_2,agent_city,agent_state,agent_zip_code,agent_title,agent_telephone,bank_name,bank_street_1,bank_street_2,bank_city,bank_state,bank_zip_code,bank2_name,bank2_street_1,bank2_street_2,bank2_city,bank2_state,bank2_zip_code\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.3|8.2|8.1|8.0|7.0|6.4\0" as *const u8 as *const libc::c_char,
        b"^f1[an]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,change_of_committee_name,committee_name,change_of_address,street_1,street_2,city,state,zip_code,change_of_committee_email,committee_email,change_of_committee_url,committee_url,effective_date,signature_last_name,signature_first_name,signature_middle_name,signature_prefix,signature_suffix,date_signed,committee_type,candidate_id_number,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix,candidate_office,candidate_state,candidate_district,party_code,party_type,organization_type,lobbyist_registrant_pac,lobbyist_registrant_pac_2,leadership_pac,affiliated_committee_id_number,affiliated_committee_name,affiliated_candidate_id_number,affiliated_last_name,affiliated_first_name,affiliated_middle_name,affiliated_prefix,affiliated_suffix,affiliated_street_1,affiliated_street_2,affiliated_city,affiliated_state,affiliated_zip_code,affiliated_relationship_code,custodian_last_name,custodian_first_name,custodian_middle_name,custodian_prefix,custodian_suffix,custodian_street_1,custodian_street_2,custodian_city,custodian_state,custodian_zip_code,custodian_title,custodian_telephone,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,treasurer_street_1,treasurer_street_2,treasurer_city,treasurer_state,treasurer_zip_code,treasurer_title,treasurer_telephone,agent_last_name,agent_first_name,agent_middle_name,agent_prefix,agent_suffix,agent_street_1,agent_street_2,agent_city,agent_state,agent_zip_code,agent_title,agent_telephone,bank_name,bank_street_1,bank_street_2,bank_city,bank_state,bank_zip_code,bank2_name,bank2_street_1,bank2_street_2,bank2_city,bank2_state,bank2_zip_code\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^6.3|6.2|6.1\0" as *const u8 as *const libc::c_char,
        b"^f1[an]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,change_of_committee_name,committee_name,change_of_address,street_1,street_2,city,state,zip_code,change_of_committee_email,committee_email,change_of_committee_url,committee_url,effective_date,signature_last_name,signature_first_name,signature_middle_name,signature_prefix,signature_suffix,date_signed,committee_type,candidate_id_number,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix,candidate_office,candidate_state,candidate_district,party_code,party_type,organization_type,lobbyist_registrant_pac,lobbyist_registrant_pac_2,leadership_pac,affiliated_committee_id_number,affiliated_committee_name,affiliated_candidate_id_number,affiliated_last_name,affiliated_first_name,affiliated_middle_name,affiliated_prefix,affiliated_suffix,affiliated_street_1,affiliated_street_2,affiliated_city,affiliated_state,affiliated_zip_code,affiliated_relationship_code,custodian_last_name,custodian_first_name,custodian_middle_name,custodian_prefix,custodian_suffix,custodian_street_1,custodian_street_2,custodian_city,custodian_state,custodian_zip_code,custodian_title,custodian_telephone,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,treasurer_street_1,treasurer_street_2,treasurer_city,treasurer_state,treasurer_zip_code,treasurer_title,treasurer_telephone,agent_last_name,agent_first_name,agent_middle_name,agent_prefix,agent_suffix,agent_street_1,agent_street_2,agent_city,agent_state,agent_zip_code,agent_title,agent_telephone,bank_name,bank_street_1,bank_street_2,bank_city,bank_state,bank_zip_code,bank2_name,bank2_street_1,bank2_street_2,bank2_city,bank2_state,bank2_zip_code\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3|5.2|5.1|5.0\0" as *const u8 as *const libc::c_char,
        b"^f1[an]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,street_1,street_2,city,state,zip_code,effective_date,change_of_committee_name,change_of_address,committee_type,candidate_id_number,candidate_name,candidate_office,candidate_state,candidate_district,party_code,party_type,affiliated_committee_id_number,affiliated_committee_name,affiliated_street_1,affiliated_street_2,affiliated_city,affiliated_state,affiliated_zip_code,affiliated_relationship_code,organization_type,custodian_name,custodian_street_1,custodian_street_2,custodian_city,custodian_state,custodian_zip_code,custodian_title,custodian_telephone,treasurer_name,treasurer_street_1,treasurer_street_2,treasurer_city,treasurer_state,treasurer_zip_code,treasurer_title,treasurer_telephone,agent_name,agent_street_1,agent_street_2,agent_city,agent_state,agent_zip_code,agent_title,agent_telephone,bank_name,bank_street_1,bank_street_2,bank_city,bank_state,bank_zip_code,signature_name,date_signed,committee_email,committee_url,committee_fax_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^3.0\0" as *const u8 as *const libc::c_char,
        b"^f1[an]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,street_1,street_2,city,state,zip_code,effective_date,change_of_committee_name,change_of_address,committee_type,candidate_id_number,candidate_name,candidate_office,candidate_state,candidate_district,party_code,party_type,affiliated_committee_id_number,affiliated_committee_name,affiliated_street_1,affiliated_street_2,affiliated_city,affiliated_state,affiliated_zip_code,affiliated_relationship_code,organization_type,custodian_name,custodian_street_1,custodian_street_2,custodian_city,custodian_state,custodian_zip_code,custodian_title,custodian_telephone,treasurer_name,treasurer_street_1,treasurer_street_2,treasurer_city,treasurer_state,treasurer_zip_code,treasurer_title,treasurer_telephone,agent_name,agent_street_1,agent_street_2,agent_city,agent_state,agent_zip_code,agent_title,agent_telephone,bank_name,bank_street_1,bank_street_2,bank_city,bank_state,bank_zip_code,signature_name,date_signed,committee_email,committee_url\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^2\0" as *const u8 as *const libc::c_char,
        b"^f1[an]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,street_1,street_2,city,state,zip_code,effective_date,change_of_committee_name,change_of_address,committee_type,candidate_id_number,candidate_name,candidate_office,candidate_state,candidate_district,party_code,party_type,affiliated_committee_id_number,affiliated_committee_name,affiliated_street_1,affiliated_street_2,affiliated_city,affiliated_state,affiliated_zip_code,affiliated_relationship_code,organization_type,custodian_name,custodian_street_1,custodian_street_2,custodian_city,custodian_state,custodian_zip_code,custodian_title,custodian_telephone,treasurer_name,treasurer_street_1,treasurer_street_2,treasurer_city,treasurer_state,treasurer_zip_code,treasurer_title,treasurer_telephone,agent_name,agent_street_1,agent_street_2,agent_city,agent_state,agent_zip_code,agent_title,agent_telephone,bank_name,bank_street_1,bank_street_2,bank_city,bank_state,bank_zip_code,signature_name,date_signed\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P3.2|^P3.3|^P3.4\0" as *const u8 as *const libc::c_char,
        b"^f13[an]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,change_of_address,street_1,street_2,city,state,zip_code,report_code,amendment_date,coverage_from_date,coverage_through_date,total_donations_accepted,total_donations_refunded,net_donations,designated_last_name,designated_first_name,designated_middle_name,designated_prefix,designated_suffix,date_signed,beginning_image_number,end_image_number,receipt_date\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P3.0|^P3.1\0" as *const u8 as *const libc::c_char,
        b"^f13[an]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,change_of_address,street_1,street_2,city,state,zip_code,report_code,amendment_date,coverage_from_date,coverage_through_date,total_donations_accepted,total_donations_refunded,net_donations,designated_last_name,designated_first_name,designated_middle_name,designated_prefix,designated_suffix,date_signed,beginning_image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0|7.0|6.4|6.3|6.2|6.1\0" as *const u8
            as *const libc::c_char,
        b"^f13[an]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,change_of_address,street_1,street_2,city,state,zip_code,report_code,amendment_date,coverage_from_date,coverage_through_date,total_donations_accepted,total_donations_refunded,net_donations,designated_last_name,designated_first_name,designated_middle_name,designated_prefix,designated_suffix,date_signed\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3|5.2\0" as *const u8 as *const libc::c_char,
        b"^f13[an]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,street_1,street_2,city,state,zip_code,change_of_address,report_code,amendment_date,coverage_from_date,coverage_through_date,total_donations_accepted,total_donations_refunded,net_donations,designated_last_name,designated_first_name,designated_middle_name,designated_prefix,designated_suffix,date_signed\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P3.2|^P3.3|^P3.4\0" as *const u8 as *const libc::c_char,
        b"^f132\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,contributor_organization_name,contributor_last_name,contributor_first_name,contributor_middle_name,contributor_prefix,contributor_suffix,contributor_street_1,contributor_street_2,contributor_city,contributor_state,contributor_zip,donation_date,donation_amount,donation_aggregate_amount,memo_code,memo_text_description,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P3.0|^P3.1\0" as *const u8 as *const libc::c_char,
        b"^f132\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,contributor_organization_name,contributor_last_name,contributor_first_name,contributor_middle_name,contributor_prefix,contributor_suffix,contributor_street_1,contributor_street_2,contributor_city,contributor_state,contributor_zip,donation_date,donation_amount,donation_aggregate_amount,memo_text_description,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0|7.0|6.4|6.3|6.2|6.1\0" as *const u8
            as *const libc::c_char,
        b"^f132\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id_number,back_reference_tran_id_number,back_reference_sched_name,entity_type,contributor_organization_name,contributor_last_name,contributor_first_name,contributor_middle_name,contributor_prefix,contributor_suffix,contributor_street_1,contributor_street_2,contributor_city,contributor_state,contributor_zip,donation_date,donation_amount,donation_aggregate_amount,memo_code,memo_text_description\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3|5.2\0" as *const u8 as *const libc::c_char,
        b"^f132\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,contributor_organization_name,contributor_last_name,contributor_first_name,contributor_middle_name,contributor_prefix,contributor_suffix,contributor_street_1,contributor_street_2,contributor_city,contributor_state,contributor_zip,donation_date,donation_amount,donation_aggregate_amount,,transaction_id_number,back_reference_tran_id_number,back_reference_sched_name\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P3.2|^P3.3|^P3.4\0" as *const u8 as *const libc::c_char,
        b"^f133\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,contributor_organization_name,contributor_last_name,contributor_first_name,contributor_middle_name,contributor_prefix,contributor_suffix,contributor_street_1,contributor_street_2,contributor_city,contributor_state,contributor_zip,refund_date,refund_amount,memo_code,memo_text_description,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P3.0|^P3.1\0" as *const u8 as *const libc::c_char,
        b"^f133\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,contributor_organization_name,contributor_last_name,contributor_first_name,contributor_middle_name,contributor_prefix,contributor_suffix,contributor_street_1,contributor_street_2,contributor_city,contributor_state,contributor_zip,refund_date,refund_amount,memo_text_description,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0|7.0|6.4|6.3|6.2|6.1\0" as *const u8
            as *const libc::c_char,
        b"^f133\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id_number,back_reference_tran_id_number,back_reference_sched_name,entity_type,contributor_organization_name,contributor_last_name,contributor_first_name,contributor_middle_name,contributor_prefix,contributor_suffix,contributor_street_1,contributor_street_2,contributor_city,contributor_state,contributor_zip,refund_date,refund_amount,memo_code,memo_text_description\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3|5.2\0" as *const u8 as *const libc::c_char,
        b"^f133\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,contributor_organization_name,contributor_last_name,contributor_first_name,contributor_middle_name,contributor_prefix,contributor_suffix,contributor_street_1,contributor_street_2,contributor_city,contributor_state,contributor_zip,refund_date,refund_amount,,transaction_id_number,back_reference_tran_id_number,back_reference_sched_name\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P3.2|^P3.3|^P3.4\0" as *const u8 as *const libc::c_char,
        b"^(f1m$|f1m[a|n])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,street_1,street_2,city,state,zip_code,committee_type,affiliated_date_f1_filed,affiliated_committee_name,affiliated_committee_id_number,first_candidate_last_name,first_candidate_first_name,first_candidate_middle_name,first_candidate_prefix,first_candidate_suffix,first_candidate_office,first_candidate_state,first_candidate_district,first_candidate_contribution_date,second_candidate_last_name,second_candidate_first_name,second_candidate_middle_name,second_candidate_prefix,second_candidate_suffix,second_candidate_office,second_candidate_state,second_candidate_district,second_candidate_contribution_date,third_candidate_last_name,third_candidate_first_name,third_candidate_middle_name,third_candidate_prefix,third_candidate_suffix,third_candidate_office,third_candidate_state,third_candidate_district,third_candidate_contribution_date,fourth_candidate_last_name,fourth_candidate_first_name,fourth_candidate_middle_name,fourth_candidate_prefix,fourth_candidate_suffix,fourth_candidate_office,fourth_candidate_state,fourth_candidate_district,fourth_candidate_contribution_date,fifth_candidate_last_name,fifth_candidate_first_name,fifth_candidate_middle_name,fifth_candidate_prefix,fifth_candidate_suffix,fifth_candidate_office,fifth_candidate_state,fifth_candidate_district,fifth_candidate_contribution_date,fifty_first_contributor_date,original_registration_date,requirements_met_date,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,date_signed,beginning_image_number,end_image_number,receipt_date\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P1|^P2|^P3.0|^P3.1\0" as *const u8 as *const libc::c_char,
        b"^(f1m$|f1m[a|n])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,street_1,street_2,city,state,zip_code,committee_type,affiliated_date_f1_filed,affiliated_committee_name,affiliated_committee_id_number,first_candidate_last_name,first_candidate_first_name,first_candidate_middle_name,first_candidate_prefix,first_candidate_suffix,first_candidate_office,first_candidate_state,first_candidate_district,first_candidate_contribution_date,second_candidate_last_name,second_candidate_first_name,second_candidate_middle_name,second_candidate_prefix,second_candidate_suffix,second_candidate_office,second_candidate_state,second_candidate_district,second_candidate_contribution_date,third_candidate_last_name,third_candidate_first_name,third_candidate_middle_name,third_candidate_prefix,third_candidate_suffix,third_candidate_office,third_candidate_state,third_candidate_district,third_candidate_contribution_date,fourth_candidate_last_name,fourth_candidate_first_name,fourth_candidate_middle_name,fourth_candidate_prefix,fourth_candidate_suffix,fourth_candidate_office,fourth_candidate_state,fourth_candidate_district,fourth_candidate_contribution_date,fifth_candidate_last_name,fifth_candidate_first_name,fifth_candidate_middle_name,fifth_candidate_prefix,fifth_candidate_suffix,fifth_candidate_office,fifth_candidate_state,fifth_candidate_district,fifth_candidate_contribution_date,fifty_first_contributor_date,original_registration_date,requirements_met_date,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,date_signed,beginning_image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0|7.0|6.4|6.3|6.2|6.1\0" as *const u8
            as *const libc::c_char,
        b"^(f1m$|f1m[a|n])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,street_1,street_2,city,state,zip_code,committee_type,affiliated_date_f1_filed,affiliated_committee_id_number,affiliated_committee_name,first_candidate_id_number,first_candidate_last_name,first_candidate_first_name,first_candidate_middle_name,first_candidate_prefix,first_candidate_suffix,first_candidate_office,first_candidate_state,first_candidate_district,first_candidate_contribution_date,second_candidate_id_number,second_candidate_last_name,second_candidate_first_name,second_candidate_middle_name,second_candidate_prefix,second_candidate_suffix,second_candidate_office,second_candidate_state,second_candidate_district,second_candidate_contribution_date,third_candidate_id_number,third_candidate_last_name,third_candidate_first_name,third_candidate_middle_name,third_candidate_prefix,third_candidate_suffix,third_candidate_office,third_candidate_state,third_candidate_district,third_candidate_contribution_date,fourth_candidate_id_number,fourth_candidate_last_name,fourth_candidate_first_name,fourth_candidate_middle_name,fourth_candidate_prefix,fourth_candidate_suffix,fourth_candidate_office,fourth_candidate_state,fourth_candidate_district,fourth_candidate_contribution_date,fifth_candidate_id_number,fifth_candidate_last_name,fifth_candidate_first_name,fifth_candidate_middle_name,fifth_candidate_prefix,fifth_candidate_suffix,fifth_candidate_office,fifth_candidate_state,fifth_candidate_district,fifth_candidate_contribution_date,fifty_first_contributor_date,original_registration_date,requirements_met_date,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,date_signed\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3|5.2|5.1|5.0|^3.0\0" as *const u8 as *const libc::c_char,
        b"^(f1m$|f1m[a|n])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,street_1,street_2,city,state,zip_code,committee_type,affiliated_date_f1_filed,affiliated_committee_id_number,affiliated_committee_name,first_candidate_id_number,first_candidate_name,first_candidate_office,first_candidate_state,first_candidate_district,first_candidate_contribution_date,second_candidate_id_number,second_candidate_name,second_candidate_office,second_candidate_state,second_candidate_district,second_candidate_contribution_date,third_candidate_id_number,third_candidate_name,third_candidate_office,third_candidate_state,third_candidate_district,third_candidate_contribution_date,fourth_candidate_id_number,fourth_candidate_name,fourth_candidate_office,fourth_candidate_state,fourth_candidate_district,fourth_candidate_contribution_date,fifth_candidate_id_number,fifth_candidate_name,fifth_candidate_office,fifth_candidate_state,fifth_candidate_district,fifth_candidate_contribution_date,fifty_first_contributor_date,original_registration_date,requirements_met_date,treasurer_name,date_signed\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P2.6|^P3\0" as *const u8 as *const libc::c_char,
        b"^f1s\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,joint_fund_participant_committee_name,joint_fund_participant_committee_id_number,affiliated_committee_name,affiliated_last_name,affiliated_first_name,affiliated_middle_name,affiliated_prefix,affiliated_suffix,affiliated_street_1,affiliated_street_2,affiliated_city,affiliated_state,affiliated_zip_code,affiliated_relationship_code,agent_last_name,agent_first_name,agent_middle_name,agent_prefix,agent_suffix,agent_street_1,agent_street_2,agent_city,agent_state,agent_zip_code,agent_title,agent_telephone,bank_name,bank_street_1,bank_street_2,bank_city,bank_state,bank_zip_code,beginning_image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P2.4\0" as *const u8 as *const libc::c_char,
        b"^f1s\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,joint_fund_participant_committee_name,joint_fund_participant_committee_id_number,affiliated_committee_name,affiliated_street_1,affiliated_street_2,affiliated_city,affiliated_state,affiliated_zip_code,affiliated_relationship_code,agent_last_name,agent_first_name,agent_middle_name,agent_prefix,agent_suffix,agent_street_1,agent_street_2,agent_city,agent_state,agent_zip_code,agent_title,agent_telephone,bank_name,bank_street_1,bank_street_2,bank_city,bank_state,bank_zip_code,beginning_image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P1|^P2.2|^P2.3\0" as *const u8 as *const libc::c_char,
        b"^f1s\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,affiliated_committee_name,affiliated_street_1,affiliated_street_2,affiliated_city,affiliated_state,affiliated_zip_code,affiliated_relationship_code,affiliated_organization_type,agent_last_name,agent_first_name,agent_middle_name,agent_prefix,agent_suffix,agent_street_1,agent_street_2,agent_city,agent_state,agent_zip_code,agent_title,agent_telephone,bank_name,bank_street_1,bank_street_2,bank_city,bank_state,bank_zip_code,beginning_image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0|7.0|6.4|6.3|6.2\0" as *const u8 as *const libc::c_char,
        b"^f1s\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,joint_fund_participant_committee_name,joint_fund_participant_committee_id_number,affiliated_committee_id_number,affiliated_committee_name,affiliated_candidate_id_number,affiliated_last_name,affiliated_first_name,affiliated_middle_name,affiliated_prefix,affiliated_suffix,affiliated_street_1,affiliated_street_2,affiliated_city,affiliated_state,affiliated_zip_code,affiliated_relationship_code,agent_last_name,agent_first_name,agent_middle_name,agent_prefix,agent_suffix,agent_street_1,agent_street_2,agent_city,agent_state,agent_zip_code,agent_title,agent_telephone,bank_name,bank_street_1,bank_street_2,bank_city,bank_state,bank_zip_code\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^6.1\0" as *const u8 as *const libc::c_char,
        b"^f1s\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,affiliated_committee_id_number,affiliated_committee_name,affiliated_street_1,affiliated_street_2,affiliated_city,affiliated_state,affiliated_zip_code,affiliated_relationship_code,,agent_last_name,agent_first_name,agent_middle_name,agent_prefix,agent_suffix,agent_street_1,agent_street_2,agent_city,agent_state,agent_zip_code,agent_title,agent_telephone,bank_name,bank_street_1,bank_street_2,bank_city,bank_state,bank_zip_code\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3|5.2|5.1|5.0\0" as *const u8 as *const libc::c_char,
        b"^f1s\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    ],
    [
        b"^3.0\0" as *const u8 as *const libc::c_char,
        b"^f1s\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    ],
    [
        b"^P3.4\0" as *const u8 as *const libc::c_char,
        b"(^f2$)|(^f2[^4])\0" as *const u8 as *const libc::c_char,
        b"form_type,candidate_id_number,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix,candidate_street_1,candidate_street_2,change_of_address,candidate_city,candidate_state,candidate_zip_code,candidate_party_code,candidate_office,candidate_state,candidate_district,election_year,committee_name,committee_street_1,committee_street_2,committee_city,committee_state,committee_zip_code,authorized_committee_name,authorized_committee_street_1,authorized_committee_street_2,authorized_committee_city,authorized_committee_state,authorized_committee_zip_code,candidate_signature_last_name,candidate_signature_first_name,candidate_signature_middle_name,candidate_signature_prefix,candidate_signature_suffix,date_signed,beginning_image_number,end_image_number,receipt_date,vice_president_last_name,vice_president_first_name,vice_president_middle_name,vice_president_prefix,vice_president_suffix\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P3.3|^P3.2\0" as *const u8 as *const libc::c_char,
        b"(^f2$)|(^f2[^4])\0" as *const u8 as *const libc::c_char,
        b"form_type,candidate_id_number,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix,candidate_street_1,candidate_street_2,change_of_address,candidate_city,candidate_state,candidate_zip_code,candidate_party_code,candidate_office,candidate_state,candidate_district,election_year,committee_name,committee_street_1,committee_street_2,committee_city,committee_state,committee_zip_code,authorized_committee_name,authorized_committee_street_1,authorized_committee_street_2,authorized_committee_city,authorized_committee_state,authorized_committee_zip_code,candidate_signature_last_name,candidate_signature_first_name,candidate_signature_middle_name,candidate_signature_prefix,candidate_signature_suffix,date_signed,beginning_image_number,end_image_number,receipt_date\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P(3.1|3.0|2.6)\0" as *const u8 as *const libc::c_char,
        b"(^f2$)|(^f2[^4])\0" as *const u8 as *const libc::c_char,
        b"form_type,candidate_id_number,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix,candidate_street_1,candidate_street_2,change_of_address,candidate_city,candidate_state,candidate_zip_code,candidate_party_code,candidate_office,candidate_state,candidate_district,election_year,committee_name,committee_street_1,committee_street_2,committee_city,committee_state,committee_zip_code,authorized_committee_name,authorized_committee_street_1,authorized_committee_street_2,authorized_committee_city,authorized_committee_state,authorized_committee_zip_code,candidate_signature_last_name,candidate_signature_first_name,candidate_signature_middle_name,candidate_signature_prefix,candidate_signature_suffix,date_signed,beginning_image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P(2.4|2.3|2.2|1)\0" as *const u8 as *const libc::c_char,
        b"(^f2$)|(^f2[^4])\0" as *const u8 as *const libc::c_char,
        b"form_type,candidate_id_number,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix,candidate_street_1,candidate_street_2,change_of_address,candidate_city,candidate_state,candidate_zip_code,candidate_party_code,candidate_office,candidate_state,candidate_district,election_year,committee_name,committee_street_1,committee_street_2,committee_city,committee_state,committee_zip_code,authorized_committee_name,authorized_committee_street_1,authorized_committee_street_2,authorized_committee_city,authorized_committee_state,authorized_committee_zip_code,primary_personal_funds_declared,general_personal_funds_declared,candidate_signature_last_name,candidate_signature_first_name,candidate_signature_middle_name,candidate_signature_prefix,candidate_signature_suffix,date_signed,beginning_image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2\0" as *const u8 as *const libc::c_char,
        b"(^f2$)|(^f2[^4])\0" as *const u8 as *const libc::c_char,
        b"form_type,candidate_id_number,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix,vice_president_last_name,vice_president_first_name,vice_president_middle_name,vice_president_prefix,vice_president_suffix,change_of_address,candidate_street_1,candidate_street_2,candidate_city,candidate_state,candidate_zip_code,candidate_party_code,candidate_office,candidate_state,candidate_district,election_year,committee_id_number,committee_name,committee_street_1,committee_street_2,committee_city,committee_state,committee_zip_code,authorized_committee_id_number,authorized_committee_name,authorized_committee_street_1,authorized_committee_street_2,authorized_committee_city,authorized_committee_state,authorized_committee_zip_code,candidate_signature_last_name,candidate_signature_first_name,candidate_signature_middle_name,candidate_signature_prefix,candidate_signature_suffix,date_signed\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.1|8.0|7.0|6.4\0" as *const u8 as *const libc::c_char,
        b"(^f2$)|(^f2[^4])\0" as *const u8 as *const libc::c_char,
        b"form_type,candidate_id_number,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix,change_of_address,candidate_street_1,candidate_street_2,candidate_city,candidate_state,candidate_zip_code,candidate_party_code,candidate_office,candidate_state,candidate_district,election_year,committee_id_number,committee_name,committee_street_1,committee_street_2,committee_city,committee_state,committee_zip_code,authorized_committee_id_number,authorized_committee_name,authorized_committee_street_1,authorized_committee_street_2,authorized_committee_city,authorized_committee_state,authorized_committee_zip_code,candidate_signature_last_name,candidate_signature_first_name,candidate_signature_middle_name,candidate_signature_prefix,candidate_signature_suffix,date_signed\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^6.3|6.2|6.1\0" as *const u8 as *const libc::c_char,
        b"(^f2$)|(^f2[^4])\0" as *const u8 as *const libc::c_char,
        b"form_type,candidate_id_number,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix,change_of_address,candidate_street_1,candidate_street_2,candidate_city,candidate_state,candidate_zip_code,candidate_party_code,candidate_office,candidate_state,candidate_district,election_year,committee_id_number,committee_name,committee_street_1,committee_street_2,committee_city,committee_state,committee_zip_code,authorized_committee_id_number,authorized_committee_name,authorized_committee_street_1,authorized_committee_street_2,authorized_committee_city,authorized_committee_state,authorized_committee_zip_code,primary_personal_funds_declared,general_personal_funds_declared,candidate_signature_last_name,candidate_signature_first_name,candidate_signature_middle_name,candidate_signature_prefix,candidate_signature_suffix,date_signed\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3|5.2|5.1|5.0\0" as *const u8 as *const libc::c_char,
        b"(^f2$)|(^f2[^4])\0" as *const u8 as *const libc::c_char,
        b"form_type,candidate_id_number,candidate_name,candidate_street_1,candidate_street_2,candidate_city,candidate_state,candidate_zip_code,candidate_party_code,candidate_office,candidate_state,candidate_district,election_year,committee_id_number,committee_name,committee_street_1,committee_street_2,committee_city,committee_state,committee_zip_code,authorized_committee_id_number,authorized_committee_name,authorized_committee_street_1,authorized_committee_street_2,authorized_committee_city,authorized_committee_state,authorized_committee_zip_code,candidate_signature_name,date_signed,primary_personal_funds_declared,general_personal_funds_declared,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^3.0\0" as *const u8 as *const libc::c_char,
        b"(^f2$)|(^f2[^4])\0" as *const u8 as *const libc::c_char,
        b"form_type,candidate_id_number,candidate_name,candidate_street_1,candidate_street_2,candidate_city,candidate_state,candidate_zip_code,candidate_party_code,candidate_office,candidate_state,candidate_district,election_year,committee_id_number,committee_name,committee_street_1,committee_street_2,committee_city,committee_state,committee_zip_code,authorized_committee_id_number,authorized_committee_name,authorized_committee_street_1,authorized_committee_street_2,authorized_committee_city,authorized_committee_state,authorized_committee_zip_code,candidate_signature_name,date_signed\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(P3.4|P3.3|P3.2)\0" as *const u8 as *const libc::c_char,
        b"(^f24$)|(^f24[an])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,report_type,original_amendment_date,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,date_signed,beginning_image_number,end_image_number,receipt_date\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(P3.1|P3.0)\0" as *const u8 as *const libc::c_char,
        b"(^f24$)|(^f24[an])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,report_type,original_amendment_date,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,date_signed,beginning_image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(P2|P1)\0" as *const u8 as *const libc::c_char,
        b"(^f24$)|(^f24[an])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,report_type,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,date_signed,beginning_image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0\0" as *const u8 as *const libc::c_char,
        b"(^f24$)|(^f24[an])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,report_type,original_amendment_date,committee_name,street_1,street_2,city,state,zip_code,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,date_signed\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^7.0|6.4|6.3|6.2|6.1\0" as *const u8 as *const libc::c_char,
        b"(^f24$)|(^f24[an])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,report_type,committee_name,street_1,street_2,city,state,zip_code,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,date_signed\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.0|5.1|5.2|5.3\0" as *const u8 as *const libc::c_char,
        b"(^f24$)|(^f24[an])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,street_1,street_2,city,state,zip_code,,date_signed,report_type\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^3\0" as *const u8 as *const libc::c_char,
        b"(^f24$)|(^f24[an])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,street_1,street_2,city,state,zip_code,,date_signed\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P3.2|^P3.3|^P3.4\0" as *const u8 as *const libc::c_char,
        b"^f3[a|n|t]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,change_of_address,street_1,street_2,city,state,zip_code,election_state,election_district,report_code,election_date,state_of_election,coverage_from_date,coverage_through_date,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,date_signed,col_a_total_contributions_no_loans,col_a_total_contributions_refunds,col_a_net_contributions,col_a_total_operating_expenditures,col_a_total_offset_to_operating_expenditures,col_a_net_operating_expenditures,col_a_cash_on_hand_close_of_period,col_a_debts_to,col_a_debts_by,col_a_individual_contributions_itemized,col_a_individual_contributions_unitemized,col_a_total_individual_contributions,col_a_political_party_contributions,col_a_pac_contributions,col_a_candidate_contributions,col_a_total_contributions,col_a_transfers_from_authorized,col_a_candidate_loans,col_a_other_loans,col_a_total_loans,col_a_offset_to_operating_expenditures,col_a_other_receipts,col_a_total_receipts,col_a_operating_expenditures,col_a_transfers_to_authorized,col_a_candidate_loan_repayments,col_a_other_loan_repayments,col_a_total_loan_repayments,col_a_refunds_to_individuals,col_a_refunds_to_party_committees,col_a_refunds_to_other_committees,col_a_total_refunds,col_a_other_disbursements,col_a_total_disbursements,col_b_total_contributions_no_loans,col_b_total_contributions_refunds,col_b_net_contributions,col_b_total_operating_expenditures,col_b_total_offset_to_operating_expenditures,col_b_net_operating_expenditures,col_b_individual_contributions_itemized,col_b_individual_contributions_unitemized,col_b_total_individual_contributions,col_b_political_party_contributions,col_b_pac_contributions,col_b_candidate_contributions,col_b_total_contributions,col_b_transfers_from_authorized,col_b_candidate_loans,col_b_other_loans,col_b_total_loans,col_b_offset_to_operating_expenditures,col_b_other_receipts,col_b_total_receipts,col_b_operating_expenditures,col_b_transfers_to_authorized,col_b_candidate_loan_repayments,col_b_other_loan_repayments,col_b_total_loan_repayments,col_b_refunds_to_individuals,col_b_refunds_to_party_committees,col_b_refunds_to_other_committees,col_b_total_refunds,col_b_other_disbursements,col_b_total_disbursements,col_a_cash_beginning_reporting_period,col_a_total_receipts_period,col_a_subtotals,col_a_total_disbursements_period,col_a_cash_on_hand_close,beginning_image_number,end_image_number,receipt_date\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(P2.6|P3.0|P3.1)\0" as *const u8 as *const libc::c_char,
        b"^f3[a|n|t]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,change_of_address,street_1,street_2,city,state,zip_code,election_state,election_district,report_code,election_date,state_of_election,coverage_from_date,coverage_through_date,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,date_signed,col_a_total_contributions_no_loans,col_a_total_contributions_refunds,col_a_net_contributions,col_a_total_operating_expenditures,col_a_total_offset_to_operating_expenditures,col_a_net_operating_expenditures,col_a_cash_on_hand_close_of_period,col_a_debts_to,col_a_debts_by,col_a_individual_contributions_itemized,col_a_individual_contributions_unitemized,col_a_total_individual_contributions,col_a_political_party_contributions,col_a_pac_contributions,col_a_candidate_contributions,col_a_total_contributions,col_a_transfers_from_authorized,col_a_candidate_loans,col_a_other_loans,col_a_total_loans,col_a_offset_to_operating_expenditures,col_a_other_receipts,col_a_total_receipts,col_a_operating_expenditures,col_a_transfers_to_authorized,col_a_candidate_loan_repayments,col_a_other_loan_repayments,col_a_total_loan_repayments,col_a_refunds_to_individuals,col_a_refunds_to_party_committees,col_a_refunds_to_other_committees,col_a_total_refunds,col_a_other_disbursements,col_a_total_disbursements,col_b_total_contributions_no_loans,col_b_total_contributions_refunds,col_b_net_contributions,col_b_total_operating_expenditures,col_b_total_offset_to_operating_expenditures,col_b_net_operating_expenditures,col_b_individual_contributions_itemized,col_b_individual_contributions_unitemized,col_b_total_individual_contributions,col_b_political_party_contributions,col_b_pac_contributions,col_b_candidate_contributions,col_b_total_contributions,col_b_transfers_from_authorized,col_b_candidate_loans,col_b_other_loans,col_b_total_loans,col_b_offset_to_operating_expenditures,col_b_other_receipts,col_b_total_receipts,col_b_operating_expenditures,col_b_transfers_to_authorized,col_b_candidate_loan_repayments,col_b_other_loan_repayments,col_b_total_loan_repayments,col_b_refunds_to_individuals,col_b_refunds_to_party_committees,col_b_refunds_to_other_committees,col_b_total_refunds,col_b_other_disbursements,col_b_total_disbursements,col_a_cash_beginning_reporting_period,col_a_total_receipts_period,col_a_subtotals,col_a_total_disbursements_period,col_a_cash_on_hand_close,beginning_image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(P1|P2.2|P2.3|P2.4)\0" as *const u8 as *const libc::c_char,
        b"^f3[a|n|t]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,change_of_address,street_1,street_2,city,state,zip_code,election_state,election_district,report_code,election_date,state_of_election,coverage_from_date,coverage_through_date,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,date_signed,col_a_total_contributions_no_loans,col_a_total_contributions_refunds,col_a_net_contributions,col_a_total_operating_expenditures,col_a_total_offset_to_operating_expenditures,col_a_net_operating_expenditures,col_a_cash_on_hand_close_of_period,col_a_debts_to,col_a_debts_by,col_a_individual_contributions_itemized,col_a_individual_contributions_unitemized,col_a_total_individual_contributions,col_a_political_party_contributions,col_a_pac_contributions,col_a_candidate_contributions,col_a_total_contributions,col_a_transfers_from_authorized,col_a_candidate_loans,col_a_other_loans,col_a_total_loans,col_a_offset_to_operating_expenditures,col_a_other_receipts,col_a_total_receipts,col_a_operating_expenditures,col_a_transfers_to_authorized,col_a_candidate_loan_repayments,col_a_other_loan_repayments,col_a_total_loan_repayments,col_a_refunds_to_individuals,col_a_refunds_to_party_committees,col_a_refunds_to_other_committees,col_a_total_refunds,col_a_other_disbursements,col_a_total_disbursements,col_b_total_contributions_no_loans,col_b_total_contributions_refunds,col_b_net_contributions,col_b_total_operating_expenditures,col_b_total_offset_to_operating_expenditures,col_b_net_operating_expenditures,col_b_individual_contributions_itemized,col_b_individual_contributions_unitemized,col_b_total_individual_contributions,col_b_political_party_contributions,col_b_pac_contributions,col_b_candidate_contributions,col_b_total_contributions,col_b_transfers_from_authorized,col_b_candidate_loans,col_b_other_loans,col_b_total_loans,col_b_offset_to_operating_expenditures,col_b_other_receipts,col_b_total_receipts,col_b_operating_expenditures,col_b_transfers_to_authorized,col_b_candidate_loan_repayments,col_b_other_loan_repayments,col_b_total_loan_repayments,col_b_refunds_to_individuals,col_b_refunds_to_party_committees,col_b_refunds_to_other_committees,col_b_total_refunds,col_b_other_disbursements,col_b_total_disbursements,col_a_cash_beginning_reporting_period,col_a_total_receipts_period,col_a_subtotals,col_a_total_disbursements_period,col_a_cash_on_hand_close,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix,candidate_id_number,report_type,col_b_gross_receipts_authorized_primary,col_b_aggregate_personal_funds_primary,col_b_gross_receipts_minus_personal_funds_primary,col_b_gross_receipts_authorized_general,col_b_aggregate_personal_funds_general,col_b_gross_receipts_minus_personal_funds_general,beginning_image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0|7.0|6.4\0" as *const u8 as *const libc::c_char,
        b"^f3[a|n|t]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,change_of_address,street_1,street_2,city,state,zip_code,election_state,election_district,report_code,election_code,election_date,state_of_election,coverage_from_date,coverage_through_date,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,date_signed,col_a_total_contributions_no_loans,col_a_total_contributions_refunds,col_a_net_contributions,col_a_total_operating_expenditures,col_a_total_offset_to_operating_expenditures,col_a_net_operating_expenditures,col_a_cash_on_hand_close_of_period,col_a_debts_to,col_a_debts_by,col_a_individual_contributions_itemized,col_a_individual_contributions_unitemized,col_a_total_individual_contributions,col_a_political_party_contributions,col_a_pac_contributions,col_a_candidate_contributions,col_a_total_contributions,col_a_transfers_from_authorized,col_a_candidate_loans,col_a_other_loans,col_a_total_loans,col_a_offset_to_operating_expenditures,col_a_other_receipts,col_a_total_receipts,col_a_operating_expenditures,col_a_transfers_to_authorized,col_a_candidate_loan_repayments,col_a_other_loan_repayments,col_a_total_loan_repayments,col_a_refunds_to_individuals,col_a_refunds_to_party_committees,col_a_refunds_to_other_committees,col_a_total_refunds,col_a_other_disbursements,col_a_total_disbursements,col_a_cash_beginning_reporting_period,col_a_total_receipts_period,col_a_subtotals,col_a_total_disbursements_period,col_a_cash_on_hand_close,col_b_total_contributions_no_loans,col_b_total_contributions_refunds,col_b_net_contributions,col_b_total_operating_expenditures,col_b_total_offset_to_operating_expenditures,col_b_net_operating_expenditures,col_b_individual_contributions_itemized,col_b_individual_contributions_unitemized,col_b_total_individual_contributions,col_b_political_party_contributions,col_b_pac_contributions,col_b_candidate_contributions,col_b_total_contributions,col_b_transfers_from_authorized,col_b_candidate_loans,col_b_other_loans,col_b_total_loans,col_b_offset_to_operating_expenditures,col_b_other_receipts,col_b_total_receipts,col_b_operating_expenditures,col_b_transfers_to_authorized,col_b_candidate_loan_repayments,col_b_other_loan_repayments,col_b_total_loan_repayments,col_b_refunds_to_individuals,col_b_refunds_to_party_committees,col_b_refunds_to_other_committees,col_b_total_refunds,col_b_other_disbursements,col_b_total_disbursements\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^6.3|6.2|6.1\0" as *const u8 as *const libc::c_char,
        b"^f3[a|n|t]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,change_of_address,street_1,street_2,city,state,zip_code,election_state,election_district,report_code,election_code,election_date,state_of_election,coverage_from_date,coverage_through_date,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,date_signed,candidate_id_number,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix,report_type,col_a_total_contributions_no_loans,col_a_total_contributions_refunds,col_a_net_contributions,col_a_total_operating_expenditures,col_a_total_offset_to_operating_expenditures,col_a_net_operating_expenditures,col_a_cash_on_hand_close_of_period,col_a_debts_to,col_a_debts_by,col_a_individual_contributions_itemized,col_a_individual_contributions_unitemized,col_a_total_individual_contributions,col_a_political_party_contributions,col_a_pac_contributions,col_a_candidate_contributions,col_a_total_contributions,col_a_transfers_from_authorized,col_a_candidate_loans,col_a_other_loans,col_a_total_loans,col_a_offset_to_operating_expenditures,col_a_other_receipts,col_a_total_receipts,col_a_operating_expenditures,col_a_transfers_to_authorized,col_a_candidate_loan_repayments,col_a_other_loan_repayments,col_a_total_loan_repayments,col_a_refunds_to_individuals,col_a_refunds_to_party_committees,col_a_refunds_to_other_committees,col_a_total_refunds,col_a_other_disbursements,col_a_total_disbursements,col_a_cash_beginning_reporting_period,col_a_total_receipts_period,col_a_subtotals,col_a_total_disbursements_period,col_a_cash_on_hand_close,col_b_total_contributions_no_loans,col_b_total_contributions_refunds,col_b_net_contributions,col_b_total_operating_expenditures,col_b_total_offset_to_operating_expenditures,col_b_net_operating_expenditures,col_b_individual_contributions_itemized,col_b_individual_contributions_unitemized,col_b_total_individual_contributions,col_b_political_party_contributions,col_b_pac_contributions,col_b_candidate_contributions,col_b_total_contributions,col_b_transfers_from_authorized,col_b_candidate_loans,col_b_other_loans,col_b_total_loans,col_b_offset_to_operating_expenditures,col_b_other_receipts,col_b_total_receipts,col_b_operating_expenditures,col_b_transfers_to_authorized,col_b_candidate_loan_repayments,col_b_other_loan_repayments,col_b_total_loan_repayments,col_b_refunds_to_individuals,col_b_refunds_to_party_committees,col_b_refunds_to_other_committees,col_b_total_refunds,col_b_other_disbursements,col_b_total_disbursements,col_b_gross_receipts_authorized_primary,col_b_aggregate_personal_funds_primary,col_b_gross_receipts_minus_personal_funds_primary,col_b_gross_receipts_authorized_general,col_b_aggregate_personal_funds_general,col_b_gross_receipts_minus_personal_funds_general\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3|5.2|5.1|5.0\0" as *const u8 as *const libc::c_char,
        b"^f3[a|n|t]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,street_1,street_2,city,state,zip_code,change_of_address,election_state,election_district,report_code,election_code,election_date,state_of_election,primary_election,general_election,special_election,runoff_election,coverage_from_date,coverage_through_date,col_a_total_contributions_no_loans,col_a_total_contributions_refunds,col_a_net_contributions,col_a_total_operating_expenditures,col_a_total_offset_to_operating_expenditures,col_a_net_operating_expenditures,col_a_cash_on_hand_close_of_period,col_a_debts_to,col_a_debts_by,col_a_individual_contributions_itemized,col_a_individual_contributions_unitemized,col_a_total_individual_contributions,col_a_political_party_contributions,col_a_pac_contributions,col_a_candidate_contributions,col_a_total_contributions,col_a_transfers_from_authorized,col_a_candidate_loans,col_a_other_loans,col_a_total_loans,col_a_offset_to_operating_expenditures,col_a_other_receipts,col_a_total_receipts,col_a_operating_expenditures,col_a_transfers_to_authorized,col_a_candidate_loan_repayments,,col_a_total_loan_repayments,col_a_refunds_to_individuals,col_a_refunds_to_party_committees,col_a_refunds_to_other_committees,col_a_total_refunds,col_a_other_disbursements,col_a_total_disbursements,col_a_cash_beginning_reporting_period,col_a_total_receipts_period,col_a_subtotals,col_a_total_disbursements_period,col_a_cash_on_hand_close,col_b_total_contributions_no_loans,col_b_total_contributions_refunds,col_b_net_contributions,col_b_total_operating_expenditures,col_b_total_offset_to_operating_expenditures,col_b_net_operating_expenditures,col_b_individual_contributions_itemized,col_b_individual_contributions_unitemized,col_b_total_individual_contributions,col_b_political_party_contributions,col_b_pac_contributions,col_b_candidate_contributions,col_b_total_contributions,col_b_transfers_from_authorized,col_b_candidate_loans,col_b_other_loans,col_b_total_loans,col_b_offset_to_operating_expenditures,col_b_other_receipts,col_b_total_receipts,col_b_operating_expenditures,col_b_transfers_to_authorized,col_b_candidate_loan_repayments,,col_b_total_loan_repayments,col_b_refunds_to_individuals,,,col_b_total_refunds,col_b_other_disbursements,col_b_total_disbursements,treasurer_name,date_signed,candidate_id_number,candidate_name,report_type,col_b_gross_receipts_authorized_primary,col_b_aggregate_personal_funds_primary,col_b_gross_receipts_minus_personal_funds_primary,col_b_gross_receipts_authorized_general,col_b_aggregate_personal_funds_general,col_b_gross_receipts_minus_personal_funds_general\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^3.0|^2|^1\0" as *const u8 as *const libc::c_char,
        b"^f3[a|n|t]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,street_1,street_2,city,state,zip_code,change_of_address,election_state,election_district,report_code,election_code,election_date,state_of_election,primary_election,general_election,special_election,runoff_election,coverage_from_date,coverage_through_date,col_a_total_contributions_no_loans,col_a_total_contributions_refunds,col_a_net_contributions,col_a_total_operating_expenditures,col_a_total_offset_to_operating_expenditures,col_a_net_operating_expenditures,col_a_cash_on_hand_close_of_period,col_a_debts_to,col_a_debts_by,col_a_individual_contributions_itemized,col_a_individual_contributions_unitemized,col_a_total_individual_contributions,col_a_political_party_contributions,col_a_pac_contributions,col_a_candidate_contributions,col_a_total_contributions,col_a_transfers_from_authorized,col_a_candidate_loans,col_a_other_loans,col_a_total_loans,col_a_offset_to_operating_expenditures,col_a_other_receipts,col_a_total_receipts,col_a_operating_expenditures,col_a_transfers_to_authorized,col_a_candidate_loan_repayments,col_a_other_loan_repayments,col_a_total_loan_repayments,col_a_refunds_to_individuals,col_a_refunds_to_party_committees,col_a_refunds_to_other_committees,col_a_total_refunds,col_a_other_disbursements,col_a_total_disbursements,col_a_cash_beginning_reporting_period,col_a_total_receipts_period,col_a_subtotals,col_a_total_disbursements_period,col_a_cash_on_hand_close,col_b_total_contributions_no_loans,col_b_total_contributions_refunds,col_b_net_contributions,col_b_total_operating_expenditures,col_b_total_offset_to_operating_expenditures,col_b_net_operating_expenditures,col_b_individual_contributions_itemized,col_b_individual_contributions_unitemized,col_b_total_individual_contributions,col_b_political_party_contributions,col_b_pac_contributions,col_b_candidate_contributions,col_b_total_contributions,col_b_transfers_from_authorized,col_b_candidate_loans,col_b_other_loans,col_b_total_loans,col_b_offset_to_operating_expenditures,col_b_other_receipts,col_b_total_receipts,col_b_operating_expenditures,col_b_transfers_to_authorized,col_b_candidate_loan_repayments,col_b_other_loan_repayments,col_b_total_loan_repayments,col_b_refunds_to_individuals,col_b_refunds_to_party_committees,col_b_refunds_to_other_committees,col_b_total_refunds,col_b_other_disbursements,col_b_total_disbursements,treasurer_name,date_signed\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P(3.4|3.3|3.2)\0" as *const u8 as *const libc::c_char,
        b"^f3l[a|n]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,change_of_address,street_1,street_2,city,state,zip_code,election_state,election_district,report_code,election_date,election_state,semi_annual_period,coverage_from_date,coverage_through_date,semi_annual_period_jan_june,semi_annual_period_jul_dec,quarterly_monthly_bundled_contributions,semi_annual_bundled_contributions,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,date_signed,beginning_image_number,end_image_number,receipt_date\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P(3.1|3.0|2.6)\0" as *const u8 as *const libc::c_char,
        b"^f3l[a|n]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,change_of_address,street_1,street_2,city,state,zip_code,election_state,election_district,report_code,election_date,election_state,semi_annual_period,coverage_from_date,coverage_through_date,semi_annual_period_jan_june,semi_annual_period_jul_dec,quarterly_monthly_bundled_contributions,semi_annual_bundled_contributions,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,date_signed,beginning_image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0|7.0|6.4\0" as *const u8 as *const libc::c_char,
        b"^f3l[a|n]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,change_of_address,street_1,street_2,city,state,zip_code,election_state,election_district,report_code,election_date,,semi_annual_period,coverage_from_date,coverage_through_date,semi_annual_period_jan_june,semi_annual_period_jul_dec,quarterly_monthly_bundled_contributions,semi_annual_bundled_contributions,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,date_signed\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P3.2|^P3.3|^P3.4\0" as *const u8 as *const libc::c_char,
        b"(^f3p$)|(^f3p[^s|3])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,change_of_address,street_1,street_2,city,state,zip_code,activity_primary,activity_general,report_code,date_of_election,state_of_election,coverage_from_date,coverage_through_date,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,date_signed,col_a_cash_on_hand_beginning_period,col_a_total_receipts,col_a_subtotal,col_a_total_disbursements,col_a_cash_on_hand_close_of_period,col_a_debts_to,col_a_debts_by,col_a_expenditures_subject_to_limits,col_a_net_contributions,col_a_net_operating_expenditures,col_a_federal_funds,col_a_individuals_itemized,col_a_individuals_unitemized,col_a_individual_contribution_total,col_a_political_party_committees_receipts,col_a_other_political_committees_pacs,col_a_the_candidate,col_a_total_contributions,col_a_transfers_from_aff_other_party_cmttees,col_a_received_from_or_guaranteed_by_cand,col_a_other_loans,col_a_total_loans,col_a_operating,col_a_fundraising,col_a_legal_and_accounting,col_a_total_offsets_to_expenditures,col_a_other_receipts,col_a_total_receipts,col_a_operating_expenditures,col_a_transfers_to_other_authorized_committees,col_a_fundraising_disbursements,col_a_exempt_legal_accounting_disbursement,col_a_made_or_guaranteed_by_candidate,col_a_other_repayments,col_a_total_loan_repayments_made,col_a_individuals,col_a_political_party_committees_refunds,col_a_other_political_committees,col_a_total_contributions_refunds,col_a_other_disbursements,col_a_total_disbursements,col_a_items_on_hand_to_be_liquidated,col_a_alabama,col_a_alaska,col_a_arizona,col_a_arkansas,col_a_california,col_a_colorado,col_a_connecticut,col_a_delaware,col_a_dist_of_columbia,col_a_florida,col_a_georgia,col_a_hawaii,col_a_idaho,col_a_illinois,col_a_indiana,col_a_iowa,col_a_kansas,col_a_kentucky,col_a_louisiana,col_a_maine,col_a_maryland,col_a_massachusetts,col_a_michigan,col_a_minnesota,col_a_mississippi,col_a_missouri,col_a_montana,col_a_nebraska,col_a_nevada,col_a_new_hampshire,col_a_new_jersey,col_a_new_mexico,col_a_new_york,col_a_north_carolina,col_a_north_dakota,col_a_ohio,col_a_oklahoma,col_a_oregon,col_a_pennsylvania,col_a_rhode_island,col_a_south_carolina,col_a_south_dakota,col_a_tennessee,col_a_texas,col_a_utah,col_a_vermont,col_a_virginia,col_a_washington,col_a_west_virginia,col_a_wisconsin,col_a_wyoming,col_a_puerto_rico,col_a_guam,col_a_virgin_islands,col_a_totals,col_b_federal_funds,col_b_individuals_itemized,col_b_individuals_unitemized,col_b_individual_contribution_total,col_b_political_party_committees_receipts,col_b_other_political_committees_pacs,col_b_the_candidate,col_b_total_contributions_other_than_loans,col_b_transfers_from_aff_other_party_cmttees,col_b_received_from_or_guaranteed_by_cand,col_b_other_loans,col_b_total_loans,col_b_operating,col_b_fundraising,col_b_legal_and_accounting,col_b_total_offsets_to_operating_expenditures,col_b_other_receipts,col_b_total_receipts,col_b_operating_expenditures,col_b_transfers_to_other_authorized_committees,col_b_fundraising_disbursements,col_b_exempt_legal_accounting_disbursement,col_b_made_or_guaranteed_by_the_candidate,col_b_other_repayments,col_b_total_loan_repayments_made,col_b_individuals,col_b_political_party_committees_refunds,col_b_other_political_committees,col_b_total_contributions_refunds,col_b_other_disbursements,col_b_total_disbursements,col_b_alabama,col_b_alaska,col_b_arizona,col_b_arkansas,col_b_california,col_b_colorado,col_b_connecticut,col_b_delaware,col_b_dist_of_columbia,col_b_florida,col_b_georgia,col_b_hawaii,col_b_idaho,col_b_illinois,col_b_indiana,col_b_iowa,col_b_kansas,col_b_kentucky,col_b_louisiana,col_b_maine,col_b_maryland,col_b_massachusetts,col_b_michigan,col_b_minnesota,col_b_mississippi,col_b_missouri,col_b_montana,col_b_nebraska,col_b_nevada,col_b_new_hampshire,col_b_new_jersey,col_b_new_mexico,col_b_new_york,col_b_north_carolina,col_b_north_dakota,col_b_ohio,col_b_oklahoma,col_b_oregon,col_b_pennsylvania,col_b_rhode_island,col_b_south_carolina,col_b_south_dakota,col_b_tennessee,col_b_texas,col_b_utah,col_b_vermont,col_b_virginia,col_b_washington,col_b_west_virginia,col_b_wisconsin,col_b_wyoming,col_b_puerto_rico,col_b_guam,col_b_virgin_islands,col_b_totals,beginning_image_number,end_image_number,receipt_date\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0|7.0\0" as *const u8 as *const libc::c_char,
        b"(^f3p$)|(^f3p[^s|3])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,change_of_address,street_1,street_2,city,state,zip_code,activity_primary,activity_general,report_code,election_code,date_of_election,state_of_election,coverage_from_date,coverage_through_date,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,date_signed,col_a_cash_on_hand_beginning_period,col_a_total_receipts,col_a_subtotal,col_a_total_disbursements,col_a_cash_on_hand_close_of_period,col_a_debts_to,col_a_debts_by,col_a_expenditures_subject_to_limits,col_a_net_contributions,col_a_net_operating_expenditures,col_a_federal_funds,col_a_individuals_itemized,col_a_individuals_unitemized,col_a_individual_contribution_total,col_a_political_party_committees_receipts,col_a_other_political_committees_pacs,col_a_the_candidate,col_a_total_contributions,col_a_transfers_from_aff_other_party_cmttees,col_a_received_from_or_guaranteed_by_cand,col_a_other_loans,col_a_total_loans,col_a_operating,col_a_fundraising,col_a_legal_and_accounting,col_a_total_offsets_to_expenditures,col_a_other_receipts,col_a_total_receipts,col_a_operating_expenditures,col_a_transfers_to_other_authorized_committees,col_a_fundraising_disbursements,col_a_exempt_legal_accounting_disbursement,col_a_made_or_guaranteed_by_candidate,col_a_other_repayments,col_a_total_loan_repayments_made,col_a_individuals,col_a_political_party_committees_refunds,col_a_other_political_committees,col_a_total_contributions_refunds,col_a_other_disbursements,col_a_total_disbursements,col_a_items_on_hand_to_be_liquidated,col_a_alabama,col_a_alaska,col_a_arizona,col_a_arkansas,col_a_california,col_a_colorado,col_a_connecticut,col_a_delaware,col_a_dist_of_columbia,col_a_florida,col_a_georgia,col_a_hawaii,col_a_idaho,col_a_illinois,col_a_indiana,col_a_iowa,col_a_kansas,col_a_kentucky,col_a_louisiana,col_a_maine,col_a_maryland,col_a_massachusetts,col_a_michigan,col_a_minnesota,col_a_mississippi,col_a_missouri,col_a_montana,col_a_nebraska,col_a_nevada,col_a_new_hampshire,col_a_new_jersey,col_a_new_mexico,col_a_new_york,col_a_north_carolina,col_a_north_dakota,col_a_ohio,col_a_oklahoma,col_a_oregon,col_a_pennsylvania,col_a_rhode_island,col_a_south_carolina,col_a_south_dakota,col_a_tennessee,col_a_texas,col_a_utah,col_a_vermont,col_a_virginia,col_a_washington,col_a_west_virginia,col_a_wisconsin,col_a_wyoming,col_a_puerto_rico,col_a_guam,col_a_virgin_islands,col_a_totals,col_b_federal_funds,col_b_individuals_itemized,col_b_individuals_unitemized,col_b_individual_contribution_total,col_b_political_party_committees_receipts,col_b_other_political_committees_pacs,col_b_the_candidate,col_b_total_contributions_other_than_loans,col_b_transfers_from_aff_other_party_cmttees,col_b_received_from_or_guaranteed_by_cand,col_b_other_loans,col_b_total_loans,col_b_operating,col_b_fundraising,col_b_legal_and_accounting,col_b_total_offsets_to_operating_expenditures,col_b_other_receipts,col_b_total_receipts,col_b_operating_expenditures,col_b_transfers_to_other_authorized_committees,col_b_fundraising_disbursements,col_b_exempt_legal_accounting_disbursement,col_b_made_or_guaranteed_by_the_candidate,col_b_other_repayments,col_b_total_loan_repayments_made,col_b_individuals,col_b_political_party_committees_refunds,col_b_other_political_committees,col_b_total_contributions_refunds,col_b_other_disbursements,col_b_total_disbursements,col_b_alabama,col_b_alaska,col_b_arizona,col_b_arkansas,col_b_california,col_b_colorado,col_b_connecticut,col_b_delaware,col_b_dist_of_columbia,col_b_florida,col_b_georgia,col_b_hawaii,col_b_idaho,col_b_illinois,col_b_indiana,col_b_iowa,col_b_kansas,col_b_kentucky,col_b_louisiana,col_b_maine,col_b_maryland,col_b_massachusetts,col_b_michigan,col_b_minnesota,col_b_mississippi,col_b_missouri,col_b_montana,col_b_nebraska,col_b_nevada,col_b_new_hampshire,col_b_new_jersey,col_b_new_mexico,col_b_new_york,col_b_north_carolina,col_b_north_dakota,col_b_ohio,col_b_oklahoma,col_b_oregon,col_b_pennsylvania,col_b_rhode_island,col_b_south_carolina,col_b_south_dakota,col_b_tennessee,col_b_texas,col_b_utah,col_b_vermont,col_b_virginia,col_b_washington,col_b_west_virginia,col_b_wisconsin,col_b_wyoming,col_b_puerto_rico,col_b_guam,col_b_virgin_islands,col_b_totals\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^6.4|6.3|6.2|6.1\0" as *const u8 as *const libc::c_char,
        b"(^f3p$)|(^f3p[^s|3])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,change_of_address,street_1,street_2,city,state,zip_code,activity_primary,activity_general,report_code,election_code,date_of_election,state_of_election,coverage_from_date,coverage_through_date,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,date_signed,col_a_cash_on_hand_beginning_period,col_a_total_receipts,col_a_subtotal,col_a_total_disbursements,col_a_cash_on_hand_close_of_period,col_a_debts_to,col_a_debts_by,col_a_expenditures_subject_to_limits,col_a_net_contributions,col_a_net_operating_expenditures,col_a_federal_funds,col_a_individual_contribution_total,col_a_political_party_committees_receipts,col_a_other_political_committees_pacs,col_a_the_candidate,col_a_total_contributions,col_a_transfers_from_aff_other_party_cmttees,col_a_received_from_or_guaranteed_by_cand,col_a_other_loans,col_a_total_loans,col_a_operating,col_a_fundraising,col_a_legal_and_accounting,col_a_total_offsets_to_expenditures,col_a_other_receipts,col_a_total_receipts,col_a_operating_expenditures,col_a_transfers_to_other_authorized_committees,col_a_fundraising_disbursements,col_a_exempt_legal_accounting_disbursement,col_a_made_or_guaranteed_by_candidate,col_a_other_repayments,col_a_total_loan_repayments_made,col_a_individuals,col_a_political_party_committees_refunds,col_a_other_political_committees,col_a_total_contributions_refunds,col_a_other_disbursements,col_a_total_disbursements,col_a_items_on_hand_to_be_liquidated,col_a_alabama,col_a_alaska,col_a_arizona,col_a_arkansas,col_a_california,col_a_colorado,col_a_connecticut,col_a_delaware,col_a_dist_of_columbia,col_a_florida,col_a_georgia,col_a_hawaii,col_a_idaho,col_a_illinois,col_a_indiana,col_a_iowa,col_a_kansas,col_a_kentucky,col_a_louisiana,col_a_maine,col_a_maryland,col_a_massachusetts,col_a_michigan,col_a_minnesota,col_a_mississippi,col_a_missouri,col_a_montana,col_a_nebraska,col_a_nevada,col_a_new_hampshire,col_a_new_jersey,col_a_new_mexico,col_a_new_york,col_a_north_carolina,col_a_north_dakota,col_a_ohio,col_a_oklahoma,col_a_oregon,col_a_pennsylvania,col_a_rhode_island,col_a_south_carolina,col_a_south_dakota,col_a_tennessee,col_a_texas,col_a_utah,col_a_vermont,col_a_virginia,col_a_washington,col_a_west_virginia,col_a_wisconsin,col_a_wyoming,col_a_puerto_rico,col_a_guam,col_a_virgin_islands,col_a_totals,col_b_federal_funds,col_b_individual_contribution_total,col_b_political_party_committees_receipts,col_b_other_political_committees_pacs,col_b_the_candidate,col_b_total_contributions_other_than_loans,col_b_transfers_from_aff_other_party_cmttees,col_b_received_from_or_guaranteed_by_cand,col_b_other_loans,col_b_total_loans,col_b_operating,col_b_fundraising,col_b_legal_and_accounting,col_b_total_offsets_to_operating_expenditures,col_b_other_receipts,col_b_total_receipts,col_b_operating_expenditures,col_b_transfers_to_other_authorized_committees,col_b_fundraising_disbursements,col_b_exempt_legal_accounting_disbursement,col_b_made_or_guaranteed_by_the_candidate,col_b_other_repayments,col_b_total_loan_repayments_made,col_b_individuals,col_b_political_party_committees_refunds,col_b_other_political_committees,col_b_total_contributions_refunds,col_b_other_disbursements,col_b_total_disbursements,col_b_alabama,col_b_alaska,col_b_arizona,col_b_arkansas,col_b_california,col_b_colorado,col_b_connecticut,col_b_delaware,col_b_dist_of_columbia,col_b_florida,col_b_georgia,col_b_hawaii,col_b_idaho,col_b_illinois,col_b_indiana,col_b_iowa,col_b_kansas,col_b_kentucky,col_b_louisiana,col_b_maine,col_b_maryland,col_b_massachusetts,col_b_michigan,col_b_minnesota,col_b_mississippi,col_b_missouri,col_b_montana,col_b_nebraska,col_b_nevada,col_b_new_hampshire,col_b_new_jersey,col_b_new_mexico,col_b_new_york,col_b_north_carolina,col_b_north_dakota,col_b_ohio,col_b_oklahoma,col_b_oregon,col_b_pennsylvania,col_b_rhode_island,col_b_south_carolina,col_b_south_dakota,col_b_tennessee,col_b_texas,col_b_utah,col_b_vermont,col_b_virginia,col_b_washington,col_b_west_virginia,col_b_wisconsin,col_b_wyoming,col_b_puerto_rico,col_b_guam,col_b_virgin_islands,col_b_totals\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3|5.2\0" as *const u8 as *const libc::c_char,
        b"(^f3p$)|(^f3p[^s|3])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,street_1,street_2,city,state,zip_code,change_of_address,activity_primary,activity_general,report_code,election_code,date_of_election,state_of_election,coverage_from_date,coverage_through_date,col_a_cash_on_hand_beginning_period,col_a_total_receipts,col_a_subtotal,col_a_total_disbursements,col_a_cash_on_hand_close_of_period,col_a_debts_to,col_a_debts_by,col_a_expenditures_subject_to_limits,col_a_net_contributions,col_a_net_operating_expenditures,col_a_federal_funds,col_a_individual_contribution_total,col_a_political_party_committees_receipts,col_a_other_political_committees_pacs,col_a_the_candidate,col_a_total_contributions,col_a_transfers_from_aff_other_party_cmttees,col_a_received_from_or_guaranteed_by_cand,col_a_other_loans,col_a_total_loans,col_a_operating,col_a_fundraising,col_a_legal_and_accounting,col_a_total_offsets_to_expenditures,col_a_other_receipts,col_a_total_receipts,col_a_operating_expenditures,col_a_transfers_to_other_authorized_committees,col_a_fundraising_disbursements,col_a_exempt_legal_accounting_disbursement,col_a_made_or_guaranteed_by_candidate,col_a_other_repayments,col_a_total_loan_repayments_made,col_a_individuals,col_a_political_party_committees_refunds,col_a_other_political_committees,col_a_total_contributions_refunds,col_a_other_disbursements,col_a_total_disbursements,col_a_items_on_hand_to_be_liquidated,col_a_alabama,col_a_alaska,col_a_arizona,col_a_arkansas,col_a_california,col_a_colorado,col_a_connecticut,col_a_delaware,col_a_dist_of_columbia,col_a_florida,col_a_georgia,col_a_hawaii,col_a_idaho,col_a_illinois,col_a_indiana,col_a_iowa,col_a_kansas,col_a_kentucky,col_a_louisiana,col_a_maine,col_a_maryland,col_a_massachusetts,col_a_michigan,col_a_minnesota,col_a_mississippi,col_a_missouri,col_a_montana,col_a_nebraska,col_a_nevada,col_a_new_hampshire,col_a_new_jersey,col_a_new_mexico,col_a_new_york,col_a_north_carolina,col_a_north_dakota,col_a_ohio,col_a_oklahoma,col_a_oregon,col_a_pennsylvania,col_a_rhode_island,col_a_south_carolina,col_a_south_dakota,col_a_tennessee,col_a_texas,col_a_utah,col_a_vermont,col_a_virginia,col_a_washington,col_a_west_virginia,col_a_wisconsin,col_a_wyoming,col_a_puerto_rico,col_a_guam,col_a_virgin_islands,col_a_totals,col_b_federal_funds,col_b_individual_contribution_total,col_b_political_party_committees_receipts,col_b_other_political_committees_pacs,col_b_the_candidate,col_b_total_contributions_other_than_loans,col_b_transfers_from_aff_other_party_cmttees,col_b_received_from_or_guaranteed_by_cand,col_b_other_loans,col_b_total_loans,col_b_operating,col_b_fundraising,col_b_legal_and_accounting,col_b_total_offsets_to_operating_expenditures,col_b_other_receipts,col_b_total_receipts,col_b_operating_expenditures,col_b_transfers_to_other_authorized_committees,col_b_fundraising_disbursements,col_b_exempt_legal_accounting_disbursement,col_b_made_or_guaranteed_by_the_candidate,col_b_other_repayments,col_b_total_loan_repayments_made,col_b_individuals,col_b_political_party_committees_refunds,col_b_other_political_committees,col_b_total_contributions_refunds,col_b_other_disbursements,col_b_total_disbursements,col_b_alabama,col_b_alaska,col_b_arizona,col_b_arkansas,col_b_california,col_b_colorado,col_b_connecticut,col_b_delaware,col_b_dist_of_columbia,col_b_florida,col_b_georgia,col_b_hawaii,col_b_idaho,col_b_illinois,col_b_indiana,col_b_iowa,col_b_kansas,col_b_kentucky,col_b_louisiana,col_b_maine,col_b_maryland,col_b_massachusetts,col_b_michigan,col_b_minnesota,col_b_mississippi,col_b_missouri,col_b_montana,col_b_nebraska,col_b_nevada,col_b_new_hampshire,col_b_new_jersey,col_b_new_mexico,col_b_new_york,col_b_north_carolina,col_b_north_dakota,col_b_ohio,col_b_oklahoma,col_b_oregon,col_b_pennsylvania,col_b_rhode_island,col_b_south_carolina,col_b_south_dakota,col_b_tennessee,col_b_texas,col_b_utah,col_b_vermont,col_b_virginia,col_b_washington,col_b_west_virginia,col_b_wisconsin,col_b_wyoming,col_b_puerto_rico,col_b_guam,col_b_virgin_islands,col_b_totals,treasurer_name,date_signed\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(5.1|5.0|3|2|1)\0" as *const u8 as *const libc::c_char,
        b"(^f3p$)|(^f3p[^s|3])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,street_1,street_2,city,state,zip_code,change_of_address,activity_primary,activity_general,report_code,election_code,date_of_election,state_of_election,coverage_from_date,coverage_through_date,col_a_cash_on_hand_beginning_period,col_a_total_receipts,col_a_subtotal,col_a_total_disbursements,col_a_cash_on_hand_close_of_period,col_a_debts_to,col_a_debts_by,col_a_expenditures_subject_to_limits,col_a_net_contributions,col_a_net_operating_expenditures,col_a_federal_funds,col_a_individual_contribution_total,col_a_political_party_committees_receipts,col_a_other_political_committees_pacs,col_a_the_candidate,col_a_total_contributions,col_a_transfers_from_aff_other_party_cmttees,col_a_received_from_or_guaranteed_by_cand,col_a_other_loans,col_a_total_loans,col_a_operating,col_a_fundraising,col_a_legal_and_accounting,col_a_total_offsets_to_expenditures,col_a_other_receipts,col_a_total_receipts,col_a_operating_expenditures,col_a_transfers_to_other_authorized_committees,col_a_fundraising_disbursements,col_a_exempt_legal_accounting_disbursement,col_a_made_or_guaranteed_by_candidate,col_a_other_repayments,col_a_total_loan_repayments_made,col_a_individuals,col_a_political_party_committees_refunds,col_a_other_political_committees,col_a_total_contributions_refunds,col_a_other_disbursements,col_a_total_disbursements,col_a_items_on_hand_to_be_liquidated,col_a_alabama,col_a_alaska,col_a_arizona,col_a_arkansas,col_a_california,col_a_colorado,col_a_connecticut,col_a_delaware,col_a_dist_of_columbia,col_a_florida,col_a_georgia,col_a_hawaii,col_a_idaho,col_a_illinois,col_a_indiana,col_a_iowa,col_a_kansas,col_a_kentucky,col_a_louisiana,col_a_maine,col_a_maryland,col_a_massachusetts,col_a_michigan,col_a_minnesota,col_a_mississippi,col_a_missouri,col_a_montana,col_a_nebraska,col_a_nevada,col_a_new_hampshire,col_a_new_jersey,col_a_new_mexico,col_a_new_york,col_a_north_carolina,col_a_north_dakota,col_a_ohio,col_a_oklahoma,col_a_oregon,col_a_pennsylvania,col_a_rhode_island,col_a_south_carolina,col_a_south_dakota,col_a_tennessee,col_a_texas,col_a_utah,col_a_vermont,col_a_virginia,col_a_washington,col_a_west_virginia,col_a_wisconsin,col_a_wyoming,col_a_puerto_rico,col_a_guam,col_a_virgin_islands,col_a_totals,col_b_federal_funds,col_b_individual_contribution_total,col_b_political_party_committees_receipts,col_b_other_political_committees_pacs,col_b_the_candidate,col_b_total_contributions_other_than_loans,col_b_transfers_from_aff_other_party_cmttees,col_b_received_from_or_guaranteed_by_cand,col_b_other_loans,col_b_total_loans,col_b_operating,col_b_fundraising,col_b_legal_and_accounting,col_b_total_offsets_to_operating_expenditures,col_b_other_receipts,col_b_total_receipts,col_b_operating_expenditures,col_b_transfers_to_other_authorized_committees,col_b_fundraising_disbursements,col_b_exempt_legal_accounting_disbursement,col_b_made_or_guaranteed_by_the_candidate,col_b_other_repayments,col_b_total_loan_repayments_made,col_b_individuals,col_b_political_party_committees_refunds,col_b_other_political_committees,col_b_total_contributions_refunds,col_b_other_disbursements,col_b_total_disbursements,col_b_alabama,col_b_alaska,col_b_arizona,col_b_arkansas,col_b_california,col_b_colorado,col_b_connecticut,col_b_delaware,col_b_dist_of_columbia,col_b_florida,col_b_georgia,col_b_hawaii,col_b_idaho,col_b_illinois,col_b_indiana,col_b_iowa,col_b_kansas,col_b_kentucky,col_b_louisiana,col_b_maine,col_b_maryland,col_b_massachusetts,col_b_michigan,col_b_minnesota,col_b_mississippi,col_b_missouri,col_b_montana,col_b_nebraska,col_b_nevada,col_b_new_hampshire,col_b_new_jersey,col_b_new_mexico,col_b_new_york,col_b_north_carolina,col_b_north_dakota,col_b_ohio,col_b_oklahoma,col_b_oregon,col_b_pennsylvania,col_b_rhode_island,col_b_south_carolina,col_b_south_dakota,col_b_tennessee,col_b_texas,col_b_utah,col_b_vermont,col_b_virginia,col_b_washington,col_b_west_virginia,col_b_wisconsin,col_b_wyoming,col_b_puerto_rico,col_b_guam,col_b_virgin_islands,col_b_totals,treasurer_name,date_signed\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0|7.0|6.4|6.3|6.2|6.1\0" as *const u8
            as *const libc::c_char,
        b"^f3p31\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id_number,entity_type,contributor_organization_name,contributor_last_name,contributor_first_name,contributor_middle_name,contributor_prefix,contributor_suffix,contributor_street_1,contributor_street_2,contributor_city,contributor_state,contributor_zip_code,election_code,item_description,item_contribution_aquired_date,item_fair_market_value,contributor_employer,contributor_occupation,memo_code,memo_text_description\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3\0" as *const u8 as *const libc::c_char,
        b"^f3p31\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,contributor_name,contributor_street_1,contributor_street_2,contributor_city,contributor_state,contributor_zip_code,election_code,contributor_employer,contributor_occupation,item_contribution_aquired_date,item_fair_market_value,transaction_code,transaction_description,fec_committee_id_number,fec_candidate_id_number,candidate_name,candidate_office,candidate_state,candidate_district,conduit_name,conduit_street_1,conduit_street_2,conduit_city,conduit_state,conduit_zip_code,memo_code,memo_text_description,,transaction_id_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.2|5.1|5.0|^3\0" as *const u8 as *const libc::c_char,
        b"^f3p31\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,contributor_name,contributor_street_1,contributor_street_2,contributor_city,contributor_state,contributor_zip_code,election_code,contributor_employer,contributor_occupation,item_contribution_aquired_date,item_fair_market_value,transaction_code,transaction_description,fec_committee_id_number,fec_candidate_id_number,candidate_name,candidate_office,candidate_state,candidate_district,conduit_name,conduit_street_1,conduit_street_2,conduit_city,conduit_state,conduit_zip_code,memo_code,memo_text_description,,transaction_id_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0|7.0\0" as *const u8 as *const libc::c_char,
        b"^f3ps\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,date_general_election,date_day_after_general_election,net_contributions,net_expenditures,federal_funds,a_i_individuals_itemized,a_ii_individuals_unitemized,a_iii_individual_contribution_total,b_political_party_committees,c_other_political_committees_pacs,d_the_candidate,e_total_contributions_other_than_loans,transfers_from_aff_other_party_committees,a_received_from_or_guaranteed_by_candidate,b_other_loans,c_total_loans,a_operating,b_fundraising,c_legal_and_accounting,d_total_offsets_to_operating_expenditures,other_receipts,total_receipts,operating_expenditures,transfers_to_other_authorized_committees,fundraising_disbursements,exempt_legal_and_accounting_disbursements,a_made_or_guaranteed_by_the_candidate,b_other_repayments,c_total_loan_repayments_made,a_individuals,b_political_party_committees,c_other_political_committees,d_total_contributions_refunds,other_disbursements,total_disbursements,alabama,alaska,arizona,arkansas,california,colorado,connecticut,delaware,dist_of_columbia,florida,georgia,hawaii,idaho,illinois,indiana,iowa,kansas,kentucky,louisiana,maine,maryland,massachusetts,michigan,minnesota,mississippi,missouri,montana,nebraska,nevada,new_hampshire,new_jersey,new_mexico,new_york,north_carolina,north_dakota,ohio,oklahoma,oregon,pennsylvania,rhode_island,south_carolina,south_dakota,tennessee,texas,utah,vermont,virginia,washington,west_virginia,wisconsin,wyoming,puerto_rico,guam,virgin_islands,totals\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^6.4|6.3|6.2|6.1\0" as *const u8 as *const libc::c_char,
        b"^f3ps\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,date_general_election,date_day_after_general_election,net_contributions,net_expenditures,federal_funds,a_individuals,b_political_party_committees,c_other_political_committees_pacs,d_the_candidate,e_total_contributions_other_than_loans,transfers_from_aff_other_party_committees,a_received_from_or_guaranteed_by_candidate,b_other_loans,c_total_loans,a_operating,b_fundraising,c_legal_and_accounting,d_total_offsets_to_operating_expenditures,other_receipts,total_receipts,operating_expenditures,transfers_to_other_authorized_committees,fundraising_disbursements,exempt_legal_and_accounting_disbursements,a_made_or_guaranteed_by_the_candidate,b_other_repayments,c_total_loan_repayments_made,a_individuals,b_political_party_committees,c_other_political_committees,d_total_contributions_refunds,other_disbursements,total_disbursements,alabama,alaska,arizona,arkansas,california,colorado,connecticut,delaware,dist_of_columbia,florida,georgia,hawaii,idaho,illinois,indiana,iowa,kansas,kentucky,louisiana,maine,maryland,massachusetts,michigan,minnesota,mississippi,missouri,montana,nebraska,nevada,new_hampshire,new_jersey,new_mexico,new_york,north_carolina,north_dakota,ohio,oklahoma,oregon,pennsylvania,rhode_island,south_carolina,south_dakota,tennessee,texas,utah,vermont,virginia,washington,west_virginia,wisconsin,wyoming,puerto_rico,guam,virgin_islands,totals\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3|5.2|5.1|5.0|^3\0" as *const u8 as *const libc::c_char,
        b"^f3ps\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,net_contributions,net_expenditures,federal_funds,a_individuals,b_political_party_committees,c_other_political_committees_pacs,d_the_candidate,e_total_contributions_other_than_loans,transfers_from_aff_other_party_committees,a_received_from_or_guaranteed_by_candidate,b_other_loans,c_total_loans,a_operating,b_fundraising,c_legal_and_accounting,d_total_offsets_to_operating_expenditures,other_receipts,total_receipts,operating_expenditures,transfers_to_other_authorized_committees,fundraising_disbursements,exempt_legal_and_accounting_disbursements,a_made_or_guaranteed_by_the_candidate,b_other_repayments,c_total_loan_repayments_made,a_individuals,b_political_party_committees,c_other_political_committees,d_total_contributions_refunds,other_disbursements,total_disbursements,alabama,alaska,arizona,arkansas,california,colorado,connecticut,delaware,dist_of_columbia,florida,georgia,hawaii,idaho,illinois,indiana,iowa,kansas,kentucky,louisiana,maine,maryland,massachusetts,michigan,minnesota,mississippi,missouri,montana,nebraska,nevada,new_hampshire,new_jersey,new_mexico,new_york,north_carolina,north_dakota,ohio,oklahoma,oregon,pennsylvania,rhode_island,south_carolina,south_dakota,tennessee,texas,utah,vermont,virginia,washington,west_virginia,wisconsin,wyoming,puerto_rico,guam,virgin_islands,totals,date_general_election,date_day_after_general_election\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P3|^P2\0" as *const u8 as *const libc::c_char,
        b"^f3s\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,date_general_election,date_day_after_general_election,a_i_individuals_itemized,a_ii_individuals_unitemized,a_iii_individuals_total,b_political_party_committees,c_all_other_political_committees_pacs,d_the_candidate,e_total_contributions,transfers_from_other_auth_committees,a_loans_made_or_guarn_by_the_candidate,b_all_other_loans,c_total_loans,offsets_to_operating_expenditures,other_receipts,total_receipts,operating_expenditures,transfers_to_other_auth_committees,a_loan_repayment_by_candidate,b_loan_repayments_all_other_loans,c_total_loan_repayments,a_refund_individuals_other_than_pol_cmtes,b_refund_political_party_committees,c_refund_other_political_committees,d_total_contributions_refunds,other_disbursements,total_disbursements,a_total_contributions_no_loans,c_net_operating_expenditures,beginning_image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P1\0" as *const u8 as *const libc::c_char,
        b"^f3s\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,date_general_election,date_day_after_general_election,a_iii_individuals_total,b_political_party_committees,c_all_other_political_committees_pacs,d_the_candidate,e_total_contributions,transfers_from_other_auth_committees,a_loans_made_or_guarn_by_the_candidate,b_all_other_loans,c_total_loans,offsets_to_operating_expenditures,other_receipts,total_receipts,operating_expenditures,transfers_to_other_auth_committees,a_loan_repayment_by_candidate,b_loan_repayments_all_other_loans,c_total_loan_repayments,a_refund_individuals_other_than_pol_cmtes,b_refund_political_party_committees,c_refund_other_political_committees,d_total_contributions_refunds,other_disbursements,total_disbursements,beginning_image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0|7.0|6.4|6.3|6.2|6.1\0" as *const u8
            as *const libc::c_char,
        b"^f3s\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,date_general_election,date_day_after_general_election,a_total_contributions_no_loans,b_total_contribution_refunds,c_net_contributions,a_total_operating_expenditures,b_total_offsets_to_operating_expenditures,c_net_operating_expenditures,a_i_individuals_itemized,a_ii_individuals_unitemized,a_iii_individuals_total,b_political_party_committees,c_all_other_political_committees_pacs,d_the_candidate,e_total_contributions,transfers_from_other_auth_committees,a_loans_made_or_guarn_by_the_candidate,b_all_other_loans,c_total_loans,offsets_to_operating_expenditures,other_receipts,total_receipts,operating_expenditures,transfers_to_other_auth_committees,a_loan_repayment_by_candidate,b_loan_repayments_all_other_loans,c_total_loan_repayments,a_refund_individuals_other_than_pol_cmtes,b_refund_political_party_committees,c_refund_other_political_committees,d_total_contributions_refunds,other_disbursements,total_disbursements\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3|5.2|5.1|5.0|^3\0" as *const u8 as *const libc::c_char,
        b"^f3s\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,a_total_contributions_no_loans,b_total_contribution_refunds,c_net_contributions,a_total_operating_expenditures,b_total_offsets_to_operating_expenditures,c_net_operating_expenditures,a_i_individuals_itemized,a_ii_individuals_unitemized,a_iii_individuals_total,b_political_party_committees,c_all_other_political_committees_pacs,d_the_candidate,e_total_contributions,transfers_from_other_auth_committees,a_loans_made_or_guarn_by_the_candidate,b_all_other_loans,c_total_loans,offsets_to_operating_expenditures,other_receipts,total_receipts,operating_expenditures,transfers_to_other_auth_committees,a_loan_repayment_by_candidate,,c_total_loan_repayments,a_refund_individuals_other_than_pol_cmtes,,,d_total_contributions_refunds,other_disbursements,total_disbursements,date_general_election,date_day_after_general_election\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P3.2|^P3.3|^P3.4\0" as *const u8 as *const libc::c_char,
        b"(^f3x$)|(^f3x[ant])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,change_of_address,street_1,street_2,city,state,zip_code,report_code,date_of_election,state_of_election,coverage_from_date,coverage_through_date,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,date_signed,col_a_cash_on_hand_beginning_period,col_a_total_receipts,col_a_subtotal,col_a_total_disbursements,col_a_cash_on_hand_close_of_period,col_a_debts_to,col_a_debts_by,qualified_committee,col_a_individuals_itemized,col_a_individuals_unitemized,col_a_individual_contribution_total,col_a_political_party_committees,col_a_other_political_committees_pacs,col_a_total_contributions,col_a_transfers_from_aff_other_party_cmttees,col_a_total_loans,col_a_total_loan_repayments_received,col_a_offsets_to_expenditures,col_a_federal_refunds,col_a_other_federal_receipts,col_a_transfers_from_nonfederal_h3,col_a_levin_funds,col_a_total_nonfederal_transfers,col_a_total_receipts,col_a_total_federal_receipts,col_a_shared_operating_expenditures_federal,col_a_shared_operating_expenditures_nonfederal,col_a_other_federal_operating_expenditures,col_a_total_operating_expenditures,col_a_transfers_to_affiliated,col_a_contributions_to_candidates,col_a_independent_expenditures,col_a_coordinated_expenditures_by_party_committees,col_a_total_loan_repayments_made,col_a_loans_made,col_a_refunds_to_individuals,col_a_refunds_to_party_committees,col_a_refunds_to_other_committees,col_a_total_refunds,col_a_other_disbursements,col_a_federal_election_activity_federal_share,col_a_federal_election_activity_levin_share,col_a_federal_election_activity_all_federal,col_a_federal_election_activity_total,col_a_total_disbursements,col_a_total_federal_disbursements,col_a_total_contributions,col_a_total_contributions_refunds,col_a_net_contributions,col_a_total_federal_operating_expenditures,col_a_total_offsets_to_expenditures,col_a_net_operating_expenditures,col_b_cash_on_hand_jan_1,col_b_year,col_b_total_receipts,col_b_subtotal,col_b_total_disbursements,col_b_cash_on_hand_close_of_period,col_b_individuals_itemized,col_b_individuals_unitemized,col_b_individual_contribution_total,col_b_political_party_committees,col_b_other_political_committees_pacs,col_b_total_contributions,col_b_transfers_from_aff_other_party_cmttees,col_b_total_loans,col_b_total_loan_repayments_received,col_b_offsets_to_expenditures,col_b_federal_refunds,col_b_other_federal_receipts,col_b_transfers_from_nonfederal_h3,col_b_levin_funds,col_b_total_nonfederal_transfers,col_b_total_receipts,col_b_total_federal_receipts,col_b_shared_operating_expenditures_federal,col_b_shared_operating_expenditures_nonfederal,col_b_other_federal_operating_expenditures,col_b_total_operating_expenditures,col_b_transfers_to_affiliated,col_b_contributions_to_candidates,col_b_independent_expenditures,col_b_coordinated_expenditures_by_party_committees,col_b_total_loan_repayments_made,col_b_loans_made,col_b_refunds_to_individuals,col_b_refunds_to_party_committees,col_b_refunds_to_other_committees,col_b_total_refunds,col_b_other_disbursements,col_b_federal_election_activity_federal_share,col_b_federal_election_activity_levin_share,col_b_federal_election_activity_all_federal,col_b_federal_election_activity_total,col_b_total_disbursements,col_b_total_federal_disbursements,col_b_total_contributions,col_b_total_contributions_refunds,col_b_net_contributions,col_b_total_federal_operating_expenditures,col_b_total_offsets_to_expenditures,col_b_net_operating_expenditures,beginning_image_number,end_image_number,receipt_date\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P1|^P2|^P3.0|^P3.1\0" as *const u8 as *const libc::c_char,
        b"(^f3x$)|(^f3x[ant])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,change_of_address,street_1,street_2,city,state,zip_code,report_code,date_of_election,state_of_election,coverage_from_date,coverage_through_date,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,date_signed,col_a_cash_on_hand_beginning_period,col_a_total_receipts,col_a_subtotal,col_a_total_disbursements,col_a_cash_on_hand_close_of_period,col_a_debts_to,col_a_debts_by,qualified_committee,col_a_individuals_itemized,col_a_individuals_unitemized,col_a_individual_contribution_total,col_a_political_party_committees,col_a_other_political_committees_pacs,col_a_total_contributions,col_a_transfers_from_aff_other_party_cmttees,col_a_total_loans,col_a_total_loan_repayments_received,col_a_offsets_to_expenditures,col_a_federal_refunds,col_a_other_federal_receipts,col_a_transfers_from_nonfederal_h3,col_a_levin_funds,col_a_total_nonfederal_transfers,col_a_total_receipts,col_a_total_federal_receipts,col_a_shared_operating_expenditures_federal,col_a_shared_operating_expenditures_nonfederal,col_a_other_federal_operating_expenditures,col_a_total_operating_expenditures,col_a_transfers_to_affiliated,col_a_contributions_to_candidates,col_a_independent_expenditures,col_a_coordinated_expenditures_by_party_committees,col_a_total_loan_repayments_made,col_a_loans_made,col_a_refunds_to_individuals,col_a_refunds_to_party_committees,col_a_refunds_to_other_committees,col_a_total_refunds,col_a_other_disbursements,col_a_federal_election_activity_federal_share,col_a_federal_election_activity_levin_share,col_a_federal_election_activity_all_federal,col_a_federal_election_activity_total,col_a_total_disbursements,col_a_total_federal_disbursements,col_a_total_contributions,col_a_total_contributions_refunds,col_a_net_contributions,col_a_total_federal_operating_expenditures,col_a_total_offsets_to_expenditures,col_a_net_operating_expenditures,col_b_cash_on_hand_jan_1,col_b_year,col_b_total_receipts,col_b_subtotal,col_b_total_disbursements,col_b_cash_on_hand_close_of_period,col_b_individuals_itemized,col_b_individuals_unitemized,col_b_individual_contribution_total,col_b_political_party_committees,col_b_other_political_committees_pacs,col_b_total_contributions,col_b_transfers_from_aff_other_party_cmttees,col_b_total_loans,col_b_total_loan_repayments_received,col_b_offsets_to_expenditures,col_b_federal_refunds,col_b_other_federal_receipts,col_b_transfers_from_nonfederal_h3,col_b_levin_funds,col_b_total_nonfederal_transfers,col_b_total_receipts,col_b_total_federal_receipts,col_b_shared_operating_expenditures_federal,col_b_shared_operating_expenditures_nonfederal,col_b_other_federal_operating_expenditures,col_b_total_operating_expenditures,col_b_transfers_to_affiliated,col_b_contributions_to_candidates,col_b_independent_expenditures,col_b_coordinated_expenditures_by_party_committees,col_b_total_loan_repayments_made,col_b_loans_made,col_b_refunds_to_individuals,col_b_refunds_to_party_committees,col_b_refunds_to_other_committees,col_b_total_refunds,col_b_other_disbursements,col_b_federal_election_activity_federal_share,col_b_federal_election_activity_levin_share,col_b_federal_election_activity_all_federal,col_b_federal_election_activity_total,col_b_total_disbursements,col_b_total_federal_disbursements,col_b_total_contributions,col_b_total_contributions_refunds,col_b_net_contributions,col_b_total_federal_operating_expenditures,col_b_total_offsets_to_expenditures,col_b_net_operating_expenditures,beginning_image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0|7.0|6.4|6.3|6.2|6.1\0" as *const u8
            as *const libc::c_char,
        b"(^f3x$)|(^f3x[ant])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,change_of_address,street_1,street_2,city,state,zip_code,report_code,election_code,date_of_election,state_of_election,coverage_from_date,coverage_through_date,qualified_committee,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,date_signed,col_a_cash_on_hand_beginning_period,col_a_total_receipts,col_a_subtotal,col_a_total_disbursements,col_a_cash_on_hand_close_of_period,col_a_debts_to,col_a_debts_by,col_a_individuals_itemized,col_a_individuals_unitemized,col_a_individual_contribution_total,col_a_political_party_committees,col_a_other_political_committees_pacs,col_a_total_contributions,col_a_transfers_from_aff_other_party_cmttees,col_a_total_loans,col_a_total_loan_repayments_received,col_a_offsets_to_expenditures,col_a_federal_refunds,col_a_other_federal_receipts,col_a_transfers_from_nonfederal_h3,col_a_levin_funds,col_a_total_nonfederal_transfers,col_a_total_receipts,col_a_total_federal_receipts,col_a_shared_operating_expenditures_federal,col_a_shared_operating_expenditures_nonfederal,col_a_other_federal_operating_expenditures,col_a_total_operating_expenditures,col_a_transfers_to_affiliated,col_a_contributions_to_candidates,col_a_independent_expenditures,col_a_coordinated_expenditures_by_party_committees,col_a_total_loan_repayments_made,col_a_loans_made,col_a_refunds_to_individuals,col_a_refunds_to_party_committees,col_a_refunds_to_other_committees,col_a_total_refunds,col_a_other_disbursements,col_a_federal_election_activity_federal_share,col_a_federal_election_activity_levin_share,col_a_federal_election_activity_all_federal,col_a_federal_election_activity_total,col_a_total_disbursements,col_a_total_federal_disbursements,col_a_total_contributions,col_a_total_contributions_refunds,col_a_net_contributions,col_a_total_federal_operating_expenditures,col_a_total_offsets_to_expenditures,col_a_net_operating_expenditures,col_b_cash_on_hand_jan_1,col_b_year,col_b_total_receipts,col_b_subtotal,col_b_total_disbursements,col_b_cash_on_hand_close_of_period,col_b_individuals_itemized,col_b_individuals_unitemized,col_b_individual_contribution_total,col_b_political_party_committees,col_b_other_political_committees_pacs,col_b_total_contributions,col_b_transfers_from_aff_other_party_cmttees,col_b_total_loans,col_b_total_loan_repayments_received,col_b_offsets_to_expenditures,col_b_federal_refunds,col_b_other_federal_receipts,col_b_transfers_from_nonfederal_h3,col_b_levin_funds,col_b_total_nonfederal_transfers,col_b_total_receipts,col_b_total_federal_receipts,col_b_shared_operating_expenditures_federal,col_b_shared_operating_expenditures_nonfederal,col_b_other_federal_operating_expenditures,col_b_total_operating_expenditures,col_b_transfers_to_affiliated,col_b_contributions_to_candidates,col_b_independent_expenditures,col_b_coordinated_expenditures_by_party_committees,col_b_total_loan_repayments_made,col_b_loans_made,col_b_refunds_to_individuals,col_b_refunds_to_party_committees,col_b_refunds_to_other_committees,col_b_total_refunds,col_b_other_disbursements,col_b_federal_election_activity_federal_share,col_b_federal_election_activity_levin_share,col_b_federal_election_activity_all_federal,col_b_federal_election_activity_total,col_b_total_disbursements,col_b_total_federal_disbursements,col_b_total_contributions,col_b_total_contributions_refunds,col_b_net_contributions,col_b_total_federal_operating_expenditures,col_b_total_offsets_to_expenditures,col_b_net_operating_expenditures\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3|5.2|5.1|5.0\0" as *const u8 as *const libc::c_char,
        b"(^f3x$)|(^f3x[ant])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,street_1,street_2,city,state,zip_code,change_of_address,qualified_committee,report_code,election_code,date_of_election,state_of_election,coverage_from_date,coverage_through_date,col_a_cash_on_hand_beginning_period,col_a_total_receipts,col_a_subtotal,col_a_total_disbursements,col_a_cash_on_hand_close_of_period,col_a_debts_to,col_a_debts_by,col_a_individuals_itemized,col_a_individuals_unitemized,col_a_individual_contribution_total,col_a_political_party_committees,col_a_other_political_committees_pacs,col_a_total_contributions,col_a_transfers_from_aff_other_party_cmttees,col_a_total_loans,col_a_total_loan_repayments_received,col_a_offsets_to_expenditures,col_a_federal_refunds,col_a_other_federal_receipts,col_a_transfers_from_nonfederal_h3,col_a_total_receipts,col_a_total_federal_receipts,col_a_shared_operating_expenditures_federal,col_a_shared_operating_expenditures_nonfederal,col_a_other_federal_operating_expenditures,col_a_total_operating_expenditures,col_a_transfers_to_affiliated,col_a_contributions_to_candidates,col_a_independent_expenditures,col_a_coordinated_expenditures_by_party_committees,col_a_total_loan_repayments_made,col_a_loans_made,col_a_refunds_to_individuals,col_a_refunds_to_party_committees,col_a_refunds_to_other_committees,col_a_total_refunds,col_a_other_disbursements,col_a_total_disbursements,col_a_total_federal_disbursements,col_a_total_contributions,col_a_total_contributions_refunds,col_a_net_contributions,col_a_total_federal_operating_expenditures,col_a_total_offsets_to_expenditures,col_a_net_operating_expenditures,col_b_cash_on_hand_jan_1,col_b_year,col_b_total_receipts,col_b_subtotal,col_b_total_disbursements,col_b_cash_on_hand_close_of_period,col_b_individuals_itemized,col_b_individuals_unitemized,col_b_individual_contribution_total,col_b_political_party_committees,col_b_other_political_committees_pacs,col_b_total_contributions,col_b_transfers_from_aff_other_party_cmttees,col_b_total_loans,col_b_total_loan_repayments_received,col_b_offsets_to_expenditures,col_b_federal_refunds,col_b_other_federal_receipts,col_b_transfers_from_nonfederal_h3,col_b_total_receipts,col_b_total_federal_receipts,col_b_shared_operating_expenditures_federal,col_b_shared_operating_expenditures_nonfederal,col_b_other_federal_operating_expenditures,col_b_total_operating_expenditures,col_b_transfers_to_affiliated,col_b_contributions_to_candidates,col_b_independent_expenditures,col_b_coordinated_expenditures_by_party_committees,col_b_total_loan_repayments_made,col_b_loans_made,col_b_refunds_to_individuals,col_b_refunds_to_party_committees,col_b_refunds_to_other_committees,col_b_total_refunds,col_b_other_disbursements,col_b_total_disbursements,col_b_total_federal_disbursements,col_b_total_contributions,col_b_total_contributions_refunds,col_b_net_contributions,col_b_total_federal_operating_expenditures,col_b_total_offsets_to_expenditures,col_b_net_operating_expenditures,treasurer_name,date_signed,col_a_levin_funds,col_a_total_nonfederal_transfers,col_a_federal_election_activity_federal_share,col_a_federal_election_activity_levin_share,col_a_federal_election_activity_all_federal,col_a_federal_election_activity_total,col_b_levin_funds,col_b_total_nonfederal_transfers,col_b_federal_election_activity_federal_share,col_b_federal_election_activity_levin_share,col_b_federal_election_activity_all_federal,col_b_federal_election_activity_total\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^3|^2|^1\0" as *const u8 as *const libc::c_char,
        b"(^f3x$)|(^f3x[ant])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,street_1,street_2,city,state,zip_code,change_of_address,qualified_committee,report_code,election_code,date_of_election,state_of_election,coverage_from_date,coverage_through_date,col_a_cash_on_hand_beginning_period,col_a_total_receipts,col_a_subtotal,col_a_total_disbursements,col_a_cash_on_hand_close_of_period,col_a_debts_to,col_a_debts_by,col_a_individuals_itemized,col_a_individuals_unitemized,col_a_individual_contribution_total,col_a_political_party_committees,col_a_other_political_committees_pacs,col_a_total_contributions,col_a_transfers_from_aff_other_party_cmttees,col_a_total_loans,col_a_total_loan_repayments_received,col_a_offsets_to_expenditures,col_a_federal_refunds,col_a_other_federal_receipts,col_a_transfers_from_nonfederal_h3,col_a_total_receipts,col_a_total_federal_receipts,col_a_shared_operating_expenditures_federal,col_a_shared_operating_expenditures_nonfederal,col_a_other_federal_operating_expenditures,col_a_total_operating_expenditures,col_a_transfers_to_affiliated,col_a_contributions_to_candidates,col_a_independent_expenditures,col_a_coordinated_expenditures_by_party_committees,col_a_total_loan_repayments_made,col_a_loans_made,col_a_refunds_to_individuals,col_a_refunds_to_party_committees,col_a_refunds_to_other_committees,col_a_total_refunds,col_a_other_disbursements,col_a_total_disbursements,col_a_total_federal_disbursements,col_a_total_contributions,col_a_total_contributions_refunds,col_a_net_contributions,col_a_total_federal_operating_expenditures,col_a_total_offsets_to_expenditures,col_a_net_operating_expenditures,col_b_cash_on_hand_jan_1,col_b_year,col_b_total_receipts,col_b_subtotal,col_b_total_disbursements,col_b_cash_on_hand_close_of_period,col_b_individuals_itemized,col_b_individuals_unitemized,col_b_individual_contribution_total,col_b_political_party_committees,col_b_other_political_committees_pacs,col_b_total_contributions,col_b_transfers_from_aff_other_party_cmttees,col_b_total_loans,col_b_total_loan_repayments_received,col_b_offsets_to_expenditures,col_b_federal_refunds,col_b_other_federal_receipts,col_b_transfers_from_nonfederal_h3,col_b_total_receipts,col_b_total_federal_receipts,col_b_shared_operating_expenditures_federal,col_b_shared_operating_expenditures_nonfederal,col_b_other_federal_operating_expenditures,col_b_total_operating_expenditures,col_b_transfers_to_affiliated,col_b_contributions_to_candidates,col_b_independent_expenditures,col_b_coordinated_expenditures_by_party_committees,col_b_total_loan_repayments_made,col_b_loans_made,col_b_refunds_to_individuals,col_b_refunds_to_party_committees,col_b_refunds_to_other_committees,col_b_total_refunds,col_b_other_disbursements,col_b_total_disbursements,col_b_total_federal_disbursements,col_b_total_contributions,col_b_total_contributions_refunds,col_b_net_contributions,col_b_total_federal_operating_expenditures,col_b_total_offsets_to_expenditures,col_b_net_operating_expenditures,treasurer_name,date_signed\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(P3|P2)\0" as *const u8 as *const libc::c_char,
        b"(^f3z$)|(^f3z[t])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,principal_committee_name,coverage_from_date,coverage_through_date,authorized_committee_name,col_a_individual_contributions_itemized,col_a_political_party_contributions,col_a_pac_contributions,col_a_candidate_contributions,col_a_total_contributions,col_a_transfers_from_authorized,col_a_candidate_loans,col_a_other_loans,col_a_total_loans,col_a_offset_to_operating_expenditures,col_a_other_receipts,col_a_total_receipts,col_a_operating_expenditures,col_a_transfers_to_authorized,col_a_candidate_loan_repayments,col_a_other_loan_repayments,col_a_total_loan_repayments,col_a_refunds_to_individuals,col_a_refunds_to_party_committees,col_a_refunds_to_other_committees,col_a_total_refunds,col_a_other_disbursements,col_a_total_disbursements,col_a_cash_beginning_reporting_period,col_a_cash_on_hand_close,col_a_debts_to,col_a_debts_by,col_a_net_contributions,col_a_net_operating_expenditures,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P1\0" as *const u8 as *const libc::c_char,
        b"(^f3z$)|(^f3z[t])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,principal_committee_name,coverage_from_date,coverage_through_date,col_a_individual_contributions_itemized,col_a_political_party_contributions,col_a_pac_contributions,col_a_candidate_contributions,col_a_total_contributions,col_a_transfers_from_authorized,col_a_candidate_loans,col_a_other_loans,col_a_total_loans,col_a_offset_to_operating_expenditures,col_a_other_receipts,col_a_total_receipts,col_a_operating_expenditures,col_a_transfers_to_authorized,col_a_candidate_loan_repayments,col_a_other_loan_repayments,col_a_total_loan_repayments,col_a_refunds_to_individuals,col_a_refunds_to_party_committees,col_a_refunds_to_other_committees,col_a_total_refunds,col_a_other_disbursements,col_a_total_disbursements,col_a_cash_beginning_reporting_period,col_a_cash_on_hand_close,col_a_debts_to,col_a_debts_by,col_a_net_contributions,col_a_net_operating_expenditures,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(8.1|8.0|7.0|6|5|3|2|1)\0" as *const u8 as *const libc::c_char,
        b"(^f3z$)|(^f3z[t])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,principal_committee_name,coverage_from_date,coverage_through_date,authorized_committee_id_number,authorized_committee_name,col_a_individual_contributions_itemized,col_a_political_party_contributions,col_a_pac_contributions,col_a_candidate_contributions,col_a_total_contributions,col_a_transfers_from_authorized,col_a_candidate_loans,col_a_other_loans,col_a_total_loans,col_a_offset_to_operating_expenditures,col_a_other_receipts,col_a_total_receipts,col_a_operating_expenditures,col_a_transfers_to_authorized,col_a_candidate_loan_repayments,col_a_other_loan_repayments,col_a_total_loan_repayments,col_a_refunds_to_individuals,col_a_refunds_to_party_committees,col_a_refunds_to_other_committees,col_a_total_refunds,col_a_other_disbursements,col_a_total_disbursements,col_a_cash_beginning_reporting_period,col_a_cash_on_hand_close,col_a_debts_to,col_a_debts_by,col_a_net_contributions,col_a_net_operating_expenditures\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(P3.4|8.3|8.2)\0" as *const u8 as *const libc::c_char,
        b"^f3z1\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,principal_committee_name,coverage_from_date,coverage_through_date,authorized_committee_id_number,authorized_committee_name,col_a_net_contributions,col_a_net_operating_expenditures,col_a_debts_to,col_a_debts_by,col_a_individual_contributions,col_a_political_party_contributions,col_a_pac_contributions,col_a_candidate_contributions,col_a_total_contributions,col_a_transfers_from_authorized,col_a_candidate_loans,col_a_other_loans,col_a_total_loans,col_a_offset_to_operating_expenditures,col_a_other_receipts,col_a_total_receipts,col_a_operating_expenditures,col_a_transfers_to_authorized,col_a_candidate_loan_repayments,col_a_other_loan_repayments,col_a_total_loan_repayments,col_a_refunds_to_individuals,col_a_refunds_to_party_committees,col_a_refunds_to_other_committees,col_a_total_refunds,col_a_other_disbursements,col_a_total_disbursements,col_a_cash_beginning_reporting_period,col_a_cash_on_hand_close\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(P3.4|8.3|8.2)\0" as *const u8 as *const libc::c_char,
        b"^f3z2\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,principal_committee_name,coverage_from_date,coverage_through_date,col_a_net_contributions,col_a_net_operating_expenditures,col_a_debts_to,col_a_debts_by,col_a_individual_contributions,col_a_political_party_contributions,col_a_pac_contributions,col_a_candidate_contributions,col_a_total_contributions,col_a_transfers_from_authorized,col_a_candidate_loans,col_a_other_loans,col_a_total_loans,col_a_offset_to_operating_expenditures,col_a_other_receipts,col_a_total_receipts,col_a_operating_expenditures,col_a_transfers_to_authorized,col_a_candidate_loan_repayments,col_a_other_loan_repayments,col_a_total_loan_repayments,col_a_refunds_to_individuals,col_a_refunds_to_party_committees,col_a_refunds_to_other_committees,col_a_total_refunds,col_a_other_disbursements,col_a_total_disbursements,col_a_cash_beginning_reporting_period,col_a_cash_on_hand_close\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(P3.4|P3.3|P3.2)\0" as *const u8 as *const libc::c_char,
        b"^f4[ant]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,street_1,street_2,city,state,zip_code,committee_type,committee_type_description,report_code,coverage_from_date,coverage_through_date,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,date_signed,col_a_cash_on_hand_beginning_reporting_period,col_a_total_receipts,col_a_subtotal,col_a_total_disbursements,col_a_cash_on_hand_close_of_period,col_a_debts_to,col_a_debts_by,col_a_convention_expenditures,col_a_convention_refunds,col_a_expenditures_subject_to_limits,col_a_prior_expenditures_subject_to_limits,col_a_federal_funds,col_a_contributions_itemized,col_a_contributions_unitemized,col_a_contributions_subtotal,col_a_transfers_from_affiliated,col_a_loans_received,col_a_loan_repayments_received,col_a_loan_receipts_subtotal,col_a_convention_refunds_itemized,col_a_convention_refunds_unitemized,col_a_convention_refunds_subtotal,col_a_other_refunds_itemized,col_a_other_refunds_unitemized,col_a_other_refunds_subtotal,col_a_other_income_itemized,col_a_other_income_unitemized,col_a_other_income_subtotal,col_a_total_receipts,col_a_convention_expenses_itemized,col_a_convention_expenses_unitemized,col_a_convention_expenses_subtotal,col_a_transfers_to_affiliated,col_a_loans_made,col_a_loan_repayments_made,col_a_loan_disbursements_subtotal,col_a_other_disbursements_itemized,col_a_other_disbursements_unitemized,col_a_other_disbursements_subtotal,col_a_total_disbursements,col_b_cash_on_hand_beginning_year,col_b_beginning_year,col_b_total_receipts,col_b_subtotal,col_b_total_disbursements,col_b_cash_on_hand_close_of_period,col_b_convention_expenditures,col_b_convention_refunds,col_b_expenditures_subject_to_limits,col_b_prior_expendiutres_subject_to_limits,col_b_total_expenditures_subject_to_limits,col_b_federal_funds,col_b_contributions_subtotal,col_b_transfers_from_affiliated,col_b_loan_receipts_subtotal,col_b_convention_refunds_subtotal,col_b_other_refunds_subtotal,col_b_other_income_subtotal,col_b_total_receipts,col_b_convention_expenses_subtotal,col_b_transfers_to_affiliated,col_b_loan_disbursements_subtotal,col_b_other_disbursements_subtotal,col_b_total_disbursements,beginning_image_number,end_image_number,receipt_date\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(P3.1|P3.0|P2|P1)\0" as *const u8 as *const libc::c_char,
        b"^f4[ant]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,street_1,street_2,city,state,zip_code,committee_type,committee_type_description,report_code,coverage_from_date,coverage_through_date,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,date_signed,col_a_cash_on_hand_beginning_reporting_period,col_a_total_receipts,col_a_subtotal,col_a_total_disbursements,col_a_cash_on_hand_close_of_period,col_a_debts_to,col_a_debts_by,col_a_convention_expenditures,col_a_convention_refunds,col_a_expenditures_subject_to_limits,col_a_prior_expenditures_subject_to_limits,col_a_federal_funds,col_a_contributions_itemized,col_a_contributions_unitemized,col_a_contributions_subtotal,col_a_transfers_from_affiliated,col_a_loans_received,col_a_loan_repayments_received,col_a_loan_receipts_subtotal,col_a_convention_refunds_itemized,col_a_convention_refunds_unitemized,col_a_convention_refunds_subtotal,col_a_other_refunds_itemized,col_a_other_refunds_unitemized,col_a_other_refunds_subtotal,col_a_other_income_itemized,col_a_other_income_unitemized,col_a_other_income_subtotal,col_a_total_receipts,col_a_convention_expenses_itemized,col_a_convention_expenses_unitemized,col_a_convention_expenses_subtotal,col_a_transfers_to_affiliated,col_a_loans_made,col_a_loan_repayments_made,col_a_loan_disbursements_subtotal,col_a_other_disbursements_itemized,col_a_other_disbursements_unitemized,col_a_other_disbursements_subtotal,col_a_total_disbursements,col_b_cash_on_hand_beginning_year,col_b_beginning_year,col_b_total_receipts,col_b_subtotal,col_b_total_disbursements,col_b_cash_on_hand_close_of_period,col_b_convention_expenditures,col_b_convention_refunds,col_b_expenditures_subject_to_limits,col_b_prior_expendiutres_subject_to_limits,col_b_total_expenditures_subject_to_limits,col_b_federal_funds,col_b_contributions_subtotal,col_b_transfers_from_affiliated,col_b_loan_receipts_subtotal,col_b_convention_refunds_subtotal,col_b_other_refunds_subtotal,col_b_other_income_subtotal,col_b_total_receipts,col_b_convention_expenses_subtotal,col_b_transfers_to_affiliated,col_b_loan_disbursements_subtotal,col_b_other_disbursements_subtotal,col_b_total_disbursements,beginning_image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0|7.0|6.4|6.3|6.2|6.1\0" as *const u8
            as *const libc::c_char,
        b"^f4[ant]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,street_1,street_2,city,state,zip_code,committee_type,committee_type_description,report_code,coverage_from_date,coverage_through_date,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,date_signed,col_a_cash_on_hand_beginning_reporting_period,col_a_total_receipts,col_a_subtotal,col_a_total_disbursements,col_a_cash_on_hand_close_of_period,col_a_debts_to,col_a_debts_by,col_a_convention_expenditures,col_a_convention_refunds,col_a_expenditures_subject_to_limits,col_a_prior_expenditures_subject_to_limits,col_a_federal_funds,col_a_contributions_itemized,col_a_contributions_unitemized,col_a_contributions_subtotal,col_a_transfers_from_affiliated,col_a_loans_received,col_a_loan_repayments_received,col_a_loan_receipts_subtotal,col_a_convention_refunds_itemized,col_a_convention_refunds_unitemized,col_a_convention_refunds_subtotal,col_a_other_refunds_itemized,col_a_other_refunds_unitemized,col_a_other_refunds_subtotal,col_a_other_income_itemized,col_a_other_income_unitemized,col_a_other_income_subtotal,col_a_total_receipts,col_a_convention_expenses_itemized,col_a_convention_expenses_unitemized,col_a_convention_expenses_subtotal,col_a_transfers_to_affiliated,col_a_loans_made,col_a_loan_repayments_made,col_a_loan_disbursements_subtotal,col_a_other_disbursements_itemized,col_a_other_disbursements_unitemized,col_a_other_disbursements_subtotal,col_a_total_disbursements,col_b_cash_on_hand_beginning_year,col_b_beginning_year,col_b_total_receipts,col_b_subtotal,col_b_total_disbursements,col_b_cash_on_hand_close_of_period,col_b_convention_expenditures,col_b_convention_refunds,col_b_expenditures_subject_to_limits,col_b_prior_expendiutres_subject_to_limits,col_b_total_expenditures_subject_to_limits,col_b_federal_funds,col_b_contributions_subtotal,col_b_transfers_from_affiliated,col_b_loan_receipts_subtotal,col_b_convention_refunds_subtotal,col_b_other_refunds_subtotal,col_b_other_income_subtotal,col_b_total_receipts,col_b_convention_expenses_subtotal,col_b_transfers_to_affiliated,col_b_loan_disbursements_subtotal,col_b_other_disbursements_subtotal,col_b_total_disbursements\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3|5.2|5.1|5.0|^3\0" as *const u8 as *const libc::c_char,
        b"^f4[ant]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,street_1,street_2,city,state,zip_code,committee_type,committee_type_description,report_code,coverage_from_date,coverage_through_date,col_a_cash_on_hand_beginning_reporting_period,col_a_total_receipts,col_a_subtotal,col_a_total_disbursements,col_a_cash_on_hand_close_of_period,col_a_debts_to,col_a_debts_by,col_a_convention_expenditures,col_a_convention_refunds,col_a_expenditures_subject_to_limits,col_a_prior_expenditures_subject_to_limits,col_a_total_expenditures_subject_to_limits,col_a_federal_funds,col_a_contributions_itemized,col_a_contributions_unitemized,col_a_contributions_subtotal,col_a_transfers_from_affiliated,col_a_loans_received,col_a_loan_repayments_received,col_a_loan_receipts_subtotal,col_a_convention_refunds_itemized,col_a_convention_refunds_unitemized,col_a_convention_refunds_subtotal,col_a_other_refunds_itemized,col_a_other_refunds_unitemized,col_a_other_refunds_subtotal,col_a_other_income_itemized,col_a_other_income_unitemized,col_a_other_income_subtotal,col_a_total_receipts,col_a_convention_expenses_itemized,col_a_convention_expenses_unitemized,col_a_convention_expenses_subtotal,col_a_transfers_to_affiliated,col_a_loans_made,col_a_loan_repayments_made,col_a_loan_disbursements_subtotal,col_a_other_disbursements_itemized,col_a_other_disbursements_unitemized,col_a_other_disbursements_subtotal,col_a_total_disbursements,col_b_cash_on_hand_beginning_year,col_b_beginning_year,col_b_total_receipts,col_b_subtotal,col_b_total_disbursements,col_b_cash_on_hand_close_of_period,col_b_convention_expenditures,col_b_convention_refunds,col_b_expenditures_subject_to_limits,col_b_prior_expendiutres_subject_to_limits,col_b_total_expenditures_subject_to_limits,col_b_federal_funds,col_b_contributions_subtotal,col_b_transfers_from_affiliated,col_b_loan_receipts_subtotal,col_b_convention_refunds_subtotal,col_b_other_refunds_subtotal,col_b_other_income_subtotal,col_b_total_receipts,col_b_convention_expenses_subtotal,col_b_transfers_to_affiliated,col_b_loan_disbursements_subtotal,col_b_other_disbursements_subtotal,col_b_total_disbursements,treasurer_name,date_signed\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(P3.4|P3.3|P3.2)\0" as *const u8 as *const libc::c_char,
        b"^f5[na]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,organization_name,individual_last_name,individual_first_name,individual_middle_name,individual_prefix,individual_suffix,change_of_address,street_1,street_2,city,state,zip_code,individual_occupation,individual_employer,report_code,report_type,original_amendment_date,coverage_from_date,coverage_through_date,total_contribution,total_independent_expenditure,person_completing_last_name,person_completing_first_name,person_completing_middle_name,person_completing_prefix,person_completing_suffix,date_signed,beginning_image_number,end_image_number,receipt_date\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P3.1\0" as *const u8 as *const libc::c_char,
        b"^f5[na]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,organization_name,individual_last_name,individual_first_name,individual_middle_name,individual_prefix,individual_suffix,change_of_address,street_1,street_2,city,state,zip_code,individual_occupation,individual_employer,report_code,report_type,original_amendment_date,coverage_from_date,coverage_through_date,total_contribution,total_independent_expenditure,person_completing_last_name,person_completing_first_name,person_completing_middle_name,person_completing_prefix,person_completing_suffix,date_signed,beginning_image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P2|^P3.0\0" as *const u8 as *const libc::c_char,
        b"^f5[na]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,organization_name,individual_last_name,individual_first_name,individual_middle_name,individual_prefix,individual_suffix,change_of_address,street_1,street_2,city,state,zip_code,qualified_nonprofit,individual_employer,individual_occupation,report_code,report_type,coverage_from_date,coverage_through_date,total_contribution,total_independent_expenditure,person_completing_last_name,person_completing_first_name,person_completing_middle_name,person_completing_prefix,person_completing_suffix,date_signed,beginning_image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P1\0" as *const u8 as *const libc::c_char,
        b"^f5[na]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,organization_name,individual_last_name,individual_first_name,individual_middle_name,individual_prefix,individual_suffix,change_of_address,street_1,street_2,city,state,zip_code,qualified_nonprofit,individual_employer,individual_occupation,report_code,report_type,election_code,election_date,election_state,coverage_from_date,coverage_through_date,total_contribution,total_independent_expenditure,person_completing_last_name,person_completing_first_name,person_completing_middle_name,person_completing_prefix,person_completing_suffix,date_signed,beginning_image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(8.3|8.2|8.1)\0" as *const u8 as *const libc::c_char,
        b"^f5[na]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,organization_name,individual_last_name,individual_first_name,individual_middle_name,individual_prefix,individual_suffix,change_of_address,street_1,street_2,city,state,zip_code,individual_occupation,individual_employer,report_code,report_type,original_amendment_date,coverage_from_date,coverage_through_date,total_contribution,total_independent_expenditure,person_completing_last_name,person_completing_first_name,person_completing_middle_name,person_completing_prefix,person_completing_suffix,date_signed\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(8.0|7.0|6.4|6.3|6.2|6.1)\0" as *const u8 as *const libc::c_char,
        b"^f5[na]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,organization_name,individual_last_name,individual_first_name,individual_middle_name,individual_prefix,individual_suffix,change_of_address,street_1,street_2,city,state,zip_code,qualified_nonprofit,individual_employer,individual_occupation,report_code,report_type,coverage_from_date,coverage_through_date,total_contribution,total_independent_expenditure,person_completing_last_name,person_completing_first_name,person_completing_middle_name,person_completing_prefix,person_completing_suffix,date_signed\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3\0" as *const u8 as *const libc::c_char,
        b"^f5[na]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,street_1,street_2,city,state,zip_code,change_of_address,qualified_nonprofit,individual_employer,,report_code,,,,coverage_from_date,coverage_through_date,total_contribution,total_independent_expenditure,person_completing_name,date_signed,,,,report_type\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(5.2|5.1|5.0)\0" as *const u8 as *const libc::c_char,
        b"^f5[na]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,street_1,street_2,city,state,zip_code,change_of_address,qualified_nonprofit,individual_employer,individual_occupation,report_code,report_pgi,election_date,election_state,coverage_from_date,coverage_through_date,total_contribution,total_independent_expenditure,person_completing_name,date_signed,date_notarized,date_notary_commission_expires,notary_name,report_type\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^3\0" as *const u8 as *const libc::c_char,
        b"^f5[na]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,street_1,street_2,city,state,zip_code,change_of_address,qualified_nonprofit,individual_employer,individual_occupation,report_code,report_pgi,election_date,election_state,coverage_from_date,coverage_through_date,total_contribution,total_independent_expenditure,person_completing_name,date_signed,date_notarized,date_notary_commission_expires,notary_name\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(P3|P2|P1)\0" as *const u8 as *const libc::c_char,
        b"^f56\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,contributor_organization_name,contributor_last_name,contributor_first_name,contributor_middle_name,contributor_prefix,contributor_suffix,contributor_street_1,contributor_street_2,contributor_city,contributor_state,contributor_zip_code,contributor_fec_id,contribution_date,contribution_amount,contributor_employer,contributor_occupation,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0|7.0|6.4|6.3|6.2\0" as *const u8 as *const libc::c_char,
        b"^f56\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id,entity_type,contributor_organization_name,contributor_last_name,contributor_first_name,contributor_middle_name,contributor_prefix,contributor_suffix,contributor_street_1,contributor_street_2,contributor_city,contributor_state,contributor_zip_code,contributor_fec_id,contribution_date,contribution_amount,contributor_employer,contributor_occupation\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3|5.2|5.1|5.0|^3.0\0" as *const u8 as *const libc::c_char,
        b"^f56\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,contributor_name,contributor_street_1,contributor_street_2,contributor_city,contributor_state,contributor_zip_code,contributor_employer,contributor_occupation,contribution_date,contribution_amount,contributor_fec_id,candidate_id,candidate_name,candidate_office,candidate_state,candidate_district,conduit_name,conduit_street_1,conduit_street_2,conduit_city,conduit_state,conduit_zip_code,,transaction_id\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P(3|2|1)\0" as *const u8 as *const libc::c_char,
        b"^f57\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,dissemination_date,expenditure_amount,expenditure_purpose_descrip,category_code,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix,candidate_office,candidate_state,candidate_district,support_oppose_code,calendar_y_t_d_per_election_office,election_code,election_other_description,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1\0" as *const u8 as *const libc::c_char,
        b"^f57\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id_number,entity_type,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,election_code,election_other_description,dissemination_date,expenditure_amount,calendar_y_t_d_per_election_office,expenditure_purpose_descrip,category_code,payee_cmtte_fec_id_number,support_oppose_code,candidate_id_number,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix,candidate_office,candidate_state,candidate_district\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.0\0" as *const u8 as *const libc::c_char,
        b"^f57\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id_number,entity_type,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,election_code,election_other_description,dissemination_date,expenditure_amount,calendar_y_t_d_per_election_office,expenditure_purpose_descrip,category_code,payee_cmtte_fec_id_number,support_oppose_code,candidate_id_number,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix,candidate_office,candidate_state,candidate_district\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^7.0|6.4|6.3|6.2|6.1\0" as *const u8 as *const libc::c_char,
        b"^f57\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id_number,entity_type,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,election_code,election_other_description,dissemination_date,expenditure_amount,calendar_y_t_d_per_election_office,expenditure_purpose_code,expenditure_purpose_descrip,category_code,payee_cmtte_fec_id_number,support_oppose_code,candidate_id_number,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix,candidate_office,candidate_state,candidate_district\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3|5.2|5.1|5.0\0" as *const u8 as *const libc::c_char,
        b"^f57\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,payee_name,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,expenditure_purpose_descrip,dissemination_date,expenditure_amount,support_oppose_code,candidate_id_number,candidate_name,candidate_office,candidate_state,candidate_district,,,,,,,conduit_name,conduit_street_1,conduit_street_2,conduit_city,conduit_state,conduit_zip_code,,transaction_id_number,category_code,expenditure_purpose_code,calendar_y_t_d_per_election_office,election_code,election_other_description\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^3\0" as *const u8 as *const libc::c_char,
        b"^f57\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,payee_name,payee_street_2,,payee_city,payee_state,payee_zip_code,expenditure_purpose_descrip,dissemination_date,expenditure_amount,support_oppose_code,candidate_id_number,candidate_name,candidate_office,candidate_state,candidate_district,,,,,,,conduit_name,conduit_street_1,conduit_street_2,conduit_city,conduit_state,conduit_zip_code,amended_code\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P3.2|^P3.3|^P3.4\0" as *const u8 as *const libc::c_char,
        b"(^f6$)|(^f6[an])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,original_amendment_date,committee_name,street_1,street_2,city,state,zip_code,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix,candidate_office,candidate_state,candidate_district,date_signed,beginning_image_number,end_image_number,receipt_date\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P3.0|^P3.1\0" as *const u8 as *const libc::c_char,
        b"(^f6$)|(^f6[an])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,original_amendment_date,committee_name,street_1,street_2,city,state,zip_code,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix,candidate_office,candidate_state,candidate_district,date_signed,beginning_image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P1|^P2\0" as *const u8 as *const libc::c_char,
        b"(^f6$)|(^f6[an])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,street_1,street_2,city,state,zip_code,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix,candidate_office,candidate_state,candidate_district,date_signed,beginning_image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0\0" as *const u8 as *const libc::c_char,
        b"(^f6$)|(^f6[an])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,original_amendment_date,committee_name,street_1,street_2,city,state,zip_code,candidate_id_number,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix,candidate_office,candidate_state,candidate_district,signer_last_name,signer_first_name,signer_middle_name,signer_prefix,signer_suffix,date_signed\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^7.0|6.4|6.3|6.2|6.1\0" as *const u8 as *const libc::c_char,
        b"(^f6$)|(^f6[an])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,street_1,street_2,city,state,zip_code,candidate_id_number,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix,candidate_office,candidate_state,candidate_district,signer_last_name,signer_first_name,signer_middle_name,signer_prefix,signer_suffix,date_signed\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(5.3|5.2|5.1|5.0|3.0|2|1)\0" as *const u8 as *const libc::c_char,
        b"(^f6$)|(^f6[an])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,street_1,street_2,city,state,zip_code,candidate_id_number,candidate_name,candidate_office,candidate_state,candidate_district,date_signed\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(P3|P2|P1)\0" as *const u8 as *const libc::c_char,
        b"^f65\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,contributor_organization_name,contributor_last_name,contributor_first_name,contributor_middle_name,contributor_prefix,contributor_suffix,contributor_street_1,contributor_street_2,contributor_city,contributor_state,contributor_zip_code,contributor_employer,contributor_occupation,contribution_date,contribution_amount,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0|7.0|6.4|6.3|6.2|6.1\0" as *const u8
            as *const libc::c_char,
        b"^f65\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id,entity_type,contributor_organization_name,contributor_last_name,contributor_first_name,contributor_middle_name,contributor_prefix,contributor_suffix,contributor_street_1,contributor_street_2,contributor_city,contributor_state,contributor_zip_code,contributor_fec_id,contribution_date,contribution_amount,contributor_employer,contributor_occupation\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(5.3|5.2|5.1|5.0|3.0|2)\0" as *const u8 as *const libc::c_char,
        b"^f65\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,contributor_name,contributor_street_1,contributor_street_2,contributor_city,contributor_state,contributor_zip_code,contributor_employer,contributor_occupation,contribution_date,contribution_amount,contributor_fec_id,candidate_id,candidate_name,candidate_office,candidate_state,candidate_district,conduit_name,conduit_street_1,conduit_street_2,conduit_city,conduit_state,conduit_zip_code,amended_cd,transaction_id\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P(3.4|3.3|3.2)\0" as *const u8 as *const libc::c_char,
        b"^f7[na]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,organization_name,street_1,street_2,city,state,zip_code,organization_type,report_code,election_date,election_state,coverage_from_date,coverage_through_date,total_costs,person_designated_last_name,person_designated_first_name,person_designated_middle_name,person_designated_prefix,person_designated_suffix,person_designated_title,date_signed,beginning_image_number,end_image_number,receipt_date\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P(3.1|3.0|2|1)\0" as *const u8 as *const libc::c_char,
        b"^f7[na]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,organization_name,street_1,street_2,city,state,zip_code,organization_type,report_code,election_date,election_state,coverage_from_date,coverage_through_date,total_costs,person_designated_last_name,person_designated_first_name,person_designated_middle_name,person_designated_prefix,person_designated_suffix,person_designated_title,date_signed,beginning_image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0|7.0|6.4|6.3|6.2|6.1\0" as *const u8
            as *const libc::c_char,
        b"^f7[na]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,organization_name,street_1,street_2,city,state,zip_code,organization_type,report_code,election_date,election_state,coverage_from_date,coverage_through_date,total_costs,person_designated_last_name,person_designated_first_name,person_designated_middle_name,person_designated_prefix,person_designated_suffix,person_designated_title,date_signed\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3|5.2|5.1|5.0|^3.0\0" as *const u8 as *const libc::c_char,
        b"^f7[na]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,organization_name,street_1,street_2,city,state,zip_code,organization_type,report_code,election_date,election_state,coverage_from_date,coverage_through_date,total_costs,person_designated_name,date_signed,person_designated_title\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P(3|2|1)\0" as *const u8 as *const libc::c_char,
        b"^f76\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,communication_type,communication_type_description,communication_class,communication_date,support_oppose_code,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix,candidate_office,candidate_state,candidate_district,election_code,communication_cost,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0|7.0|6.4|6.3|6.2|6.1\0" as *const u8
            as *const libc::c_char,
        b"^f76\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id,communication_type,communication_type_description,communication_class,communication_date,communication_cost,election_code,election_other_description,support_oppose_code,candidate_id_number,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix,candidate_office,candidate_state,candidate_district\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3|5.2|5.1|5.0|^3.0\0" as *const u8 as *const libc::c_char,
        b"^f76\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,communication_type,communication_type_description,communication_class,communication_date,support_oppose_code,candidate_id_number,candidate_name,candidate_office,candidate_state,candidate_district,election_code,communication_cost,,transaction_id\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^6\0" as *const u8 as *const libc::c_char,
        b"(^f8$)|(^f8[an])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,street_1,street_2,city,state,zip_code,cash_on_hand,cash_on_hand_as_of_date,total_assets_to_be_liquidated,total_assets,receipts_ytd,disbursements_ytd,total_debts_owed,total_num_creditors_owed,num_creditors_part_ii,total_debts_owed_part_ii,total_to_be_paid_to_creditors,committee_is_terminating_activities,planned_termination_report_date,other_auth_committees,other_auth_committees_description,sufficient_funds_to_pay_total,steps_taken_description,committee_filed_previous_plans,residual_funds,residual_funds_description,sufficient_funds_part_iii,sufficient_funds_part_iii_description,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,date_signed\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5\0" as *const u8 as *const libc::c_char,
        b"(^f8$)|(^f8[an])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,street_1,street_2,city,state,zip_code,cash_on_hand,cash_on_hand_as_of_date,total_assets_to_be_liquidated,total_assets,receipts_ytd,disbursements_ytd,total_debts_owed,total_num_creditors_owed,num_creditors_part_ii,total_debts_owed_part_ii,total_to_be_paid_to_creditors,committee_is_terminating_activities,planned_termination_report_date,other_auth_committees,other_auth_committees_description,sufficient_funds_to_pay_total,steps_taken_description,committee_filed_previous_plans,residual_funds,residual_funds_description,sufficient_funds_part_iii,sufficient_funds_part_iii_description,treasurer_name,date_signed\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^6\0" as *const u8 as *const libc::c_char,
        b"^f8ii$\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id,entity_type,creditor_organization_name,creditor_last_name,creditor_first_name,creditor_middle_name,creditor_prefix,creditor_suffix,creditor_street_1,creditor_street_2,creditor_city,creditor_state,creditor_zip_code,date_incurred,amount_owed_to,amount_offered_in,creditor_code,nature_of_debt_description,efforts_made_to_pay_debt,steps_taken_to_collect,effort_made_by_creditor,no_effort_description,terms_of_settlement_comparable,not_comparable_description,creditor_committee_id_number,creditor_candidate_id_number,creditor_candidate_last_name,creditor_candidate_first_name,creditor_candidate_middle_name,creditor_candidate_prefix,creditor_candidate_suffix,creditor_candidate_office,creditor_candidate_state,creditor_candidate_district,signer_last_name,signer_first_name,signer_middle_name,signer_prefix,signer_suffix,date_signed\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5\0" as *const u8 as *const libc::c_char,
        b"^f8ii$\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,creditor_code,creditor_name,creditor_street_1,creditor_street_2,creditor_city,creditor_state,creditor_zip_code,date_incurred,amount_owed_to,amount_offered_in,nature_of_debt_description,efforts_made_to_pay_debt,steps_taken_to_collect,effort_made_by_creditor,no_effort_description,terms_of_settlement_comparable,not_comparable_description,creditor_committee_id_number,creditor_candidate_id_number,creditor_candidate_name,creditor_candidate_office,creditor_candidate_state,creditor_candidate_district,signer_name,date_signed,amended_cd,transaction_id\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^6\0" as *const u8 as *const libc::c_char,
        b"^f8iii$\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id,entity_type,creditor_organization_name,creditor_last_name,creditor_first_name,creditor_middle_name,creditor_prefix,creditor_suffix,creditor_street_1,creditor_street_2,creditor_city,creditor_state,creditor_zip_code,date_incurred,amount_owed_to,amount_expected_to_pay,creditor_code,disputed_debt,creditor_committee_id_number,creditor_candidate_id_number,creditor_candidate_last_name,creditor_candidate_first_name,creditor_candidate_middle_name,creditor_candidate_prefix,creditor_candidate_suffix,creditor_candidate_office,creditor_candidate_state,creditor_candidate_district\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5\0" as *const u8 as *const libc::c_char,
        b"^f8iii$\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,creditor_code,creditor_name,creditor_street_1,creditor_street_2,creditor_city,creditor_state,creditor_zip_code,disputed_debt,date_incurred,amount_owed_to,amount_expected_to_pay,creditor_committee_id_number,creditor_candidate_id_number,creditor_candidate_name,creditor_candidate_office,creditor_candidate_state,creditor_candidate_district,amended_cd,transaction_id\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P(3.4|3.3|3.2)\0" as *const u8 as *const libc::c_char,
        b"(^f9$)|(^f9[an])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,organization_name,individual_last_name,individual_first_name,individual_middle_name,individual_prefix,individual_suffix,change_of_address,street_1,street_2,city,state,zip_code,individual_employer,individual_occupation,coverage_from_date,coverage_through_date,date_public_distribution,communication_title,filer_code,filer_code_description,segregated_bank_account,custodian_last_name,custodian_first_name,custodian_middle_name,custodian_prefix,custodian_suffix,custodian_street_1,custodian_street_2,custodian_city,custodian_state,custodian_zip_code,custodian_employer,custodian_occupation,total_donations,total_disbursements,person_completing_last_name,person_completing_first_name,person_completing_middle_name,person_completing_prefix,person_completing_suffix,date_signed,beginning_image_number,end_image_number,receipt_date\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P(3.1|3.0|2.6|2.4)\0" as *const u8 as *const libc::c_char,
        b"(^f9$)|(^f9[an])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,organization_name,individual_last_name,individual_first_name,individual_middle_name,individual_prefix,individual_suffix,change_of_address,street_1,street_2,city,state,zip_code,individual_employer,individual_occupation,coverage_from_date,coverage_through_date,date_public_distribution,communication_title,filer_code,filer_code_description,segregated_bank_account,custodian_last_name,custodian_first_name,custodian_middle_name,custodian_prefix,custodian_suffix,custodian_street_1,custodian_street_2,custodian_city,custodian_state,custodian_zip_code,custodian_employer,custodian_occupation,total_donations,total_disbursements,person_completing_last_name,person_completing_first_name,person_completing_middle_name,person_completing_prefix,person_completing_suffix,date_signed,beginning_image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P(2.3|2.2|1)\0" as *const u8 as *const libc::c_char,
        b"(^f9$)|(^f9[an])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,organization_name,individual_last_name,individual_first_name,individual_middle_name,individual_prefix,individual_suffix,change_of_address,street_1,street_2,city,state,zip_code,individual_employer,individual_occupation,coverage_from_date,coverage_through_date,date_public_distribution,communication_title,qualified_non_profit,segregated_bank_account,custodian_last_name,custodian_first_name,custodian_middle_name,custodian_prefix,custodian_suffix,custodian_street_1,custodian_street_2,custodian_city,custodian_state,custodian_zip_code,custodian_employer,custodian_occupation,total_donations,total_disbursements,person_completing_last_name,person_completing_first_name,person_completing_middle_name,person_completing_prefix,person_completing_suffix,date_signed,beginning_image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3\0" as *const u8 as *const libc::c_char,
        b"(^f9$)|(^f9[an])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,organization_name,individual_last_name,individual_first_name,individual_middle_name,individual_prefix,individual_suffix,change_of_address,street_1,street_2,city,state,zip_code,individual_employer,individual_occupation,original_amendment_date,coverage_from_date,coverage_through_date,date_public_distribution,communication_title,filer_code,filer_code_description,segregated_bank_account,custodian_last_name,custodian_first_name,custodian_middle_name,custodian_prefix,custodian_suffix,custodian_street_1,custodian_street_2,custodian_city,custodian_state,custodian_zip_code,custodian_employer,custodian_occupation,total_donations,total_disbursements,person_completing_last_name,person_completing_first_name,person_completing_middle_name,person_completing_prefix,person_completing_suffix,date_signed\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.2|8.1|8.0|7.0|6.4|6.3|6.2\0" as *const u8 as *const libc::c_char,
        b"(^f9$)|(^f9[an])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,organization_name,individual_last_name,individual_first_name,individual_middle_name,individual_prefix,individual_suffix,change_of_address,street_1,street_2,city,state,zip_code,individual_employer,individual_occupation,coverage_from_date,coverage_through_date,date_public_distribution,communication_title,filer_code,filer_code_description,segregated_bank_account,custodian_last_name,custodian_first_name,custodian_middle_name,custodian_prefix,custodian_suffix,custodian_street_1,custodian_street_2,custodian_city,custodian_state,custodian_zip_code,custodian_employer,custodian_occupation,total_donations,total_disbursements,person_completing_last_name,person_completing_first_name,person_completing_middle_name,person_completing_prefix,person_completing_suffix,date_signed\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^6.1\0" as *const u8 as *const libc::c_char,
        b"(^f9$)|(^f9[an])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,organization_name,individual_last_name,individual_first_name,individual_middle_name,individual_prefix,individual_suffix,change_of_address,street_1,street_2,city,state,zip_code,individual_employer,individual_occupation,coverage_from_date,coverage_through_date,date_public_distribution,communication_title,qualified_non_profit,segregated_bank_account,custodian_last_name,custodian_first_name,custodian_middle_name,custodian_prefix,custodian_suffix,custodian_street_1,custodian_street_2,custodian_city,custodian_state,custodian_zip_code,custodian_employer,custodian_occupation,total_donations,total_disbursements,person_completing_last_name,person_completing_first_name,person_completing_middle_name,person_completing_prefix,person_completing_suffix,date_signed\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3|5.2|5.1|5.0\0" as *const u8 as *const libc::c_char,
        b"(^f9$)|(^f9[an])\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,organization_name,street_1,street_2,city,state,zip_code,change_of_address,individual_employer,individual_occupation,coverage_from_date,coverage_through_date,date_public_distribution,communication_title,qualified_non_profit,segregated_bank_account,custodian_last_name,custodian_street_1,custodian_street_2,custodian_city,custodian_state,custodian_zip_code,custodian_employer,custodian_occupation,total_donations,total_disbursements,person_completing_last_name,date_signed\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(P3|P2|P1)\0" as *const u8 as *const libc::c_char,
        b"^f91\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,controller_last_name,controller_first_name,controller_middle_name,controller_prefix,controller_suffix,controller_street_1,controller_street_2,controller_city,controller_state,controller_zip_code,controller_employer,controller_occupation,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0|7.0|6.4|6.3|6.2|6.1\0" as *const u8
            as *const libc::c_char,
        b"^f91\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id,controller_last_name,controller_first_name,controller_middle_name,controller_prefix,controller_suffix,controller_street_1,controller_street_2,controller_city,controller_state,controller_zip_code,controller_employer,controller_occupation\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3\0" as *const u8 as *const libc::c_char,
        b"^f91\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,controller_last_name,controller_street_1,controller_street_2,controller_city,controller_state,controller_zip_code,controller_employer,controller_occupation,,transaction_id\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.2|5.1|5.0\0" as *const u8 as *const libc::c_char,
        b"^f91\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,controller_last_name,controller_street_1,controller_street_2,controller_city,controller_state,controller_zip_code,controller_employer,controller_occupation,amended_cd,transaction_id\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(P3|P2|P1)\0" as *const u8 as *const libc::c_char,
        b"^f92\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,contributor_organization_name,contributor_last_name,contributor_first_name,contributor_middle_name,contributor_prefix,contributor_suffix,contributor_street_1,contributor_street_2,contributor_city,contributor_state,contributor_zip_code,contribution_date,contribution_amount,memo_text_description,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0|7.0|6.4|6.3|6.2|6.1\0" as *const u8
            as *const libc::c_char,
        b"^f92\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id,back_reference_tran_id_number,back_reference_sched_name,entity_type,contributor_organization_name,contributor_last_name,contributor_first_name,contributor_middle_name,contributor_prefix,contributor_suffix,contributor_street_1,contributor_street_2,contributor_city,contributor_state,contributor_zip_code,contribution_date,contribution_amount\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3|5.2\0" as *const u8 as *const libc::c_char,
        b"^f92\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,contributor_organization_name,contributor_street_1,contributor_street_2,contributor_city,contributor_state,contributor_zip_code,,,contributor_employer,contributor_occupation,,contribution_date,contribution_amount,transaction_type,,,,,,,,,,,,,,,,,transaction_id,back_reference_tran_id_number,back_reference_sched_name\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.1\0" as *const u8 as *const libc::c_char,
        b"^f92\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,,contributor_street_1,contributor_street_2,contributor_city,contributor_state,contributor_zip_code,,,contributor_employer,contributor_occupation,,contribution_date,contribution_amount,transaction_type,,,,,,,,,,,,,,,,,transaction_id,back_reference_tran_id_number,back_reference_sched_name,,,contributor_organization_name,contributor_last_name,contributor_first_name,contributor_middle_name,contributor_prefix,contributor_suffix\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.0\0" as *const u8 as *const libc::c_char,
        b"^f92\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,contributor_organization_name,contributor_street_1,contributor_street_2,contributor_city,contributor_state,contributor_zip_code,,,contributor_employer,contributor_occupation,,contribution_date,contribution_amount,transaction_type,,,,,,,,,,,,,,,,,transaction_id,back_reference_tran_id_number,back_reference_sched_name\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(P3.4|3.3|3.2)\0" as *const u8 as *const libc::c_char,
        b"^f93\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,expenditure_date,expenditure_amount,communication_date,expenditure_purpose_descrip,transaction_id,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(P3.1|3.0|P2|P1)\0" as *const u8 as *const libc::c_char,
        b"^f93\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,expenditure_date,expenditure_amount,communication_date,expenditure_purpose_descrip,memo_text_description,transaction_id,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0|7.0|6.4|6.3|6.2|6.1\0" as *const u8
            as *const libc::c_char,
        b"^f93\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id,back_reference_tran_id_number,back_reference_sched_name,entity_type,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,election_code,election_other_description,expenditure_date,expenditure_amount,expenditure_purpose_descrip,payee_employer,payee_occupation,communication_date\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3|5.2\0" as *const u8 as *const libc::c_char,
        b"^f93\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,expenditure_purpose_code,expenditure_purpose_descrip,election_code,election_other_description,expenditure_date,expenditure_amount,,,,,,,,,,,,,,,,transaction_id,back_reference_tran_id_number,back_reference_sched_name,,,,communication_date,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.1\0" as *const u8 as *const libc::c_char,
        b"^f93\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,payee_organization_name,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,expenditure_purpose_descrip,expenditure_purpose_code,election_code,election_other_description,expenditure_date,expenditure_amount,,,,,,,,,,,,,,,,transaction_id,back_reference_tran_id_number,back_reference_sched_name,,,,communication_date,,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.0\0" as *const u8 as *const libc::c_char,
        b"^f93\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,payee_organization_name,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,election_code,election_other_description,payee_employer,payee_occupation,,expenditure_date,expenditure_amount,expenditure_purpose_descrip,,,,,,,,,,,,,,,,,transaction_id,back_reference_tran_id_number,back_reference_sched_name\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(P3|P2|P1)\0" as *const u8 as *const libc::c_char,
        b"^f94\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix,candidate_office,candidate_state,candidate_district,election_code,election_other_description,back_reference_tran_id_number,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0|7.0|6.4|6.3|6.2|6.1\0" as *const u8
            as *const libc::c_char,
        b"^f94\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id,back_reference_tran_id_number,back_reference_sched_name,candidate_id_number,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix,candidate_office,candidate_state,candidate_district,election_code,election_other_description\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3|5.2|5.1|5.0\0" as *const u8 as *const libc::c_char,
        b"^f94\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,candidate_id_number,candidate_name,candidate_office,candidate_state,candidate_district,election_code,election_other_description,,transaction_id,back_reference_tran_id_number,back_reference_sched_name\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(P3.4|P3.3|P3.2)\0" as *const u8 as *const libc::c_char,
        b"^f99\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,beginning_image_number,end_image_number,receipt_date\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0|7.0|6.4|6.3|6.2|6.1\0" as *const u8
            as *const libc::c_char,
        b"^f99\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,street_1,street_2,city,state,zip_code,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,date_signed,text_code,text\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3|5.2|5.1|5.0\0" as *const u8 as *const libc::c_char,
        b"^f99\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,street_1,street_2,city,state,zip_code,treasurer_name,date_signed,text_code,text\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^3\0" as *const u8 as *const libc::c_char,
        b"^f99\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,street_1,street_2,city,state,zip_code,treasurer_name,date_signed,text\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^6\0" as *const u8 as *const libc::c_char,
        b"^f10$\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,committee_name,street_1,street_2,city,state,zip_code,candidate_id,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix,candidate_office,candidate_state,candidate_district,previous_expenditure_aggregate,expenditure_total_this_report,expenditure_total_cycle_to_date,meets_f6_filing_requirements,candidate_employer,candidate_occupation,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,date_signed\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3\0" as *const u8 as *const libc::c_char,
        b"^f10$\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,candidate_name,candidate_id,candidate_office,candidate_state,candidate_district,committee_name,street_1,street_2,city,state,zip_code,previous_expenditure_aggregate,expenditure_total_this_report,expenditure_total_cycle_to_date,signer_name,date_signed,meets_f6_filing_requirements,candidate_employer,candidate_occupation\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(5.0|5.1|5.2)\0" as *const u8 as *const libc::c_char,
        b"^f10$\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,candidate_name,candidate_id,candidate_office,candidate_state,candidate_district,committee_name,street_1,street_2,city,state,zip_code,previous_expenditure_aggregate,expenditure_total_this_report,expenditure_total_cycle_to_date,signer_name,date_signed\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^6\0" as *const u8 as *const libc::c_char,
        b"^f105$\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id,election_code,election_other_description,expenditure_date,expenditure_amount,loan_check\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5\0" as *const u8 as *const libc::c_char,
        b"^f105$\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,expenditure_date,item_elect_cd,item_elect_other,expenditure_amount,loan_check,amended_cd,transaction_id\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P3.4\0" as *const u8 as *const libc::c_char,
        b"^h1\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,presidential_only_election_year,presidential_senate_election_year,senate_only_election_year,non_presidential_non_senate_election_year,federal_percent,nonfederal_percent,administrative_ratio_applies,generic_voter_drive_ratio_applies,public_communications_referencing_party_ratio_applies,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P1|^P2|^P3.0|^P3.1|^P3.2|^P3.3\0" as *const u8 as *const libc::c_char,
        b"^h1\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,presidential_only_election_year,presidential_senate_election_year,senate_only_election_year,non_presidential_non_senate_election_year,flat_minimum_federal_percentage,federal_percent,nonfederal_percent,administrative_ratio_applies,generic_voter_drive_ratio_applies,public_communications_referencing_party_ratio_applies,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2\0" as *const u8 as *const libc::c_char,
        b"^h1\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id,presidential_only_election_year,presidential_senate_election_year,senate_only_election_year,non_presidential_non_senate_election_year,federal_percent,nonfederal_percent,administrative_ratio_applies,generic_voter_drive_ratio_applies,public_communications_referencing_party_ratio_applies\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.1|8.0|7.0|6.4|6.3|6.2|6.1\0" as *const u8 as *const libc::c_char,
        b"^h1\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id,presidential_only_election_year,presidential_senate_election_year,senate_only_election_year,non_presidential_non_senate_election_year,flat_minimum_federal_percentage,federal_percent,nonfederal_percent,administrative_ratio_applies,generic_voter_drive_ratio_applies,public_communications_referencing_party_ratio_applies\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3|5.2\0" as *const u8 as *const libc::c_char,
        b"^h1\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id,,,,,,,,,,,,,,,,,,,,,,,,,,,presidential_only_election_year,presidential_senate_election_year,senate_only_election_year,non_presidential_non_senate_election_year,flat_minimum_federal_percentage,federal_percent,nonfederal_percent,administrative_ratio_applies,generic_voter_drive_ratio_applies,public_communications_referencing_party_ratio_applies\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.1|5.0\0" as *const u8 as *const libc::c_char,
        b"^h1\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,national_party_committee_percentage,house_senate_party_committees_minimum_federal_percentage,house_senate_party_committees_percentage_federal_candidate_support,house_senate_party_committees_percentage_nonfederal_candidate_support,house_senate_party_committees_actual_federal_candidate_support,house_senate_party_committees_actual_nonfederal_candidate_support,house_senate_party_committees_percentage_actual_federal,federal_percent,nonfederal_percent,actual_direct_candidate_support_federal,actual_direct_candidate_support_nonfederal,actual_direct_candidate_support_federal_percent,ballot_presidential,ballot_senate,ballot_house,subtotal_federal,ballot_governor,ballot_other_statewide,ballot_state_senate,ballot_state_representative,ballot_local_candidates,extra_nonfederal_point,subtotal,total_points,flat_minimum_federal_percentage,,transaction_id,presidential_only_election_year,presidential_senate_election_year,senate_only_election_year,non_presidential_non_senate_election_year\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^3.0|^2|^1\0" as *const u8 as *const libc::c_char,
        b"^h1\0" as *const u8 as *const libc::c_char,
        b"ballot_local_candidates,filer_committee_id_number,national_party_committee_percentage,house_senate_party_committees_minimum_federal_percentage,house_senate_party_committees_percentage_federal_candidate_support,house_senate_party_committees_percentage_nonfederal_candidate_support,house_senate_party_committees_actual_federal_candidate_support,house_senate_party_committees_actual_nonfederal_candidate_support,house_senate_party_committees_percentage_actual_federal,federal_percent,nonfederal_percent,actual_direct_candidate_support_federal,actual_direct_candidate_support_nonfederal,actual_direct_candidate_support_federal_percent,ballot_presidential,ballot_senate,ballot_house,subtotal_federal,ballot_governor,ballot_other_statewide,ballot_state_senate,ballot_state_representative,ballot_local_candidates,extra_nonfederal_point,subtotal,total_points,flat_minimum_federal_percentage,amended_cd,transaction_id\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P3|^P2|^P1\0" as *const u8 as *const libc::c_char,
        b"^h2\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,activity_event_name,direct_fundraising,direct_candidate_support,ratio_code,federal_percentage,nonfederal_percentage,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0|7.0|6.4|6.3|6.2|6.1\0" as *const u8
            as *const libc::c_char,
        b"^h2\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id,activity_event_name,direct_fundraising,direct_candidate_support,ratio_code,federal_percentage,nonfederal_percentage\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3|5.2|5.1\0" as *const u8 as *const libc::c_char,
        b"^h2\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,activity_event_name,direct_fundraising,,direct_candidate_support,ratio_code,federal_percentage,nonfederal_percentage,,transaction_id\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.0|^3.0|^2|^1\0" as *const u8 as *const libc::c_char,
        b"^h2\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,activity_event_name,direct_fundraising,exempt_activity,direct_candidate_support,ratio_code,federal_percentage,nonfederal_percentage,amended_cd,transaction_id\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P3|^P2|^P1\0" as *const u8 as *const libc::c_char,
        b"^h3\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,account_name,receipt_date,total_amount_transferred,event_type,transferred_amount,event_activity_name,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0|7.0|6.4|6.3|6.2|6.1\0" as *const u8
            as *const libc::c_char,
        b"^h3\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id,back_reference_tran_id_number,account_name,event_type,event_activity_name,receipt_date,total_amount_transferred,transferred_amount\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3|5.2|5.1|5.0|^3.0\0" as *const u8 as *const libc::c_char,
        b"^h3\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,back_reference_tran_id_number,account_name,event_activity_name,event_type,receipt_date,transferred_amount,total_amount_transferred,amended_cd,transaction_id\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^2\0" as *const u8 as *const libc::c_char,
        b"^h3\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,account_name,receipt_date,total_amount_transferred,administrative_voter_drive_activity,direct_fundraising,exempt_activity,amended_cd,transaction_id,orig_tran_id,supr_tran_id\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P3.2|^P3.3|^P3.4\0" as *const u8 as *const libc::c_char,
        b"^h4\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,expenditure_purpose_description,account_identifier,category_code,administrative_voter_drive_activity,fundraising_activity,exempt_activity,generic_voter_drive_activity,direct_candidate_support_activity,public_communications_party_activity,event_year_to_date,expenditure_date,federal_share,nonfederal_share,total_amount,memo_code,memo_text,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P1|^P2|^P3.0|^P3.1\0" as *const u8 as *const libc::c_char,
        b"^h4\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,expenditure_purpose_description,account_identifier,category_code,administrative_voter_drive_activity,fundraising_activity,exempt_activity,generic_voter_drive_activity,direct_candidate_support_activity,public_communications_party_activity,event_year_to_date,expenditure_date,federal_share,nonfederal_share,total_amount,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0\0" as *const u8 as *const libc::c_char,
        b"^h4\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id_number,back_reference_tran_id_number,back_reference_sched_name,entity_type,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,account_identifier,expenditure_date,total_amount,federal_share,nonfederal_share,event_year_to_date,expenditure_purpose_description,category_code,administrative_voter_drive_activity,fundraising_activity,exempt_activity,generic_voter_drive_activity,direct_candidate_support_activity,public_communications_party_activity,memo_code,memo_text\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^7.0|6.4|6.3|6.2|6.1\0" as *const u8 as *const libc::c_char,
        b"^h4\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id_number,back_reference_tran_id_number,back_reference_sched_name,entity_type,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,account_identifier,expenditure_date,total_amount,federal_share,nonfederal_share,event_year_to_date,expenditure_purpose_code,expenditure_purpose_description,category_code,administrative_voter_drive_activity,fundraising_activity,exempt_activity,generic_voter_drive_activity,direct_candidate_support_activity,public_communications_party_activity,memo_code,memo_text\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3|5.2\0" as *const u8 as *const libc::c_char,
        b"^h4\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,payee_name,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,expenditure_purpose_description,expenditure_date,total_amount,federal_share,nonfederal_share,,fundraising_activity,exempt_activity,direct_candidate_support_activity,event_year_to_date,account_identifier,fec_committee_id_number,fec_candidate_id_number,candidate_name,candidate_office,candidate_state,candidate_district,conduit_name,conduit_street_1,conduit_street_2,conduit_city,conduit_state,conduit_zip_code,,transaction_id_number,memo_code,memo_text,back_reference_tran_id_number,back_reference_sched_name,administrative_voter_drive_activity,generic_voter_drive_activity,category_code,expenditure_purpose_code,public_communications_party_activity\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.1|5.0\0" as *const u8 as *const libc::c_char,
        b"^h4\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,payee_name,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,expenditure_purpose_description,expenditure_date,total_amount,federal_share,nonfederal_share,,fundraising_activity,exempt_activity,direct_candidate_support_activity,event_year_to_date,account_identifier,fec_committee_id_number,fec_candidate_id_number,candidate_name,candidate_office,candidate_state,candidate_district,conduit_name,conduit_street_1,conduit_street_2,conduit_city,conduit_state,conduit_zip_code,,transaction_id_number,memo_code,memo_text,back_reference_tran_id_number,back_reference_sched_name,administrative_voter_drive_activity,generic_voter_drive_activity,category_code,expenditure_purpose_code\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^3.0\0" as *const u8 as *const libc::c_char,
        b"^h4\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,payee_name,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,expenditure_purpose_description,expenditure_date,total_amount,federal_share,nonfederal_share,administrative_voter_drive_activity,fundraising_activity,exempt_activity,direct_candidate_support_activity,event_year_to_date,account_identifier,fec_committee_id_number,fec_candidate_id_number,candidate_name,candidate_office,candidate_state,candidate_district,conduit_name,conduit_street_1,conduit_street_2,conduit_city,conduit_state,conduit_zip_code,amended_cd,transaction_id_number,memo_code,memo_text,back_reference_tran_id_number,back_reference_sched_name\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^2\0" as *const u8 as *const libc::c_char,
        b"^h4\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,payee_name,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,expenditure_purpose_description,expenditure_date,total_amount,federal_share,nonfederal_share,administrative_voter_drive_activity,fundraising_activity,exempt_activity,direct_candidate_support_activity,event_year_to_date,account_identifier,fec_committee_id_number,fec_candidate_id_number,candidate_name,candidate_office,candidate_state,candidate_district,conduit_name,conduit_street_1,conduit_street_2,conduit_city,conduit_state,conduit_zip_code,amended_cd,transaction_id_number,orig_tran_id,supr_tran_id\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(P3|P2|P1)\0" as *const u8 as *const libc::c_char,
        b"^h5\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,account_name,receipt_date,total_amount_transferred,voter_registration_amount,voter_id_amount,gotv_amount,generic_campaign_amount,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0|7.0|6.4|6.3|6.2|6.1\0" as *const u8
            as *const libc::c_char,
        b"^h5\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id,account_name,receipt_date,total_amount_transferred,voter_registration_amount,voter_id_amount,gotv_amount,generic_campaign_amount\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3|5.2|5.1|5.0\0" as *const u8 as *const libc::c_char,
        b"^h5\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,account_name,receipt_date,voter_registration_amount,voter_id_amount,gotv_amount,generic_campaign_amount,total_amount_transferred,,transaction_id\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(P3.4|P3.3|P3.2)\0" as *const u8 as *const libc::c_char,
        b"^h6\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,expenditure_purpose_description,category_code,voter_registration_activity,gotv_activity,voter_id_activity,generic_campaign_activity,event_year_to_date,expenditure_date,federal_share,levin_share,total_amount,memo_code,memo_text,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(P3.1|P3.0|P2|P1)\0" as *const u8 as *const libc::c_char,
        b"^h6\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,expenditure_purpose_description,category_code,voter_registration_activity,gotv_activity,voter_id_activity,generic_campaign_activity,event_year_to_date,expenditure_date,federal_share,levin_share,total_amount,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0\0" as *const u8 as *const libc::c_char,
        b"^h6\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id_number,back_reference_tran_id_number,back_reference_sched_name,entity_type,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,account_identifier,expenditure_date,total_amount,federal_share,levin_share,event_year_to_date,expenditure_purpose_description,category_code,voter_registration_activity,gotv_activity,voter_id_activity,generic_campaign_activity,memo_code,memo_text\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^7.0|6.4|6.3|6.2|6.1\0" as *const u8 as *const libc::c_char,
        b"^h6\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id_number,back_reference_tran_id_number,back_reference_sched_name,entity_type,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,account_identifier,expenditure_date,total_amount,federal_share,levin_share,event_year_to_date,expenditure_purpose_code,expenditure_purpose_description,category_code,voter_registration_activity,gotv_activity,voter_id_activity,generic_campaign_activity,memo_code,memo_text\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3|5.2|5.1|5.0\0" as *const u8 as *const libc::c_char,
        b"^h6\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,payee_name,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,category_code,expenditure_purpose_code,expenditure_purpose_description,expenditure_date,total_amount,federal_share,levin_share,voter_registration_activity,gotv_activity,voter_id_activity,generic_campaign_activity,event_year_to_date,account_identifier,fec_committee_id_number,fec_candidate_id_number,candidate_name,candidate_office,candidate_state,candidate_district,conduit_committee_id,conduit_name,conduit_street_1,conduit_street_2,conduit_city,conduit_state,conduit_zip_code,memo_code,memo_text,,transaction_id_number,back_reference_tran_id_number,back_reference_sched_name\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P3.2|^P3.3|^P3.4\0" as *const u8 as *const libc::c_char,
        b"^sa\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,contributor_organization_name,contributor_last_name,contributor_first_name,contributor_middle_name,contributor_prefix,contributor_suffix,contributor_street_1,contributor_street_2,contributor_city,contributor_state,contributor_zip_code,contribution_date,donor_committee_fec_id,contributor_employer,contributor_occupation,election_code,election_other_description,contribution_aggregate,contribution_amount,memo_code,memo_text_description,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(P2.6|P3.0|P3.1)\0" as *const u8 as *const libc::c_char,
        b"^sa\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,contributor_organization_name,contributor_last_name,contributor_first_name,contributor_middle_name,contributor_prefix,contributor_suffix,contributor_street_1,contributor_street_2,contributor_city,contributor_state,contributor_zip_code,contribution_date,donor_committee_fec_id,contributor_employer,contributor_occupation,election_code,election_other_description,contribution_aggregate,contribution_amount,memo_text_description,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(P1|P2.2|P2.3|P2.4)\0" as *const u8 as *const libc::c_char,
        b"^sa\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,contributor_organization_name,contributor_last_name,contributor_first_name,contributor_middle_name,contributor_prefix,contributor_suffix,contributor_street_1,contributor_street_2,contributor_city,contributor_state,contributor_zip_code,contribution_date,donor_committee_fec_id,contributor_employer,contributor_occupation,election_code,election_other_description,contribution_aggregate,contribution_amount,memo_text_description,increased_limit_code,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0\0" as *const u8 as *const libc::c_char,
        b"^sa\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id,back_reference_tran_id_number,back_reference_sched_name,entity_type,contributor_organization_name,contributor_last_name,contributor_first_name,contributor_middle_name,contributor_prefix,contributor_suffix,contributor_street_1,contributor_street_2,contributor_city,contributor_state,contributor_zip_code,election_code,election_other_description,contribution_date,contribution_amount,contribution_aggregate,contribution_purpose_descrip,contributor_employer,contributor_occupation,donor_committee_fec_id,donor_committee_name,donor_candidate_fec_id,donor_candidate_last_name,donor_candidate_first_name,donor_candidate_middle_name,donor_candidate_prefix,donor_candidate_suffix,donor_candidate_office,donor_candidate_state,donor_candidate_district,conduit_name,conduit_street1,conduit_street2,conduit_city,conduit_state,conduit_zip_code,memo_code,memo_text_description,reference_code\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^7.0|6.4\0" as *const u8 as *const libc::c_char,
        b"^sa\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id,back_reference_tran_id_number,back_reference_sched_name,entity_type,contributor_organization_name,contributor_last_name,contributor_first_name,contributor_middle_name,contributor_prefix,contributor_suffix,contributor_street_1,contributor_street_2,contributor_city,contributor_state,contributor_zip_code,election_code,election_other_description,contribution_date,contribution_amount,contribution_aggregate,contribution_purpose_code,contribution_purpose_descrip,contributor_employer,contributor_occupation,donor_committee_fec_id,donor_committee_name,donor_candidate_fec_id,donor_candidate_last_name,donor_candidate_first_name,donor_candidate_middle_name,donor_candidate_prefix,donor_candidate_suffix,donor_candidate_office,donor_candidate_state,donor_candidate_district,conduit_name,conduit_street1,conduit_street2,conduit_city,conduit_state,conduit_zip_code,memo_code,memo_text_description,reference_code\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^6.3|6.2\0" as *const u8 as *const libc::c_char,
        b"^sa\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id,back_reference_tran_id_number,back_reference_sched_name,entity_type,contributor_organization_name,contributor_last_name,contributor_first_name,contributor_middle_name,contributor_prefix,contributor_suffix,contributor_street_1,contributor_street_2,contributor_city,contributor_state,contributor_zip_code,election_code,election_other_description,contribution_date,contribution_amount,contribution_aggregate,contribution_purpose_code,contribution_purpose_descrip,increased_limit_code,contributor_employer,contributor_occupation,donor_committee_fec_id,donor_committee_name,donor_candidate_fec_id,donor_candidate_last_name,donor_candidate_first_name,donor_candidate_middle_name,donor_candidate_prefix,donor_candidate_suffix,donor_candidate_office,donor_candidate_state,donor_candidate_district,conduit_name,conduit_street1,conduit_street2,conduit_city,conduit_state,conduit_zip_code,memo_code,memo_text_description,reference_code\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^6.1\0" as *const u8 as *const libc::c_char,
        b"^sa\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id,back_reference_tran_id_number,back_reference_sched_name,entity_type,contributor_organization_name,contributor_last_name,contributor_first_name,contributor_middle_name,contributor_prefix,contributor_suffix,contributor_street_1,contributor_street_2,contributor_city,contributor_state,contributor_zip_code,election_code,election_other_description,contribution_date,contribution_amount,contribution_aggregate,contribution_purpose_code,contribution_purpose_descrip,increased_limit_code,contributor_employer,contributor_occupation,donor_committee_fec_id,donor_candidate_fec_id,donor_candidate_last_name,donor_candidate_first_name,donor_candidate_middle_name,donor_candidate_prefix,donor_candidate_suffix,donor_candidate_office,donor_candidate_state,donor_candidate_district,conduit_name,conduit_street1,conduit_street2,conduit_city,conduit_state,conduit_zip_code,memo_code,memo_text_description,reference_code\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3\0" as *const u8 as *const libc::c_char,
        b"^sa\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,contributor_name,contributor_street_1,contributor_street_2,contributor_city,contributor_state,contributor_zip_code,election_code,election_other_description,contributor_employer,contributor_occupation,contribution_aggregate,contribution_date,contribution_amount,contribution_purpose_code,contribution_purpose_descrip,donor_committee_fec_id,donor_candidate_fec_id,donor_candidate_name,donor_candidate_office,donor_candidate_state,donor_candidate_district,conduit_name,conduit_street1,conduit_street2,conduit_city,conduit_state,conduit_zip_code,memo_code,memo_text_description,amended_cd,transaction_id,back_reference_tran_id_number,back_reference_sched_name,reference_code,increased_limit_code,contributor_organization_name,contributor_last_name,contributor_first_name,contributor_middle_name,contributor_prefix,contributor_suffix\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.2\0" as *const u8 as *const libc::c_char,
        b"^sa\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,contributor_name,contributor_street_1,contributor_street_2,contributor_city,contributor_state,contributor_zip_code,election_code,election_other_description,contributor_employer,contributor_occupation,contribution_aggregate,contribution_date,contribution_amount,contribution_purpose_code,contribution_purpose_descrip,donor_committee_fec_id,donor_candidate_fec_id,donor_candidate_name,donor_candidate_office,donor_candidate_state,donor_candidate_district,conduit_name,conduit_street1,conduit_street2,conduit_city,conduit_state,conduit_zip_code,memo_code,memo_text_description,amended_cd,transaction_id,back_reference_tran_id_number,back_reference_sched_name,reference_code,increased_limit_code,contributor_organization_name,contributor_last_name,contributor_first_name,contributor_middle_name,contributor_prefix,contributor_suffix\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.1\0" as *const u8 as *const libc::c_char,
        b"^sa\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,contributor_name,contributor_street_1,contributor_street_2,contributor_city,contributor_state,contributor_zip_code,election_code,election_other_description,contributor_employer,contributor_occupation,contribution_aggregate,contribution_date,contribution_amount,contribution_purpose_code,contribution_purpose_descrip,donor_committee_fec_id,donor_candidate_fec_id,donor_candidate_name,donor_candidate_office,donor_candidate_state,donor_candidate_district,conduit_name,conduit_street1,conduit_street2,conduit_city,conduit_state,conduit_zip_code,memo_code,memo_text_description,amended_cd,transaction_id,back_reference_tran_id_number,back_reference_sched_name,reference_code,increased_limit_code,contributor_organization_name,contributor_last_name,contributor_first_name,contributor_middle_name,contributor_prefix,contributor_suffix\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.0\0" as *const u8 as *const libc::c_char,
        b"^sa\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,contributor_name,contributor_street_1,contributor_street_2,contributor_city,contributor_state,contributor_zip_code,election_code,election_other_description,contributor_employer,contributor_occupation,contribution_aggregate,contribution_date,contribution_amount,contribution_purpose_code,contribution_purpose_descrip,donor_committee_fec_id,donor_candidate_fec_id,donor_candidate_name,donor_candidate_office,donor_candidate_state,donor_candidate_district,conduit_name,conduit_street1,conduit_street2,conduit_city,conduit_state,conduit_zip_code,memo_code,memo_text_description,amended_cd,transaction_id,back_reference_tran_id_number,back_reference_sched_name,reference_code,increased_limit_code\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^3\0" as *const u8 as *const libc::c_char,
        b"^sa\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,contributor_name,contributor_street_1,contributor_street_2,contributor_city,contributor_state,contributor_zip_code,election_code,election_other_description,contributor_employer,contributor_occupation,contribution_aggregate,contribution_date,contribution_amount,contribution_purpose_code,contribution_purpose_descrip,donor_committee_fec_id,donor_candidate_fec_id,donor_candidate_name,donor_candidate_office,donor_candidate_state,donor_candidate_district,conduit_name,conduit_street1,conduit_street2,conduit_city,conduit_state,conduit_zip_code,memo_code,memo_text_description,amended_cd,transaction_id,back_reference_tran_id_number,back_reference_sched_name,reference_code\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^2\0" as *const u8 as *const libc::c_char,
        b"^sa\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,contributor_name,contributor_street_1,contributor_street_2,contributor_city,contributor_state,contributor_zip_code,election_code,election_other_description,contributor_employer,contributor_occupation,contribution_aggregate,contribution_date,contribution_amount,contribution_purpose_code,contribution_purpose_descrip,donor_committee_fec_id,donor_candidate_fec_id,donor_candidate_name,donor_candidate_office,donor_candidate_state,donor_candidate_district,conduit_name,conduit_street1,conduit_street2,conduit_city,conduit_state,conduit_zip_code,memo_code,memo_text_description,amended_cd,transaction_id,back_reference_tran_id_number,back_reference_sched_name\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^1\0" as *const u8 as *const libc::c_char,
        b"^sa\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,contributor_name,contributor_street_1,contributor_street_2,contributor_city,contributor_state,contributor_zip_code,election_code,election_other_description,contributor_employer,contributor_occupation,contribution_aggregate,contribution_date,contribution_amount,contribution_purpose_code,contribution_purpose_descrip,donor_committee_fec_id,conduit_name,conduit_street1,conduit_street2,conduit_city,conduit_state,conduit_zip_code,donor_candidate_fec_id,donor_candidate_name,donor_candidate_office,donor_candidate_state,donor_candidate_district,memo_code,memo_text_description,amended_cd\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0\0" as *const u8 as *const libc::c_char,
        b"^sa3l\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id,back_reference_tran_id_number,back_reference_sched_name,entity_type,lobbyist_registrant_organization_name,lobbyist_registrant_last_name,lobbyist_registrant_first_name,lobbyist_registrant_middle_name,lobbyist_registrant_prefix,lobbyist_registrant_suffix,lobbyist_registrant_street_1,lobbyist_registrant_street_2,lobbyist_registrant_city,lobbyist_registrant_state,lobbyist_registrant_zip_code,election_code,election_other_description,contribution_date,bundled_amount_period,bundled_amount_semi_annual,contribution_purpose_descrip,lobbyist_registrant_employer,lobbyist_registrant_occupation,donor_committee_fec_id,donor_committee_name,donor_candidate_fec_id,donor_candidate_last_name,donor_candidate_first_name,donor_candidate_middle_name,donor_candidate_prefix,donor_candidate_suffix,donor_candidate_office,donor_candidate_state,donor_candidate_district,conduit_name,conduit_street1,conduit_street2,conduit_city,conduit_state,conduit_zip_code,associated_text_record,memo_text,reference_code\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^7.0|6.4\0" as *const u8 as *const libc::c_char,
        b"^sa3l\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id,back_reference_tran_id_number,back_reference_sched_name,entity_type,lobbyist_registrant_organization_name,lobbyist_registrant_last_name,lobbyist_registrant_first_name,lobbyist_registrant_middle_name,lobbyist_registrant_prefix,lobbyist_registrant_suffix,lobbyist_registrant_street_1,lobbyist_registrant_street_2,lobbyist_registrant_city,lobbyist_registrant_state,lobbyist_registrant_zip_code,election_code,election_other_description,contribution_date,bundled_amount_period,bundled_amount_semi_annual,contribution_purpose_code,contribution_purpose_descrip,lobbyist_registrant_employer,lobbyist_registrant_occupation,donor_committee_fec_id,donor_committee_name,donor_candidate_fec_id,donor_candidate_last_name,donor_candidate_first_name,donor_candidate_middle_name,donor_candidate_prefix,donor_candidate_suffix,donor_candidate_office,donor_candidate_state,donor_candidate_district,conduit_name,conduit_street1,conduit_street2,conduit_city,conduit_state,conduit_zip_code,associated_text_record,memo_text,reference_code\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P3.3|^P3.4\0" as *const u8 as *const libc::c_char,
        b"^sb\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,expenditure_date,expenditure_purpose_descrip,beneficiary_committee_name,beneficiary_candidate_last_name,beneficiary_candidate_first_name,beneficiary_candidate_middle_name,beneficiary_candidate_prefix,beneficiary_candidate_suffix,category_code,beneficiary_candidate_office,beneficiary_candidate_state,beneficiary_candidate_district,election_code,election_other_description,expenditure_amount,semi_annual_refunded_bundled_amt,memo_code,memo_text_description,image_number,beneficiary_committee_fec_id\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P3.2\0" as *const u8 as *const libc::c_char,
        b"^sb\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,expenditure_date,expenditure_purpose_descrip,beneficiary_committee_name,beneficiary_candidate_last_name,beneficiary_candidate_first_name,beneficiary_candidate_middle_name,beneficiary_candidate_prefix,beneficiary_candidate_suffix,category_code,beneficiary_candidate_office,beneficiary_candidate_state,beneficiary_candidate_district,election_code,election_other_description,expenditure_amount,semi_annual_refunded_bundled_amt,memo_code,memo_text_description,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(P2.6|P3.0|P3.1)\0" as *const u8 as *const libc::c_char,
        b"^sb\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,expenditure_date,expenditure_purpose_descrip,beneficiary_committee_name,beneficiary_candidate_last_name,beneficiary_candidate_first_name,beneficiary_candidate_middle_name,beneficiary_candidate_prefix,beneficiary_candidate_suffix,category_code,beneficiary_candidate_office,beneficiary_candidate_state,beneficiary_candidate_district,election_code,election_other_description,expenditure_amount,semi_annual_refunded_bundled_amt,memo_text_description,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P2.4\0" as *const u8 as *const libc::c_char,
        b"^sb\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,expenditure_date,expenditure_purpose_descrip,beneficiary_committee_name,beneficiary_candidate_last_name,beneficiary_candidate_first_name,beneficiary_candidate_middle_name,beneficiary_candidate_prefix,beneficiary_candidate_suffix,category_code,beneficiary_candidate_office,beneficiary_candidate_state,beneficiary_candidate_district,election_code,election_other_description,expenditure_amount,memo_text_description,refund_or_disposal_of_excess,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(P1|P2.2|P2.3)\0" as *const u8 as *const libc::c_char,
        b"^sb\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,expenditure_date,expenditure_purpose_descrip,beneficiary_candidate_last_name,beneficiary_candidate_first_name,beneficiary_candidate_middle_name,beneficiary_candidate_prefix,beneficiary_candidate_suffix,category_code,beneficiary_candidate_office,beneficiary_candidate_state,beneficiary_candidate_district,election_code,election_other_description,expenditure_amount,memo_text_description,refund_or_disposal_of_excess,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0\0" as *const u8 as *const libc::c_char,
        b"^sb\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id_number,back_reference_tran_id_number,back_reference_sched_name,entity_type,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,election_code,election_other_description,expenditure_date,expenditure_amount,semi_annual_refunded_bundled_amt,expenditure_purpose_descrip,category_code,beneficiary_committee_fec_id,beneficiary_committee_name,beneficiary_candidate_fec_id,beneficiary_candidate_last_name,beneficiary_candidate_first_name,beneficiary_candidate_middle_name,beneficiary_candidate_prefix,beneficiary_candidate_suffix,beneficiary_candidate_office,beneficiary_candidate_state,beneficiary_candidate_district,conduit_name,conduit_street_1,conduit_street_2,conduit_city,conduit_state,conduit_zip_code,memo_code,memo_text_description,reference_to_si_or_sl_system_code_that_identifies_the_account\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^7.0|6.4\0" as *const u8 as *const libc::c_char,
        b"^sb\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id_number,back_reference_tran_id_number,back_reference_sched_name,entity_type,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,election_code,election_other_description,expenditure_date,expenditure_amount,semi_annual_refunded_bundled_amt,expenditure_purpose_code,expenditure_purpose_descrip,category_code,beneficiary_committee_fec_id,beneficiary_committee_name,beneficiary_candidate_fec_id,beneficiary_candidate_last_name,beneficiary_candidate_first_name,beneficiary_candidate_middle_name,beneficiary_candidate_prefix,beneficiary_candidate_suffix,beneficiary_candidate_office,beneficiary_candidate_state,beneficiary_candidate_district,conduit_name,conduit_street_1,conduit_street_2,conduit_city,conduit_state,conduit_zip_code,memo_code,memo_text_description,reference_to_si_or_sl_system_code_that_identifies_the_account\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^6.3|6.2\0" as *const u8 as *const libc::c_char,
        b"^sb\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id_number,back_reference_tran_id_number,back_reference_sched_name,entity_type,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,election_code,election_other_description,expenditure_date,expenditure_amount,expenditure_purpose_code,expenditure_purpose_descrip,category_code,refund_or_disposal_of_excess,communication_date,beneficiary_committee_fec_id,beneficiary_committee_name,beneficiary_candidate_fec_id,beneficiary_candidate_last_name,beneficiary_candidate_first_name,beneficiary_candidate_middle_name,beneficiary_candidate_prefix,beneficiary_candidate_suffix,beneficiary_candidate_office,beneficiary_candidate_state,beneficiary_candidate_district,conduit_name,conduit_street_1,conduit_street_2,conduit_city,conduit_state,conduit_zip_code,memo_code,memo_text_description,reference_to_si_or_sl_system_code_that_identifies_the_account\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^6.1\0" as *const u8 as *const libc::c_char,
        b"^sb\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id_number,back_reference_tran_id_number,back_reference_sched_name,entity_type,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,election_code,election_other_description,expenditure_date,expenditure_amount,expenditure_purpose_code,expenditure_purpose_descrip,category_code,refund_or_disposal_of_excess,communication_date,beneficiary_committee_fec_id,beneficiary_candidate_fec_id,beneficiary_candidate_last_name,beneficiary_candidate_first_name,beneficiary_candidate_middle_name,beneficiary_candidate_prefix,beneficiary_candidate_suffix,beneficiary_candidate_office,beneficiary_candidate_state,beneficiary_candidate_district,conduit_name,conduit_street_1,conduit_street_2,conduit_city,conduit_state,conduit_zip_code,memo_code,memo_text_description,reference_to_si_or_sl_system_code_that_identifies_the_account\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3\0" as *const u8 as *const libc::c_char,
        b"^sb\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,payee_name,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,expenditure_purpose_code,expenditure_purpose_descrip,election_code,election_other_description,expenditure_date,expenditure_amount,beneficiary_committee_fec_id,beneficiary_candidate_fec_id,beneficiary_candidate_name,beneficiary_candidate_office,beneficiary_candidate_state,beneficiary_candidate_district,conduit_name,conduit_street_1,conduit_street_2,conduit_city,conduit_state,conduit_zip_code,memo_code,memo_text_description,amended_cd,transaction_id_number,back_reference_tran_id_number,back_reference_sched_name,reference_to_si_or_sl_system_code_that_identifies_the_account,refund_or_disposal_of_excess,category_code,communication_date,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.2|5.1\0" as *const u8 as *const libc::c_char,
        b"^sb\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,payee_name,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,expenditure_purpose_code,expenditure_purpose_descrip,election_code,election_other_description,expenditure_date,expenditure_amount,beneficiary_committee_fec_id,beneficiary_candidate_fec_id,beneficiary_candidate_name,beneficiary_candidate_office,beneficiary_candidate_state,beneficiary_candidate_district,conduit_name,conduit_street_1,conduit_street_2,conduit_city,conduit_state,conduit_zip_code,memo_code,memo_text_description,amended_cd,transaction_id_number,back_reference_tran_id_number,back_reference_sched_name,reference_to_si_or_sl_system_code_that_identifies_the_account,refund_or_disposal_of_excess,category_code,communication_date,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.0\0" as *const u8 as *const libc::c_char,
        b"^sb\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,payee_name,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,expenditure_purpose_code,expenditure_purpose_descrip,election_code,election_other_description,expenditure_date,expenditure_amount,beneficiary_committee_fec_id,beneficiary_candidate_fec_id,beneficiary_candidate_name,beneficiary_candidate_office,beneficiary_candidate_state,beneficiary_candidate_district,conduit_name,conduit_street_1,conduit_street_2,conduit_city,conduit_state,conduit_zip_code,memo_code,memo_text_description,amended_cd,transaction_id_number,back_reference_tran_id_number,back_reference_sched_name,reference_to_si_or_sl_system_code_that_identifies_the_account,refund_or_disposal_of_excess,category_code,communication_date\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^3|^2\0" as *const u8 as *const libc::c_char,
        b"^sb\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,payee_name,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,expenditure_purpose_code,expenditure_purpose_descrip,election_code,election_other_description,expenditure_date,expenditure_amount,beneficiary_committee_fec_id,beneficiary_candidate_fec_id,beneficiary_candidate_name,beneficiary_candidate_office,beneficiary_candidate_state,beneficiary_candidate_district,conduit_name,conduit_street_1,conduit_street_2,conduit_city,conduit_state,conduit_zip_code,memo_code,memo_text_description,amended_cd,transaction_id_number,back_reference_tran_id_number,back_reference_sched_name,reference_to_si_or_sl_system_code_that_identifies_the_account\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^1\0" as *const u8 as *const libc::c_char,
        b"^sb\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,payee_name,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,expenditure_purpose_code,expenditure_purpose_descrip,election_code,election_other_description,expenditure_date,expenditure_amount,beneficiary_committee_fec_id,conduit_name,conduit_street_1,conduit_street_2,conduit_city,conduit_state,conduit_zip_code,beneficiary_candidate_fec_id,beneficiary_candidate_name,beneficiary_candidate_office,beneficiary_candidate_state,beneficiary_candidate_district,memo_code,memo_text_description,amended_cd\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P3.4|^P3.3|^P3.2\0" as *const u8 as *const libc::c_char,
        b"^sc[^1-2]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,receipt_line_number,lender_organization_name,lender_last_name,lender_first_name,lender_middle_name,lender_prefix,lender_suffix,lender_street_1,lender_street_2,lender_city,lender_state,lender_zip_code,election_code,election_other_description,loan_amount_original,loan_payment_to_date,loan_balance,loan_incurred_date_terms,loan_due_date_terms,loan_interest_rate_terms,secured,memo_code,memo_text_description,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P3.1|^P3.0|^P2.6|^P2.4\0" as *const u8 as *const libc::c_char,
        b"^sc[^1-2]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,receipt_line_number,lender_organization_name,lender_last_name,lender_first_name,lender_middle_name,lender_prefix,lender_suffix,lender_street_1,lender_street_2,lender_city,lender_state,lender_zip_code,election_code,election_other_description,loan_amount_original,loan_payment_to_date,loan_balance,loan_incurred_date_terms,loan_due_date_terms,loan_interest_rate_terms,secured,memo_text_description,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P2.3|^P2.2|^P1\0" as *const u8 as *const libc::c_char,
        b"^sc[^1-2]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,receipt_line_number,lender_organization_name,lender_last_name,lender_first_name,lender_middle_name,lender_prefix,lender_suffix,lender_street_1,lender_street_2,lender_city,lender_state,lender_zip_code,election_code,election_other_description,loan_amount_original,loan_payment_to_date,loan_balance,loan_incurred_date_terms,loan_due_date_terms,loan_interest_rate_terms,secured,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0|7.0|6.4|6.3|6.2\0" as *const u8 as *const libc::c_char,
        b"^sc[^1-2]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id_number,receipt_line_number,entity_type,lender_organization_name,lender_last_name,lender_first_name,lender_middle_name,lender_prefix,lender_suffix,lender_street_1,lender_street_2,lender_city,lender_state,lender_zip_code,election_code,election_other_description,loan_amount_original,loan_payment_to_date,loan_balance,loan_incurred_date_terms,loan_due_date_terms,loan_interest_rate_terms,secured,personal_funds,lender_committee_id_number,lender_candidate_id_number,lender_candidate_last_name,lender_candidate_first_name,lender_candidate_middle_nm,lender_candidate_prefix,lender_candidate_suffix,lender_candidate_office,lender_candidate_state,lender_candidate_district,memo_code,memo_text_description\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^6.1\0" as *const u8 as *const libc::c_char,
        b"^sc[^1-2]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id_number,receipt_line_number,entity_type,lender_organization_name,lender_last_name,lender_first_name,lender_middle_name,lender_prefix,lender_suffix,lender_street_1,lender_street_2,lender_city,lender_state,lender_zip_code,election_code,election_other_description,loan_amount_original,loan_payment_to_date,loan_balance,loan_incurred_date_terms,loan_due_date_terms,loan_interest_rate_terms,secured,lender_committee_id_number,lender_candidate_id_number,lender_candidate_last_name,lender_candidate_first_name,lender_candidate_middle_nm,lender_candidate_prefix,lender_candidate_suffix,lender_candidate_office,lender_candidate_state,lender_candidate_district\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3\0" as *const u8 as *const libc::c_char,
        b"^sc[^1-2]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,lender_name,lender_street_1,lender_street_2,lender_city,lender_state,lender_zip_code,election_code,election_other_description,loan_amount_original,loan_payment_to_date,loan_balance,loan_incurred_date_terms,loan_due_date_terms,loan_interest_rate_terms,secured,lender_committee_id_number,lender_candidate_id_number,lender_candidate_name,lender_candidate_office,lender_candidate_state,lender_candidate_district,amended_cd,transaction_id_number,receipt_line_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.2|5.1\0" as *const u8 as *const libc::c_char,
        b"^sc[^1-2]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,lender_name,lender_street_1,lender_street_2,lender_city,lender_state,lender_zip_code,election_code,election_other_description,loan_amount_original,loan_payment_to_date,loan_balance,loan_incurred_date_terms,loan_due_date_terms,loan_interest_rate_terms,secured,lender_committee_id_number,lender_candidate_id_number,lender_candidate_name,lender_candidate_office,lender_candidate_state,lender_candidate_district,amended_cd,transaction_id_number,receipt_line_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.0|^3|^2|^1\0" as *const u8 as *const libc::c_char,
        b"^sc[^1-2]\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,lender_name,lender_street_1,lender_street_2,lender_city,lender_state,lender_zip_code,election_code,election_other_description,loan_amount_original,loan_payment_to_date,loan_balance,loan_incurred_date_terms,loan_due_date_terms,loan_interest_rate_terms,secured,lender_committee_id_number,lender_candidate_id_number,lender_candidate_name,lender_candidate_office,lender_candidate_state,lender_candidate_district,amended_cd,transaction_id_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(P3.4|P3.3|P3.2)\0" as *const u8 as *const libc::c_char,
        b"^sc1\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,lender_organization_name,lender_street_1,lender_street_2,lender_city,lender_state,lender_zip_code,loan_amount,loan_interest_rate,loan_incurred_date,loan_due_date,loan_restructured,loan_incurred_date_original,credit_amount_this_draw,total_balance,others_liable,collateral,description,collateral_value_amount,perfected_interest,future_income,description,estimated_value,established_date,account_location_name,street_1,street_2,city,state,zip_code,f_basis_of_loan_description,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,date_signed,authorized_last_name,authorized_first_name,authorized_middle_name,authorized_prefix,authorized_suffix,authorized_title,authorized_date,deposit_acct_auth_date_presidential,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(P1|P2|P3.0|P3.1)\0" as *const u8 as *const libc::c_char,
        b"^sc1\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,lender_organization_name,lender_street_1,lender_street_2,lender_city,lender_state,lender_zip_code,loan_amount,loan_interest_rate,loan_incurred_date,loan_due_date,loan_restructured,loan_incurred_date_original,credit_amount_this_draw,total_balance,others_liable,collateral,description,collateral_value_amount,perfected_interest,future_income,description,estimated_value,established_date,account_location_name,street_1,street_2,city,state,zip_code,f_basis_of_loan_description,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,date_signed,authorized_last_name,authorized_first_name,authorized_middle_name,authorized_prefix,authorized_suffix,authorized_title,authorized_date,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0|7.0|6.4|6.3|6.2|6.1\0" as *const u8
            as *const libc::c_char,
        b"^sc1\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id_number,back_reference_tran_id_number,lender_organization_name,lender_street_1,lender_street_2,lender_city,lender_state,lender_zip_code,loan_amount,loan_interest_rate,loan_incurred_date,loan_due_date,loan_restructured,loan_incurred_date_original,credit_amount_this_draw,total_balance,others_liable,collateral,description,collateral_value_amount,perfected_interest,future_income,description,estimated_value,established_date,account_location_name,street_1,street_2,city,state,zip_code,deposit_acct_auth_date_presidential,f_basis_of_loan_description,treasurer_last_name,treasurer_first_name,treasurer_middle_name,treasurer_prefix,treasurer_suffix,date_signed,authorized_last_name,authorized_first_name,authorized_middle_name,authorized_prefix,authorized_suffix,authorized_title,authorized_date\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3\0" as *const u8 as *const libc::c_char,
        b"^sc1\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,back_reference_tran_id_number,entity_type,lender_organization_name,lender_street_1,lender_street_2,lender_city,lender_state,lender_zip_code,loan_amount,loan_interest_rate,loan_incurred_date,loan_due_date,loan_restructured,loan_incurred_date_original,credit_amount_this_draw,total_balance,others_liable,collateral,description,collateral_value_amount,perfected_interest,future_income,description,estimated_value,established_date,account_location_name,street_1,street_2,city,state,zip_code,deposit_acct_auth_date_presidential,f_basis_of_loan_description,treasurer_name,date_signed,authorized_name,authorized_title,authorized_date\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.2|5.1|5.0\0" as *const u8 as *const libc::c_char,
        b"^sc1\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,back_reference_tran_id_number,entity_type,lender_organization_name,lender_street_1,lender_street_2,lender_city,lender_state,lender_zip_code,loan_amount,loan_interest_rate,loan_incurred_date,loan_due_date,loan_restructured,loan_incurred_date_original,credit_amount_this_draw,total_balance,others_liable,collateral,description,collateral_value_amount,perfected_interest,future_income,description,estimated_value,established_date,account_location_name,street_1,street_2,city,state,zip_code,deposit_acct_auth_date_presidential,f_basis_of_loan_description,treasurer_name,date_signed,authorized_name,authorized_title,authorized_date\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^3\0" as *const u8 as *const libc::c_char,
        b"^sc1\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,back_reference_tran_id_number,entity_type,lender_organization_name,lender_street_1,lender_street_2,lender_city,lender_state,lender_zip_code,loan_amount,loan_interest_rate,loan_incurred_date,loan_due_date,loan_restructured,loan_incurred_date_original,credit_amount_this_draw,total_balance,others_liable,collateral,description,collateral_value_amount,perfected_interest,future_income,description,estimated_value,established_date,account_location_name,street_1,street_2,city,state,zip_code,deposit_acct_auth_date_presidential,f_basis_of_loan_description,treasurer_name,date_signed,authorized_name,authorized_title,authorized_date\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^2\0" as *const u8 as *const libc::c_char,
        b"^sc1\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,back_reference_tran_id_number,entity_type,lender_organization_name,lender_street_1,lender_street_2,lender_city,lender_state,lender_zip_code,loan_amount,loan_interest_rate,loan_incurred_date,loan_due_date,loan_restructured,loan_incurred_date_original,credit_amount_this_draw,total_balance,others_liable,collateral,description,collateral_value_amount,perfected_interest,future_income,description,estimated_value,established_date,account_location_name,street_1,street_2,city,state,zip_code,deposit_acct_auth_date_presidential,f_basis_of_loan_description,treasurer_name,date_signed,authorized_name,authorized_title\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(P1|P2|P3)\0" as *const u8 as *const libc::c_char,
        b"^sc2\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,guarantor_last_name,guarantor_first_name,guarantor_middle_name,guarantor_prefix,guarantor_suffix,guarantor_street_1,guarantor_street_2,guarantor_city,guarantor_state,guarantor_zip_code,guarantor_employer,guarantor_occupation,guaranteed_amount,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0|7.0|6.4|6.3|6.2|6.1\0" as *const u8
            as *const libc::c_char,
        b"^sc2\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id_number,back_reference_tran_id_number,guarantor_last_name,guarantor_first_name,guarantor_middle_name,guarantor_prefix,guarantor_suffix,guarantor_street_1,guarantor_street_2,guarantor_city,guarantor_state,guarantor_zip_code,guarantor_employer,guarantor_occupation,guaranteed_amount\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3\0" as *const u8 as *const libc::c_char,
        b"^sc2\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,back_reference_tran_id_number,guarantor_name,guarantor_street_1,guarantor_street_2,guarantor_city,guarantor_state,guarantor_zip_code,guarantor_employer,guarantor_occupation,guaranteed_amount\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.2|5.1|5.0\0" as *const u8 as *const libc::c_char,
        b"^sc2\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,back_reference_tran_id_number,guarantor_name,guarantor_street_1,guarantor_street_2,guarantor_city,guarantor_state,guarantor_zip_code,guarantor_employer,guarantor_occupation,guaranteed_amount\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^3|^2\0" as *const u8 as *const libc::c_char,
        b"^sc2\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,back_reference_tran_id_number,guarantor_name,guarantor_street_1,guarantor_street_2,guarantor_city,guarantor_state,guarantor_zip_code,guarantor_employer,guarantor_occupation,guaranteed_amount\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P3|^P2|^P1\0" as *const u8 as *const libc::c_char,
        b"^sd\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,creditor_organization_name,creditor_last_name,creditor_first_name,creditor_middle_name,creditor_prefix,creditor_suffix,creditor_street_1,creditor_street_2,creditor_city,creditor_state,creditor_zip_code,purpose_of_debt_or_obligation,beginning_balance_this_period,incurred_amount_this_period,payment_amount_this_period,balance_at_close_this_period,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0|7.0|6.4|6.3|6.2|6.1\0" as *const u8
            as *const libc::c_char,
        b"^sd\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id_number,entity_type,creditor_organization_name,creditor_last_name,creditor_first_name,creditor_middle_name,creditor_prefix,creditor_suffix,creditor_street_1,creditor_street_2,creditor_city,creditor_state,creditor_zip_code,purpose_of_debt_or_obligation,beginning_balance_this_period,incurred_amount_this_period,payment_amount_this_period,balance_at_close_this_period\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3\0" as *const u8 as *const libc::c_char,
        b"^sd\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,creditor_name,creditor_street_1,creditor_street_2,creditor_city,creditor_state,creditor_zip_code,purpose_of_debt_or_obligation,beginning_balance_this_period,incurred_amount_this_period,payment_amount_this_period,balance_at_close_this_period,fec_committee_id_number,fec_candidate_id_number,candidate_name,candidate_office,candidate_state,candidate_district,conduit_name,conduit_street_1,conduit_street_2,conduit_city,conduit_state,conduit_zip_code,amended_cd,transaction_id_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.2|5.1|5.0|^3|^2\0" as *const u8 as *const libc::c_char,
        b"^sd\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,creditor_name,creditor_street_1,creditor_street_2,creditor_city,creditor_state,creditor_zip_code,purpose_of_debt_or_obligation,beginning_balance_this_period,incurred_amount_this_period,payment_amount_this_period,balance_at_close_this_period,fec_committee_id_number,fec_candidate_id_number,candidate_name,candidate_office,candidate_state,candidate_district,conduit_name,conduit_street_1,conduit_street_2,conduit_city,conduit_state,conduit_zip_code,amended_cd,transaction_id_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^1\0" as *const u8 as *const libc::c_char,
        b"^sd\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,creditor_name,creditor_street_1,creditor_street_2,creditor_city,creditor_state,creditor_zip_code,purpose_of_debt_or_obligation,beginning_balance_this_period,incurred_amount_this_period,payment_amount_this_period,balance_at_close_this_period,amended_cd\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(P3.4|P3.3|P3.2)\0" as *const u8 as *const libc::c_char,
        b"^se\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,dissemination_date,expenditure_amount,disbursement_date,expenditure_purpose_descrip,category_code,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix,candidate_office,candidate_district,candidate_state,support_oppose_code,calendar_y_t_d_per_election_office,election_code,election_other_description,completing_last_name,completing_first_name,completing_middle_name,completing_prefix,completing_suffix,date_signed,memo_code,memo_text_description,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^P3.1\0" as *const u8 as *const libc::c_char,
        b"^se\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,dissemination_date,expenditure_amount,disbursement_date,expenditure_purpose_descrip,category_code,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix,candidate_office,candidate_district,candidate_state,support_oppose_code,calendar_y_t_d_per_election_office,election_code,election_other_description,completing_last_name,completing_first_name,completing_middle_name,completing_prefix,completing_suffix,date_signed,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(P1|P2|P3.0)\0" as *const u8 as *const libc::c_char,
        b"^se\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,dissemination_date,expenditure_amount,expenditure_purpose_descrip,category_code,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix,candidate_office,candidate_district,candidate_state,support_oppose_code,calendar_y_t_d_per_election_office,election_code,election_other_description,completing_last_name,completing_first_name,completing_middle_name,completing_prefix,completing_suffix,date_signed,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1\0" as *const u8 as *const libc::c_char,
        b"^se\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id_number,back_reference_tran_id_number,back_reference_sched_name,entity_type,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,election_code,election_other_description,dissemination_date,expenditure_amount,disbursement_date,calendar_y_t_d_per_election_office,expenditure_purpose_descrip,category_code,payee_cmtte_fec_id_number,support_oppose_code,candidate_id_number,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix,candidate_office,candidate_district,candidate_state,completing_last_name,completing_first_name,completing_middle_name,completing_prefix,completing_suffix,date_signed,memo_code,memo_text_description\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.0\0" as *const u8 as *const libc::c_char,
        b"^se\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id_number,back_reference_tran_id_number,back_reference_sched_name,entity_type,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,election_code,election_other_description,dissemination_date,expenditure_amount,calendar_y_t_d_per_election_office,expenditure_purpose_descrip,category_code,payee_cmtte_fec_id_number,support_oppose_code,candidate_id_number,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix,candidate_office,candidate_state,candidate_district,completing_last_name,completing_first_name,completing_middle_name,completing_prefix,completing_suffix,date_signed,memo_code,memo_text_description\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^7.0|6.4|6.3|6.2|6.1\0" as *const u8 as *const libc::c_char,
        b"^se\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id_number,back_reference_tran_id_number,back_reference_sched_name,entity_type,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,election_code,election_other_description,dissemination_date,expenditure_amount,calendar_y_t_d_per_election_office,expenditure_purpose_code,expenditure_purpose_descrip,category_code,payee_cmtte_fec_id_number,support_oppose_code,candidate_id_number,candidate_last_name,candidate_first_name,candidate_middle_name,candidate_prefix,candidate_suffix,candidate_office,candidate_state,candidate_district,completing_last_name,completing_first_name,completing_middle_name,completing_prefix,completing_suffix,date_signed,memo_code,memo_text_description\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3\0" as *const u8 as *const libc::c_char,
        b"^se\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,payee_name,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,expenditure_purpose_descrip,dissemination_date,expenditure_amount,support_oppose_code,candidate_id_number,candidate_name,candidate_office,candidate_state,candidate_district,payee_cmtte_fec_id_number,,,,,,conduit_name,conduit_street_1,conduit_street_2,conduit_city,conduit_state,conduit_zip_code,ind_name_as_signed,date_signed,date_notarized,date_notary_commission_expires,ind_name_notary,,transaction_id_number,memo_code,memo_text_description,back_reference_tran_id_number,back_reference_sched_name,election_code,election_other_description,category_code,expenditure_purpose_code,calendar_y_t_d_per_election_office\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.2|5.1|5.0\0" as *const u8 as *const libc::c_char,
        b"^se\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,payee_name,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,expenditure_purpose_descrip,dissemination_date,expenditure_amount,support_oppose_code,candidate_id_number,candidate_name,candidate_office,candidate_state,candidate_district,payee_cmtte_fec_id_number,,,,,,conduit_name,conduit_street_1,conduit_street_2,conduit_city,conduit_state,conduit_zip_code,ind_name_as_signed,date_signed,date_notarized,date_notary_commission_expires,ind_name_notary,,transaction_id_number,memo_code,memo_text_description,back_reference_tran_id_number,back_reference_sched_name,election_code,election_other_description,category_code,expenditure_purpose_code,calendar_y_t_d_per_election_office\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^3\0" as *const u8 as *const libc::c_char,
        b"^se\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,payee_name,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,expenditure_purpose_descrip,dissemination_date,expenditure_amount,support_oppose_code,candidate_id_number,candidate_name,candidate_office,candidate_state,candidate_district,payee_cmtte_fec_id_number,,,,,,conduit_name,conduit_street_1,conduit_street_2,conduit_city,conduit_state,conduit_zip_code,ind_name_as_signed,date_signed,date_notarized,date_notary_commission_expires,ind_name_notary,,transaction_id_number,memo_code,memo_text_description,back_reference_tran_id_number,back_reference_sched_name\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^2\0" as *const u8 as *const libc::c_char,
        b"^se\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,entity_type,payee_name,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,expenditure_purpose_descrip,dissemination_date,expenditure_amount,support_oppose_code,candidate_id_number,candidate_name,candidate_office,candidate_state,candidate_district,payee_cmtte_fec_id_number,,,,,,conduit_name,conduit_street_1,conduit_street_2,conduit_city,conduit_state,conduit_zip_code,ind_name_as_signed,date_signed,date_notarized,date_notary_commission_expires,ind_name_notary,amended_cd,transaction_id_number,back_reference_tran_id_number,back_reference_sched_name\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^1\0" as *const u8 as *const libc::c_char,
        b"^se\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,payee_name,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,expenditure_purpose_descrip,dissemination_date,expenditure_amount,candidate_id_number,candidate_name,candidate_office,candidate_state,candidate_district,support_oppose_code,ind_name_as_signed,date_signed,date_notarized,date_notary_commission_expires,ind_name_notary,amended_cd\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(P3.4|P3.3|P3.2)\0" as *const u8 as *const libc::c_char,
        b"^sf\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,coordinated_expenditures,designating_committee_name,subordinate_committee_name,subordinate_street_1,subordinate_street_2,subordinate_city,subordinate_state,subordinate_zip_code,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,expenditure_purpose_descrip,category_code,expenditure_date,payee_candidate_last_name,payee_candidate_first_name,payee_candidate_middle_name,payee_candidate_prefix,payee_candidate_suffix,payee_candidate_office,payee_candidate_state,payee_candidate_district,aggregate_general_elec_expended,expenditure_amount,memo_code,memo_text_description,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(P2.6|P3.0|P3.1)\0" as *const u8 as *const libc::c_char,
        b"^sf\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,coordinated_expenditures,designating_committee_name,subordinate_committee_name,subordinate_street_1,subordinate_street_2,subordinate_city,subordinate_state,subordinate_zip_code,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,expenditure_purpose_descrip,category_code,expenditure_date,payee_candidate_last_name,payee_candidate_first_name,payee_candidate_middle_name,payee_candidate_prefix,payee_candidate_suffix,payee_candidate_office,payee_candidate_state,payee_candidate_district,aggregate_general_elec_expended,expenditure_amount,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(P1|P2.2|P2.3|P2.4)\0" as *const u8 as *const libc::c_char,
        b"^sf\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,24_hour_notice,coordinated_expenditures,designating_committee_name,subordinate_committee_name,subordinate_street_1,subordinate_street_2,subordinate_city,subordinate_state,subordinate_zip_code,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,expenditure_purpose_descrip,category_code,expenditure_date,payee_candidate_last_name,payee_candidate_first_name,payee_candidate_middle_name,payee_candidate_prefix,payee_candidate_suffix,payee_candidate_office,payee_candidate_state,payee_candidate_district,aggregate_general_elec_expended,expenditure_amount,increased_limit,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0\0" as *const u8 as *const libc::c_char,
        b"^sf\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id_number,back_reference_tran_id_number,back_reference_sched_name,coordinated_expenditures,designating_committee_id_number,designating_committee_name,subordinate_committee_id_number,subordinate_committee_name,subordinate_street_1,subordinate_street_2,subordinate_city,subordinate_state,subordinate_zip_code,entity_type,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,expenditure_date,expenditure_amount,aggregate_general_elec_expended,expenditure_purpose_descrip,category_code,payee_committee_id_number,payee_candidate_id_number,payee_candidate_last_name,payee_candidate_first_name,payee_candidate_middle_name,payee_candidate_prefix,payee_candidate_suffix,payee_candidate_office,payee_candidate_state,payee_candidate_district,memo_code,memo_text_description\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^7.0|6.4\0" as *const u8 as *const libc::c_char,
        b"^sf\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id_number,back_reference_tran_id_number,back_reference_sched_name,coordinated_expenditures,designating_committee_id_number,designating_committee_name,subordinate_committee_id_number,subordinate_committee_name,subordinate_street_1,subordinate_street_2,subordinate_city,subordinate_state,subordinate_zip_code,entity_type,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,expenditure_date,expenditure_amount,aggregate_general_elec_expended,expenditure_purpose_code,expenditure_purpose_descrip,category_code,payee_committee_id_number,payee_candidate_id_number,payee_candidate_last_name,payee_candidate_first_name,payee_candidate_middle_name,payee_candidate_prefix,payee_candidate_suffix,payee_candidate_office,payee_candidate_state,payee_candidate_district,memo_code,memo_text_description\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^6.3|6.2|6.1\0" as *const u8 as *const libc::c_char,
        b"^sf\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id_number,back_reference_tran_id_number,back_reference_sched_name,coordinated_expenditures,designating_committee_id_number,designating_committee_name,subordinate_committee_id_number,subordinate_committee_name,subordinate_street_1,subordinate_street_2,subordinate_city,subordinate_state,subordinate_zip_code,entity_type,payee_organization_name,payee_last_name,payee_first_name,payee_middle_name,payee_prefix,payee_suffix,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,expenditure_date,expenditure_amount,aggregate_general_elec_expended,expenditure_purpose_code,expenditure_purpose_descrip,category_code,increased_limit,payee_committee_id_number,payee_candidate_id_number,payee_candidate_last_name,payee_candidate_first_name,payee_candidate_middle_name,payee_candidate_prefix,payee_candidate_suffix,payee_candidate_office,payee_candidate_state,payee_candidate_district,memo_code,memo_text_description\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3\0" as *const u8 as *const libc::c_char,
        b"^sf\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,coordinated_expenditures,designating_committee_id_number,designating_committee_name,subordinate_committee_id_number,subordinate_committee_name,subordinate_street_1,subordinate_street_2,subordinate_city,subordinate_state,subordinate_zip_code,entity_type,payee_name,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,aggregate_general_elec_expended,expenditure_purpose_descrip,expenditure_date,expenditure_amount,payee_committee_id_number,payee_candidate_id_number,payee_candidate_name,payee_candidate_office,payee_candidate_state,payee_candidate_district,conduit_name,conduit_street_1,conduit_street_2,conduit_city,conduit_state,conduit_zip_code,,transaction_id_number,memo_code,memo_text_description,back_reference_tran_id_number,back_reference_sched_name,increased_limit,category_code,expenditure_purpose_code\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.2|5.1|5.0\0" as *const u8 as *const libc::c_char,
        b"^sf\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,coordinated_expenditures,designating_committee_id_number,designating_committee_name,subordinate_committee_id_number,subordinate_committee_name,subordinate_street_1,subordinate_street_2,subordinate_city,subordinate_state,subordinate_zip_code,entity_type,payee_name,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,aggregate_general_elec_expended,expenditure_purpose_descrip,expenditure_date,expenditure_amount,payee_committee_id_number,payee_candidate_id_number,payee_candidate_name,payee_candidate_office,payee_candidate_state,payee_candidate_district,conduit_name,conduit_street_1,conduit_street_2,conduit_city,conduit_state,conduit_zip_code,,transaction_id_number,memo_code,memo_text_description,back_reference_tran_id_number,back_reference_sched_name,increased_limit,category_code,expenditure_purpose_code\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^3\0" as *const u8 as *const libc::c_char,
        b"^sf\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,coordinated_expenditures,designating_committee_id_number,designating_committee_name,subordinate_committee_id_number,subordinate_committee_name,subordinate_street_1,subordinate_street_2,subordinate_city,subordinate_state,subordinate_zip_code,entity_type,payee_name,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,aggregate_general_elec_expended,expenditure_purpose_descrip,expenditure_date,expenditure_amount,payee_committee_id_number,payee_candidate_id_number,payee_candidate_name,payee_candidate_office,payee_candidate_state,payee_candidate_district,conduit_name,conduit_street_1,conduit_street_2,conduit_city,conduit_state,conduit_zip_code,amended_cd,transaction_id_number,memo_code,memo_text_description,back_reference_tran_id_number,back_reference_sched_name\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^2\0" as *const u8 as *const libc::c_char,
        b"^sf\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,coordinated_expenditures,designating_committee_id_number,designating_committee_name,subordinate_committee_id_number,subordinate_committee_name,subordinate_street_1,subordinate_street_2,subordinate_city,subordinate_state,subordinate_zip_code,entity_type,payee_name,payee_street_1,payee_street_2,payee_city,payee_state,payee_zip_code,aggregate_general_elec_expended,expenditure_purpose_descrip,expenditure_date,expenditure_amount,payee_committee_id_number,payee_candidate_id_number,payee_candidate_name,payee_candidate_office,payee_candidate_state,payee_candidate_district,conduit_name,conduit_street_1,conduit_street_2,conduit_city,conduit_state,conduit_zip_code,amended_cd,transaction_id_number,orig_tran_id,supr_tran_id\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(3|5)\0" as *const u8 as *const libc::c_char,
        b"^si\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,bank_account_id,account_name,coverage_from_date,coverage_through_date,col_a_total_receipts,col_a_transfers_to_fed,col_a_transfers_to_state_local,col_a_direct_state_local_support,col_a_other_disbursements,col_a_total_disbursements,col_a_cash_on_hand_beginning_period,col_a_receipts_period,col_a_subtotal,col_a_disbursements_period,col_a_cash_on_hand_close_of_period,col_b_total_receipts,col_b_transfers_to_fed,col_b_transfers_to_state_local,col_b_direct_state_local_support,col_b_other_disbursements,col_b_total_disbursements,col_b_cash_on_hand_beginning_period,col_b_receipts_period,col_b_subtotal,col_b_disbursements_period,col_b_cash_on_hand_close_of_period,amended_cd,transaction_id,account_identifier\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^(P3|P2|P1)\0" as *const u8 as *const libc::c_char,
        b"^sl\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,account_name,col_a_itemized_receipts_persons,col_a_unitemized_receipts_persons,col_a_total_receipts_persons,col_a_other_receipts,col_a_total_receipts,col_a_voter_registration_disbursements,col_a_voter_id_disbursements,col_a_gotv_disbursements,col_a_generic_campaign_disbursements,col_a_disbursements_subtotal,col_a_other_disbursements,col_a_total_disbursements,col_a_cash_on_hand_beginning_period,col_a_receipts_period,col_a_subtotal_period,col_a_disbursements_period,col_a_cash_on_hand_close_of_period,col_b_itemized_receipts_persons,col_b_unitemized_receipts_persons,col_b_total_receipts_persons,col_b_other_receipts,col_b_total_receipts,col_b_voter_registration_disbursements,col_b_voter_id_disbursements,col_b_gotv_disbursements,col_b_generic_campaign_disbursements,col_b_disbursements_subtotal,col_b_other_disbursements,col_b_total_disbursements,col_b_cash_on_hand_beginning_period,col_b_receipts_period,col_b_subtotal_period,col_b_disbursements_period,col_b_cash_on_hand_close_of_period,image_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0|7.0|6.4|6.3|6.2|6.1\0" as *const u8
            as *const libc::c_char,
        b"^sl\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,transaction_id_number,record_id_number,account_name,coverage_from_date,coverage_through_date,col_a_itemized_receipts_persons,col_a_unitemized_receipts_persons,col_a_total_receipts_persons,col_a_other_receipts,col_a_total_receipts,col_a_voter_registration_disbursements,col_a_voter_id_disbursements,col_a_gotv_disbursements,col_a_generic_campaign_disbursements,col_a_disbursements_subtotal,col_a_other_disbursements,col_a_total_disbursements,col_a_cash_on_hand_beginning_period,col_a_receipts_period,col_a_subtotal_period,col_b_disbursements_period,col_b_cash_on_hand_close_of_period,col_b_itemized_receipts_persons,col_b_unitemized_receipts_persons,col_b_total_receipts_persons,col_b_other_receipts,col_b_total_receipts,col_b_voter_registration_disbursements,col_b_voter_id_disbursements,col_b_gotv_disbursements,col_b_generic_campaign_disbursements,col_b_disbursements_subtotal,col_b_other_disbursements,col_b_total_disbursements,col_b_cash_on_hand_beginning_period,col_b_receipts_period,col_b_subtotal_period,col_b_disbursements_period,col_b_cash_on_hand_close_of_period\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3|5.2|5.1|5.0\0" as *const u8 as *const libc::c_char,
        b"^sl\0" as *const u8 as *const libc::c_char,
        b"form_type,filer_committee_id_number,account_name,record_id_number,coverage_from_date,coverage_through_date,col_a_itemized_receipts_persons,col_a_unitemized_receipts_persons,col_a_total_receipts_persons,col_a_other_receipts,col_a_total_receipts,col_a_voter_registration_disbursements,col_a_voter_id_disbursements,col_a_gotv_disbursements,col_a_generic_campaign_disbursements,col_a_disbursements_subtotal,col_a_other_disbursements,col_a_total_disbursements,col_a_cash_on_hand_beginning_period,col_a_receipts_period,col_a_subtotal_period,col_b_disbursements_period,col_b_itemized_receipts_persons,col_b_unitemized_receipts_persons,col_b_total_receipts_persons,col_b_other_receipts,col_b_total_receipts,col_b_voter_registration_disbursements,col_b_voter_id_disbursements,col_b_gotv_disbursements,col_b_generic_campaign_disbursements,col_b_disbursements_subtotal,col_b_other_disbursements,col_b_total_disbursements,col_b_cash_on_hand_beginning_period,col_b_receipts_period,col_b_subtotal_period,col_b_disbursements_period,col_b_cash_on_hand_close_of_period,,transaction_id_number\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^8.4|8.3|8.2|8.1|8.0|7.0|6.4|6.3|6.2|6.1\0" as *const u8
            as *const libc::c_char,
        b"^text\0" as *const u8 as *const libc::c_char,
        b"rec_type,filer_committee_id_number,transaction_id_number,back_reference_tran_id_number,back_reference_sched_form_name,text\0"
            as *const u8 as *const libc::c_char,
    ],
    [
        b"^5.3\0" as *const u8 as *const libc::c_char,
        b"^text\0" as *const u8 as *const libc::c_char,
        b"rec_type,form_type,back_reference_tran_id_number,text\0" as *const u8
            as *const libc::c_char,
    ],
    [
        b"^5.2|5.1|5.0\0" as *const u8 as *const libc::c_char,
        b"^text\0" as *const u8 as *const libc::c_char,
        b"rec_type,form_type,back_reference_tran_id_number,text\0" as *const u8
            as *const libc::c_char,
    ],
    [
        b"^3\0" as *const u8 as *const libc::c_char,
        b"^text\0" as *const u8 as *const libc::c_char,
        b"rec_type,form_type,back_reference_tran_id_number,text\0" as *const u8
            as *const libc::c_char,
    ],
];
static mut types: [[*const libc::c_char; 4]; 102] = [
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f1[an]\0" as *const u8 as *const libc::c_char,
        b"^date_signed\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f1[an]\0" as *const u8 as *const libc::c_char,
        b"^effective_date\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^(f1m$|f1m[a|n])\0" as *const u8 as *const libc::c_char,
        b".*_date$\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^(f1m$|f1m[a|n])\0" as *const u8 as *const libc::c_char,
        b"^date_signed\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f13[an]\0" as *const u8 as *const libc::c_char,
        b".*_date$\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f13[an]\0" as *const u8 as *const libc::c_char,
        b"^date_signed\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"(^f132)|(^f133)\0" as *const u8 as *const libc::c_char,
        b".*_date$\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"(^f132)|(^f133)\0" as *const u8 as *const libc::c_char,
        b".*_amount$\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"(^f2$)|(^f2[^4])\0" as *const u8 as *const libc::c_char,
        b"^date_signed\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"(^f24$)|(^f24[an])\0" as *const u8 as *const libc::c_char,
        b"^date_signed\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^(f3[ant]|f3p$|(f3p[^s|3])|f3x$|f3x[ant]|f3z)\0" as *const u8
            as *const libc::c_char,
        b"^col_\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^(f3[ant]|f3p$|(f3p[^s|3])|f3x$|f3x[ant]|f3z)\0" as *const u8
            as *const libc::c_char,
        b"^coverage_from_date\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^(f3[ant]|f3p$|(f3p[^s|3])|f3x$|f3x[ant]|f3z)\0" as *const u8
            as *const libc::c_char,
        b"^coverage_through_date\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^(f3[ant]|f3p$|(f3p[^s|3])|f3x$|f3x[ant]|f3z)\0" as *const u8
            as *const libc::c_char,
        b"^date_signed\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^(f3[ant]|f3p$|(f3p[^s|3])|f3x$|f3x[ant]|f3z)\0" as *const u8
            as *const libc::c_char,
        b"^election_date\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f3s\0" as *const u8 as *const libc::c_char,
        b"^date_general_election\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f3s\0" as *const u8 as *const libc::c_char,
        b"^date_day_after_general_election\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f3s\0" as *const u8 as *const libc::c_char,
        b"^[abcde]_\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f3s\0" as *const u8 as *const libc::c_char,
        b"^(transfers|offsets|other|total)\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f4.*\0" as *const u8 as *const libc::c_char,
        b"^col_\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f4.*\0" as *const u8 as *const libc::c_char,
        b"^coverage_from_date\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f4.*\0" as *const u8 as *const libc::c_char,
        b"^coverage_through_date\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f4.*\0" as *const u8 as *const libc::c_char,
        b"^date_signed\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f5[na]\0" as *const u8 as *const libc::c_char,
        b"^coverage_from_date\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f5[na]\0" as *const u8 as *const libc::c_char,
        b"^coverage_through_date\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f5[na]\0" as *const u8 as *const libc::c_char,
        b"^date_signed\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f5[na]\0" as *const u8 as *const libc::c_char,
        b"^total_contribution\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f5[na]\0" as *const u8 as *const libc::c_char,
        b"^total_independent_expenditure\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f57\0" as *const u8 as *const libc::c_char,
        b"^calendar_y_t_d_per_election_office\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f57\0" as *const u8 as *const libc::c_char,
        b"^dissemination_date\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f57\0" as *const u8 as *const libc::c_char,
        b"^expenditure_amount\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"(^f6$)|(^f6[an])\0" as *const u8 as *const libc::c_char,
        b"^date_signed\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f65\0" as *const u8 as *const libc::c_char,
        b"^contribution_amount\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f65\0" as *const u8 as *const libc::c_char,
        b"^contribution_date\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"(^f8$)|(^f8[an])\0" as *const u8 as *const libc::c_char,
        b"^cash_on_hand$\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"(^f8$)|(^f8[an])\0" as *const u8 as *const libc::c_char,
        b"^cash_on_hand_as_of_date\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"(^f8$)|(^f8[an])\0" as *const u8 as *const libc::c_char,
        b"^total_assets\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"(^f8$)|(^f8[an])\0" as *const u8 as *const libc::c_char,
        b"^receipts_ytd\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"(^f8$)|(^f8[an])\0" as *const u8 as *const libc::c_char,
        b"^disbursements_ytd\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"(^f8$)|(^f8[an])\0" as *const u8 as *const libc::c_char,
        b"^total_debts_owed\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"(^f8$)|(^f8[an])\0" as *const u8 as *const libc::c_char,
        b"^total_num_creditors_owed\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"(^f8$)|(^f8[an])\0" as *const u8 as *const libc::c_char,
        b"^num_creditors_part_ii\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"(^f8$)|(^f8[an])\0" as *const u8 as *const libc::c_char,
        b"^total_debts_owed_part_ii\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"(^f8$)|(^f8[an])\0" as *const u8 as *const libc::c_char,
        b"^total_to_be_paid_to_creditors\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"(^f8$)|(^f8[an])\0" as *const u8 as *const libc::c_char,
        b"^date_signed\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f8ii$\0" as *const u8 as *const libc::c_char,
        b"^date_incurred\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f8ii$\0" as *const u8 as *const libc::c_char,
        b"^amount_owed_to\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f8ii$\0" as *const u8 as *const libc::c_char,
        b"^amount_offered_in\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f8ii$\0" as *const u8 as *const libc::c_char,
        b"^date_signed\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f8iii$\0" as *const u8 as *const libc::c_char,
        b"^date_incurred\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f8iii$\0" as *const u8 as *const libc::c_char,
        b"^amount_owed_to\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f8iii$\0" as *const u8 as *const libc::c_char,
        b"^amount_expected_to_pay\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"(^f9$)|(^f9[an])\0" as *const u8 as *const libc::c_char,
        b"^original_amendment_date\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"(^f9$)|(^f9[an])\0" as *const u8 as *const libc::c_char,
        b"^coverage_from_date\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"(^f9$)|(^f9[an])\0" as *const u8 as *const libc::c_char,
        b"^coverage_through_date\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"(^f9$)|(^f9[an])\0" as *const u8 as *const libc::c_char,
        b"^date_public_distribution\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"(^f9$)|(^f9[an])\0" as *const u8 as *const libc::c_char,
        b"^total_donations\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"(^f9$)|(^f9[an])\0" as *const u8 as *const libc::c_char,
        b"^total_disbursements\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"(^f9$)|(^f9[an])\0" as *const u8 as *const libc::c_char,
        b"^date_signed\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f92\0" as *const u8 as *const libc::c_char,
        b"^contribution_date\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f92\0" as *const u8 as *const libc::c_char,
        b"^contribution_amount\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f93\0" as *const u8 as *const libc::c_char,
        b"^expenditure_date\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f93\0" as *const u8 as *const libc::c_char,
        b"^expenditure_amount\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f93\0" as *const u8 as *const libc::c_char,
        b"^communication_date\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f99\0" as *const u8 as *const libc::c_char,
        b"^date_signed\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f10$\0" as *const u8 as *const libc::c_char,
        b"^expenditure_total_\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f10$\0" as *const u8 as *const libc::c_char,
        b"^previous_expenditure_aggregate\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f10$\0" as *const u8 as *const libc::c_char,
        b"^date_signed\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f105$\0" as *const u8 as *const libc::c_char,
        b"^expenditure_amount\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^f105$\0" as *const u8 as *const libc::c_char,
        b"^expenditure_date\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^sa\0" as *const u8 as *const libc::c_char,
        b"^contribution_aggregate\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^sa\0" as *const u8 as *const libc::c_char,
        b"^contribution_amount\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^sa\0" as *const u8 as *const libc::c_char,
        b"^contribution_date\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^sb\0" as *const u8 as *const libc::c_char,
        b"^expenditure_amount\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^sb\0" as *const u8 as *const libc::c_char,
        b"^semi_annual_refunded_bundled_amt\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^sb\0" as *const u8 as *const libc::c_char,
        b"^expenditure_date\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^sc\0" as *const u8 as *const libc::c_char,
        b"^authorized_date\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^sc\0" as *const u8 as *const libc::c_char,
        b"^date_signed\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^sc\0" as *const u8 as *const libc::c_char,
        b"^deposit_acct_auth_date_presidential\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^sc\0" as *const u8 as *const libc::c_char,
        b"^established_date\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^sc\0" as *const u8 as *const libc::c_char,
        b"^loan_amount_original\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^sc\0" as *const u8 as *const libc::c_char,
        b"^loan_balance\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^sc\0" as *const u8 as *const libc::c_char,
        b"^loan_incurred_date\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^sc\0" as *const u8 as *const libc::c_char,
        b"^loan_payment_to_date\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^sd\0" as *const u8 as *const libc::c_char,
        b"^balance_at_close_this_period\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^sd\0" as *const u8 as *const libc::c_char,
        b"^beginning_balance_this_period\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^sd\0" as *const u8 as *const libc::c_char,
        b"^incurred_amount_this_period\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^sd\0" as *const u8 as *const libc::c_char,
        b"^payment_amount_this_period\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^se\0" as *const u8 as *const libc::c_char,
        b"^calendar_y_t_d_per_election_office\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^se\0" as *const u8 as *const libc::c_char,
        b"^date_signed\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^se\0" as *const u8 as *const libc::c_char,
        b"^disbursement_date\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^se\0" as *const u8 as *const libc::c_char,
        b"^dissemination_date\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^se\0" as *const u8 as *const libc::c_char,
        b"^expenditure_amount\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^sf\0" as *const u8 as *const libc::c_char,
        b"^expenditure_date\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^sf\0" as *const u8 as *const libc::c_char,
        b"^expenditure_amount\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^sf\0" as *const u8 as *const libc::c_char,
        b"^aggregate_general_elec_expended\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^si\0" as *const u8 as *const libc::c_char,
        b"^col_\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^si\0" as *const u8 as *const libc::c_char,
        b"^coverage_from_date\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^si\0" as *const u8 as *const libc::c_char,
        b"^coverage_through_date\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^sl\0" as *const u8 as *const libc::c_char,
        b"^col_\0" as *const u8 as *const libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^sl\0" as *const u8 as *const libc::c_char,
        b"^coverage_from_date\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
    [
        b".*\0" as *const u8 as *const libc::c_char,
        b"^sl\0" as *const u8 as *const libc::c_char,
        b"^coverage_through_date\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    ],
];
static mut numHeaders: libc::c_int = 0;
static mut numTypes: libc::c_int = 0;
#[no_mangle]
pub static mut DEFAULT_STRING_SIZE: size_t = 256 as libc::c_int as size_t;
#[no_mangle]
pub unsafe extern "C" fn newString(mut size: size_t) -> *mut STRING {
    let mut s: *mut STRING = malloc(::std::mem::size_of::<STRING>() as libc::c_ulong)
        as *mut STRING;
    let ref mut fresh0 = (*s).str_0;
    *fresh0 = malloc(size) as *mut libc::c_char;
    (*s).n = size;
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn fromString(mut str: *const libc::c_char) -> *mut STRING {
    let mut s: *mut STRING = newString(
        (strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    strcpy((*s).str_0, str);
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn setString(mut s: *mut STRING, mut str: *const libc::c_char) {
    growStringTo(s, (strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong));
    strcpy((*s).str_0, str);
}
#[no_mangle]
pub unsafe extern "C" fn freeString(mut s: *mut STRING) {
    free((*s).str_0 as *mut libc::c_void);
    free(s as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn growStringTo(
    mut str: *mut STRING,
    mut newSize: size_t,
) -> libc::c_int {
    if newSize > (*str).n {
        (*str).n = newSize;
    } else {
        return 1 as libc::c_int
    }
    let ref mut fresh1 = (*str).str_0;
    *fresh1 = realloc((*str).str_0 as *mut libc::c_void, (*str).n) as *mut libc::c_char;
    if ((*str).str_0).is_null() {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn growString(mut str: *mut STRING) -> libc::c_int {
    return growStringTo(str, ((*str).n).wrapping_mul(2 as libc::c_int as libc::c_ulong));
}
#[no_mangle]
pub unsafe extern "C" fn copyString(mut src: *mut STRING, mut dst: *mut STRING) {
    growStringTo(dst, ((*src).n).wrapping_add(1 as libc::c_int as libc::c_ulong));
    strcpy((*dst).str_0, (*src).str_0);
}
#[no_mangle]
pub unsafe extern "C" fn newPersistentMemoryContext() -> *mut PERSISTENT_MEMORY_CONTEXT {
    let mut ctx: *mut PERSISTENT_MEMORY_CONTEXT = malloc(
        ::std::mem::size_of::<PERSISTENT_MEMORY_CONTEXT>() as libc::c_ulong,
    ) as *mut PERSISTENT_MEMORY_CONTEXT;
    let ref mut fresh2 = (*ctx).rawLine;
    *fresh2 = newString(DEFAULT_STRING_SIZE);
    let ref mut fresh3 = (*ctx).line;
    *fresh3 = newString(DEFAULT_STRING_SIZE);
    let ref mut fresh4 = (*ctx).bufferLine;
    *fresh4 = newString(DEFAULT_STRING_SIZE);
    let ref mut fresh5 = (*ctx).headerVersions;
    *fresh5 = malloc(
        (::std::mem::size_of::<*mut pcre>() as libc::c_ulong)
            .wrapping_mul(numHeaders as libc::c_ulong),
    ) as *mut *mut pcre;
    let ref mut fresh6 = (*ctx).headerFormTypes;
    *fresh6 = malloc(
        (::std::mem::size_of::<*mut pcre>() as libc::c_ulong)
            .wrapping_mul(numHeaders as libc::c_ulong),
    ) as *mut *mut pcre;
    let ref mut fresh7 = (*ctx).typeVersions;
    *fresh7 = malloc(
        (::std::mem::size_of::<*mut pcre>() as libc::c_ulong)
            .wrapping_mul(numTypes as libc::c_ulong),
    ) as *mut *mut pcre;
    let ref mut fresh8 = (*ctx).typeFormTypes;
    *fresh8 = malloc(
        (::std::mem::size_of::<*mut pcre>() as libc::c_ulong)
            .wrapping_mul(numTypes as libc::c_ulong),
    ) as *mut *mut pcre;
    let ref mut fresh9 = (*ctx).typeHeaders;
    *fresh9 = malloc(
        (::std::mem::size_of::<*mut pcre>() as libc::c_ulong)
            .wrapping_mul(numTypes as libc::c_ulong),
    ) as *mut *mut pcre;
    let mut error: *const libc::c_char = 0 as *const libc::c_char;
    let mut errorOffset: libc::c_int = 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numHeaders {
        let ref mut fresh10 = *((*ctx).headerVersions).offset(i as isize);
        *fresh10 = pcre_compile(
            headers[i as usize][0 as libc::c_int as usize],
            0x1 as libc::c_int,
            &mut error,
            &mut errorOffset,
            0 as *const libc::c_uchar,
        );
        if (*((*ctx).headerVersions).offset(i as isize)).is_null() {
            fprintf(
                stderr,
                b"PCRE header version compilation for \"%s\" failed at offset %d: %s\n\0"
                    as *const u8 as *const libc::c_char,
                headers[i as usize][0 as libc::c_int as usize],
                errorOffset,
                error,
            );
            exit(1 as libc::c_int);
        }
        let ref mut fresh11 = *((*ctx).headerFormTypes).offset(i as isize);
        *fresh11 = pcre_compile(
            headers[i as usize][1 as libc::c_int as usize],
            0x1 as libc::c_int,
            &mut error,
            &mut errorOffset,
            0 as *const libc::c_uchar,
        );
        if (*((*ctx).headerFormTypes).offset(i as isize)).is_null() {
            fprintf(
                stderr,
                b"PCRE header form type compilation for \"%s\" failed at offset %d: %s\n\0"
                    as *const u8 as *const libc::c_char,
                headers[i as usize][1 as libc::c_int as usize],
                errorOffset,
                error,
            );
            exit(1 as libc::c_int);
        }
        i += 1;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < numTypes {
        let ref mut fresh12 = *((*ctx).typeVersions).offset(i_0 as isize);
        *fresh12 = pcre_compile(
            types[i_0 as usize][0 as libc::c_int as usize],
            0x1 as libc::c_int,
            &mut error,
            &mut errorOffset,
            0 as *const libc::c_uchar,
        );
        if (*((*ctx).typeVersions).offset(i_0 as isize)).is_null() {
            fprintf(
                stderr,
                b"PCRE type version compilation for \"%s\" failed at offset %d: %s\n\0"
                    as *const u8 as *const libc::c_char,
                types[i_0 as usize][0 as libc::c_int as usize],
                errorOffset,
                error,
            );
            exit(1 as libc::c_int);
        }
        let ref mut fresh13 = *((*ctx).typeFormTypes).offset(i_0 as isize);
        *fresh13 = pcre_compile(
            types[i_0 as usize][1 as libc::c_int as usize],
            0x1 as libc::c_int,
            &mut error,
            &mut errorOffset,
            0 as *const libc::c_uchar,
        );
        if (*((*ctx).typeFormTypes).offset(i_0 as isize)).is_null() {
            fprintf(
                stderr,
                b"PCRE type form type compilation for \"%s\" failed at offset %d: %s\n\0"
                    as *const u8 as *const libc::c_char,
                types[i_0 as usize][1 as libc::c_int as usize],
                errorOffset,
                error,
            );
            exit(1 as libc::c_int);
        }
        let ref mut fresh14 = *((*ctx).typeHeaders).offset(i_0 as isize);
        *fresh14 = pcre_compile(
            types[i_0 as usize][2 as libc::c_int as usize],
            0x1 as libc::c_int,
            &mut error,
            &mut errorOffset,
            0 as *const libc::c_uchar,
        );
        if (*((*ctx).typeHeaders).offset(i_0 as isize)).is_null() {
            fprintf(
                stderr,
                b"PCRE type header compilation for \"%s\" failed at offset %d: %s\n\0"
                    as *const u8 as *const libc::c_char,
                types[i_0 as usize][2 as libc::c_int as usize],
                errorOffset,
                error,
            );
            exit(1 as libc::c_int);
        }
        i_0 += 1;
    }
    return ctx;
}
#[no_mangle]
pub unsafe extern "C" fn freePersistentMemoryContext(
    mut context: *mut PERSISTENT_MEMORY_CONTEXT,
) {
    freeString((*context).rawLine);
    freeString((*context).line);
    freeString((*context).bufferLine);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numHeaders {
        pcre_free
            .expect(
                "non-null function pointer",
            )(*((*context).headerVersions).offset(i as isize) as *mut libc::c_void);
        pcre_free
            .expect(
                "non-null function pointer",
            )(*((*context).headerFormTypes).offset(i as isize) as *mut libc::c_void);
        i += 1;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < numTypes {
        pcre_free
            .expect(
                "non-null function pointer",
            )(*((*context).typeVersions).offset(i_0 as isize) as *mut libc::c_void);
        pcre_free
            .expect(
                "non-null function pointer",
            )(*((*context).typeFormTypes).offset(i_0 as isize) as *mut libc::c_void);
        pcre_free
            .expect(
                "non-null function pointer",
            )(*((*context).typeHeaders).offset(i_0 as isize) as *mut libc::c_void);
        i_0 += 1;
    }
    free((*context).headerVersions as *mut libc::c_void);
    free((*context).headerFormTypes as *mut libc::c_void);
    free((*context).typeVersions as *mut libc::c_void);
    free((*context).typeFormTypes as *mut libc::c_void);
    free((*context).typeHeaders as *mut libc::c_void);
    free(context as *mut libc::c_void);
}
unsafe extern "C" fn run_static_initializers() {
    numHeaders = (::std::mem::size_of::<[[*const libc::c_char; 3]; 282]>()
        as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<[*const libc::c_char; 3]>() as libc::c_ulong)
        as libc::c_int;
    numTypes = (::std::mem::size_of::<[[*const libc::c_char; 4]; 102]>()
        as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<[*const libc::c_char; 4]>() as libc::c_ulong)
        as libc::c_int;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
