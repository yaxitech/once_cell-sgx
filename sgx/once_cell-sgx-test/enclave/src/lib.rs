// Copyright (C) 2017-2018 Baidu, Inc. All Rights Reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
//  * Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
//  * Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in
//    the documentation and/or other materials provided with the
//    distribution.
//  * Neither the name of Baidu, Inc., nor the names of its
//    contributors may be used to endorse or promote products derived
//    from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#![crate_name = "helloworldsampleenclave"]
#![crate_type = "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

#[cfg(target_env = "sgx")]
extern crate core;

extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;
extern crate sgx_tunittest;

use sgx_types::*;
use std::string::String;
use std::vec::Vec;
use std::io::{self, Write};
use std::slice;
use sgx_tunittest::*;

extern crate once_cell;

mod tests;

#[no_mangle]
pub extern "C" fn say_something(some_string: *const u8, some_len: usize) -> sgx_status_t {

    let str_slice = unsafe { slice::from_raw_parts(some_string, some_len) };
    let _ = io::stdout().write(str_slice);

    // A sample &'static string
    let rust_raw_string = "This is a in-Enclave ";
    // An array
    let word:[u8;4] = [82, 117, 115, 116];
    // An vector
    let word_vec:Vec<u8> = vec![32, 115, 116, 114, 105, 110, 103, 33];

    // Construct a string from &'static string
    let mut hello_string = String::from(rust_raw_string);

    // Iterate on word array
    for c in word.iter() {
        hello_string.push(*c as char);
    }

    // Rust style convertion
    hello_string += String::from_utf8(word_vec).expect("Invalid UTF-8")
                                               .as_str();

    // Ocall to normal world for output
    println!("{}", &hello_string);

    rsgx_unit_tests!(
tests::unsync::once_cell,
tests::unsync::once_cell_get_mut,
tests::unsync::once_cell_drop,
tests::unsync::unsync_once_cell_drop_empty,
tests::unsync::clone,
tests::unsync::from_impl,
tests::unsync::partialeq_impl,
tests::unsync::into_inner,
tests::unsync::debug_impl,
tests::unsync::lazy_new,
tests::unsync::lazy_deref_mut,
tests::unsync::lazy_default,
tests::unsync::lazy_poisoning,
tests::unsync::aliasing_in_get,
|| should_panic!(tests::unsync::reentrant_init()),
tests::sync::once_cell,
tests::sync::once_cell_get_mut,
tests::sync::once_cell_get_unchecked,
tests::sync::once_cell_drop,
tests::sync::once_cell_drop_empty,
tests::sync::clone,
tests::sync::get_or_try_init,
tests::sync::from_impl,
tests::sync::partialeq_impl,
tests::sync::into_inner,
tests::sync::debug_impl,
tests::sync::lazy_new,
tests::sync::lazy_deref_mut,
tests::sync::lazy_default,
tests::sync::static_lazy,
tests::sync::static_lazy_via_fn,
tests::sync::lazy_poisoning,
tests::sync::once_cell_is_sync_send,
tests::sync::eval_once_macro,
);
    sgx_status_t::SGX_SUCCESS
}
