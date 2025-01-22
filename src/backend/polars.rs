// Copyright 2025 Titouan Real <titouan.real@gmail.com>
// SPDX-License-Identifier: GPL-3.0

use super::StorageBackend;
use polars::{
    df,
    frame::DataFrame,
    io::SerReader,
    prelude::{AnyValue, NamedFrom, ParquetReader, ParquetWriter},
    series::Series,
};
use std::{collections::HashSet, fs::File, io::BufReader, path::Path};
use vcard::{
    properties::{Email, Telephone},
    values::{email_value::EmailValue, text::Text},
    Set, VCard,
};

pub struct Backend {
    df: DataFrame,
}

impl StorageBackend for Backend {
    fn new() -> Self {
        Self {
            df: {
                if !Path::new("data/contacts.parquet").exists() {
                    let mut df: DataFrame = df!(
                        "id" => [1, 2, 3],
                        "name" => ["Leopold", "Frank", "Bob"],
                        "mails" => [
                            Series::new(
                                "".into(),
                                ["leopold@gmail.com", "leopold@outlook.com"],
                            ),
                            Series::new(
                                "".into(),
                                ["frank@gmail.com"],
                            ),
                            <Series as NamedFrom<&[&str], _>>::new(
                                "".into(),
                                &[],
                            ),
                        ],
                        "phones" => [
                            <Series as NamedFrom<&[&str], _>>::new(
                                "".into(),
                                &[],
                            ),
                            Series::new(
                                "".into(),
                                ["+33 0 12 34 56 78"],
                            ),
                            Series::new(
                                "".into(),
                                ["+31 0 12 34 56 78", "+32 0 12 34 56 78"],
                            ),
                        ]
                    )
                    .unwrap();

                    ParquetWriter::new(File::create("data/contacts.parquet").unwrap())
                        .finish(&mut df)
                        .unwrap();
                }

                ParquetReader::new(File::open("data/contacts.parquet").unwrap())
                    .finish()
                    .unwrap()
            },
        }
    }

    fn get_contacts(&self) -> Vec<(u64, String)> {
        let mut contacts = Vec::new();

        let mut i = 0;
        while let Ok(row) = self.df.get_row(i) {
            let AnyValue::String(name) = row.0[1] else {
                panic!()
            };
            let AnyValue::List(ref mails) = row.0[2] else {
                panic!()
            };
            let AnyValue::List(ref phones) = row.0[3] else {
                panic!()
            };

            let mut vcard = VCard::from_formatted_name_str(name).unwrap();

            let emails = {
                let mut emails = HashSet::new();

                for address in mails.str().unwrap().into_iter() {
                    emails.insert(Email::from_email_value(
                        EmailValue::from_str(address.unwrap()).unwrap(),
                    ));
                }

                emails
            };

            if !emails.is_empty() {
                vcard.emails = Some(Set::from_hash_set(emails).unwrap());
            };

            let telephones = {
                let mut telephones = HashSet::new();

                for number in phones.str().unwrap().into_iter() {
                    telephones.insert(Telephone::from_text(
                        Text::from_str(number.unwrap()).unwrap(),
                    ));
                }

                telephones
            };

            if !telephones.is_empty() {
                vcard.telephones = Some(Set::from_hash_set(telephones).unwrap());
            }

            contacts.push((row.0[0].try_extract().unwrap(), format!("{vcard}")));

            i += 1;
        }

        contacts
    }

    fn add_contact(&mut self, vcard: String) {
        let buf = BufReader::new(vcard.as_bytes());

        let mut reader = ical::VcardParser::new(buf);

        // Read only the first contact of the vCard string passed.
        match reader.next() {
            None => {
                return;
            }
            Some(Err(_)) => {
                return;
            }
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

                let _name = data
                    .properties
                    .iter()
                    .find(|property| property.name == "FN")
                    .unwrap()
                    .value
                    .as_ref()
                    .unwrap()
                    .to_owned();

                // let mails = data
                //     .properties
                //     .iter()
                //     .filter(|property| property.name == "EMAIL")
                //     .map(|property| property.value.as_ref().unwrap().to_owned())
                //     .collect();

                // let phones = data
                //     .properties
                //     .iter()
                //     .filter(|property| property.name == "TEL")
                //     .map(|property| property.value.as_ref().unwrap().to_owned())
                //     .collect();
            }
        }
    }

    fn remove_contact(&mut self, _id: u64) {}
}
