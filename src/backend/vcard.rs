// Copyright 2025 Titouan Real <titouan.real@gmail.com>
// SPDX-License-Identifier: GPL-3.0

use super::StorageBackend;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{create_dir, File},
    io::{Read, Write},
    path::Path,
};

#[derive(Serialize, Deserialize)]
pub struct Backend {
    contacts: HashMap<u64, String>,
}

impl StorageBackend for Backend {
    fn new() -> Self {
        if !Path::new("data").exists() {
            create_dir("data").unwrap();
        }

        if !Path::new("data/contacts.json").exists() {
            let _ = Self {
                contacts: {
                    let mut contacts = HashMap::new();

                    contacts.insert(0, "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Frank\r\nTEL:+33 0 12 34 56 78\r\nEMAIL:frank@gmail.com\r\nREV:20250121T173505Z\r\nEND:VCARD".to_owned());

                    contacts.insert(1, "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Bob\r\nTEL:+32 0 12 34 56 78\r\nTEL:+31 0 12 34 56 78\r\nREV:20250121T173505Z\r\nEND:VCARD".to_owned());

                    contacts.insert(2, "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Leopold\r\nEMAIL:leopold@gmail.com\r\nEMAIL:leopold@outlook.com\r\nREV:20250121T173505Z\r\nEND:VCARD\r\n".to_owned());

                    contacts
                },
            };
        }

        let mut buf = String::new();

        File::read_to_string(&mut File::open("data/contacts.json").unwrap(), &mut buf).unwrap();

        serde_json::from_str(&buf).unwrap()
    }

    fn get_contacts(&self) -> Vec<(u64, String)> {
        self.contacts
            .iter()
            .map(|(id, vcard)| (*id, vcard.to_owned()))
            .collect()
    }

    fn add_contact(&mut self, vcard: String) {
        let mut id = rand::random();

        while self.contacts.contains_key(&id) {
            id = rand::random();
        }

        // TODO: Add vcard validation

        self.contacts.insert(id, vcard);
    }

    fn remove_contact(&mut self, id: u64) {
        self.contacts.remove(&id);
    }
}

impl Drop for Backend {
    fn drop(&mut self) {
        let json = serde_json::to_string(&self).unwrap();

        let mut out = File::create("data/contacts.json").unwrap();

        out.write(json.as_bytes()).unwrap();
    }
}
