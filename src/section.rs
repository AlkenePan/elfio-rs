/*
Copyright (C) 2001-present by Serge Lamikhov-Center

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

use super::utils::*;
use super::*;

// --------------------------------------------------------------------------
pub trait ElfSectionAccessTrait {
    ELFIO_GET_SET_ACCESS_DECL!(String, name);
    ELFIO_GET_SET_ACCESS_DECL!(ElfWord, type);
    ELFIO_GET_SET_ACCESS_DECL!(ElfXword, flags);
    ELFIO_GET_SET_ACCESS_DECL!(ElfWord, info);
    ELFIO_GET_SET_ACCESS_DECL!(ElfWord, link);
    ELFIO_GET_SET_ACCESS_DECL!(ElfXword, addr_align);
    ELFIO_GET_SET_ACCESS_DECL!(ElfXword, entry_size);
    ELFIO_GET_SET_ACCESS_DECL!(Elf64Addr, address);
    ELFIO_GET_SET_ACCESS_DECL!(ElfXword, size);
    ELFIO_GET_SET_ACCESS_DECL!(ElfWord, name_string_offset);
    ELFIO_GET_SET_ACCESS_DECL!(Elf64Off, offset);

    fn set_converter(&mut self, converter: &Converter);
}

// --------------------------------------------------------------------------
pub trait ElfSectionTrait: ElfSectionAccessTrait + Load {}

// --------------------------------------------------------------------------
// ELF file header
#[repr(C)]
#[derive(Debug)]
pub struct ElfSection<Addr, Offset> {
    sh_name: String,
    sh_type: ElfWord,
    sh_flags: ElfWord,
    sh_addr: Addr,
    sh_offset: Offset,
    sh_size: ElfWord,
    sh_link: ElfWord,
    sh_info: ElfWord,
    sh_addralign: ElfWord,
    sh_entsize: ElfWord,

    converter: Converter,
}
