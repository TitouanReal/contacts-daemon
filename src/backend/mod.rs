// Copyright 2025 Titouan Real <titouan.real@gmail.com>
// SPDX-License-Identifier: GPL-3.0

mod naive;
mod polars;
mod vcard;

pub type Backend = vcard::Backend;

pub trait StorageBackend {
    fn new() -> Self;
    fn get_contacts(&self) -> Vec<(u64, String)>;
    fn add_contact(&mut self, vcard: String);
    fn remove_contact(&mut self, id: u64);
}
