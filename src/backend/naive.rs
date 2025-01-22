// Copyright 2025 Titouan Real <titouan.real@gmail.com>
// SPDX-License-Identifier: GPL-3.0
//
use super::StorageBackend;
use std::{
    collections::{HashMap, HashSet},
    io::BufReader,
};
use vcard::{
    properties::{Email, Telephone},
    values::{email_value::EmailValue, text::Text},
    Set, VCard,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Mail {
    pub address: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Phone {
    pub number: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Contact {
    pub name: String,
    pub mails: Vec<Mail>,
    pub phones: Vec<Phone>,
}

pub struct Backend {
    contacts: HashMap<u64, Contact>,
}

impl StorageBackend for Backend {
    fn new() -> Self {
        Self {
            contacts: {
                let mut contacts = HashMap::new();

                contacts.insert(
                    0,
                    Contact {
                        name: "Leopold".to_owned(),
                        mails: vec![
                            Mail {
                                address: "leopold@gmail.com".to_owned(),
                            },
                            Mail {
                                address: "leopold@outlook.com".to_owned(),
                            },
                        ],
                        phones: vec![],
                    },
                );

                contacts.insert(
                    1,
                    Contact {
                        name: "Frank".to_owned(),
                        mails: vec![Mail {
                            address: "frank@gmail.com".to_owned(),
                        }],
                        phones: vec![Phone {
                            number: "+33 0 12 34 56 78".to_owned(),
                        }],
                    },
                );

                contacts.insert(
                    2,
                    Contact {
                        name: "Bob".to_owned(),
                        mails: vec![],
                        phones: vec![
                            Phone {
                                number: "+31 0 12 34 56 78".to_owned(),
                            },
                            Phone {
                                number: "+32 0 12 34 56 78".to_owned(),
                            },
                        ],
                    },
                );

                contacts
            },
        }
    }

    fn get_contacts(&self) -> Vec<(u64, String)> {
        self.contacts
            .iter()
            .map(|(id, contact)| {
                let mut vcard = VCard::from_formatted_name_str(&contact.name).unwrap();

                let emails = {
                    let mut emails = HashSet::new();

                    for mail in contact.mails.iter() {
                        emails.insert(Email::from_email_value(
                            EmailValue::from_str(&mail.address).unwrap(),
                        ));
                    }

                    emails
                };

                if !emails.is_empty() {
                    vcard.emails = Some(Set::from_hash_set(emails).unwrap());
                };

                let telephones = {
                    let mut telephones = HashSet::new();

                    for phone in contact.phones.iter() {
                        telephones
                            .insert(Telephone::from_text(Text::from_str(&phone.number).unwrap()));
                    }

                    telephones
                };

                if !telephones.is_empty() {
                    vcard.telephones = Some(Set::from_hash_set(telephones).unwrap());
                }

                (*id, format!("{vcard}"))
            })
            .collect()
    }

    fn add_contact(&mut self, vcard: String) {
        let buf = BufReader::new(vcard.as_bytes());

        let mut reader = ical::VcardParser::new(buf);

        // Read only the first contact of the vCard string passed.
        match reader.next() {
            None => {
                return;
            }
            Some(Err(_)) => return,
            Some(Ok(data)) => {
                let version = data
                    .properties
                    .iter()
                    .find(|property| property.name == "VERSION")
                    .unwrap()
                    .value
                    .as_ref()
                    .unwrap()
                    .to_owned();

                if version != "4.0" {
                    return;
                }

                let name = data
                    .properties
                    .iter()
                    .find(|property| property.name == "FN")
                    .unwrap()
                    .value
                    .as_ref()
                    .unwrap()
                    .to_owned();

                let mails = data
                    .properties
                    .iter()
                    .filter(|property| property.name == "EMAIL")
                    .map(|property| Mail {
                        address: property.value.as_ref().unwrap().to_owned(),
                    })
                    .collect();

                let phones = data
                    .properties
                    .iter()
                    .filter(|property| property.name == "TEL")
                    .map(|property| Phone {
                        number: property.value.as_ref().unwrap().to_owned(),
                    })
                    .collect();

                let mut id = rand::random();

                while self.contacts.contains_key(&id) {
                    id = rand::random();
                }

                self.contacts.insert(
                    id,
                    Contact {
                        name,
                        mails,
                        phones,
                    },
                );
            }
        }
    }

    fn remove_contact(&mut self, id: u64) {
        self.contacts.remove(&id);
    }
}
