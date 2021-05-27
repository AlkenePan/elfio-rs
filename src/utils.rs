/*
Copyright (C) 2021-present by Serge Lamikhov-Center

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
*/

// use std::io;
// use std::io::prelude::*;
// use std::slice;

// pub fn read_struct<T, R: Read>(reader: &mut R) -> io::Result<T> {
//     let num_bytes = ::std::mem::size_of::<T>();
//     unsafe {
//         let mut mem = ::std::mem::MaybeUninit::uninit().assume_init();
//         let ptr = slice::from_raw_parts_mut(&mut mem as *mut T as *mut u8, num_bytes);
//         match reader.read_exact(ptr) {
//             Ok(()) => Ok(mem),
//             Err(e) => {
//                 ::std::mem::forget(mem);
//                 Err(e)
//             }
//         }
//     }
// }
