use std::os::raw::c_char;

// NOTE(bryce): These match types in C. It should not be used outside of platform code. Thus for
//  constancy and ease of reading for those with C experience, I will keep the traditional name even
//  though it breaks Rust convention

#[allow(non_camel_case_types)]
pub type sa_family_t = u8;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct sockaddr {
    pub sa_len: u8,
    pub sa_family: sa_family_t,
    pub sa_data: [c_char; 14],
}
