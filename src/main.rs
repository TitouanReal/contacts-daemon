// Copyright 2025 Titouan Real <titouan.real@gmail.com>
// SPDX-License-Identifier: GPL-3.0

mod backend;

use backend::{Backend, StorageBackend};
use std::error::Error;
use tokio::signal;
use zbus::{connection, interface};

struct ContactsManager {
    backend: Backend,
}

#[interface(name = "com.github.TitouanReal.ContactsDaemon")]
impl ContactsManager {
    fn get_contacts(&mut self) -> Vec<(u64, String)> {
        self.backend.get_contacts()
    }

    fn add_contact(&mut self, vcard: String) {
        self.backend.add_contact(vcard);
    }

    fn remove_contact(&mut self, id: u64) {
        self.backend.remove_contact(id);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let manager = ContactsManager {
        backend: Backend::new(),
    };

    let _conn = connection::Builder::session()?
        .name("com.github.TitouanReal.ContactsDaemon")?
        .serve_at("/com/github/TitouanReal/ContactsDaemon", manager)?
        .build()
        .await?;

    let int = signal::unix::SignalKind::interrupt();
    let mut int = signal::unix::signal(int)?;
    let term = signal::unix::SignalKind::terminate();
    let mut term = signal::unix::signal(term)?;

    tokio::select! {
        _ = int.recv() => {},
        _ = term.recv() => {},
    }

    Ok(())
}
