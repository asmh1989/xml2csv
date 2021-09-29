use std::{sync::Arc, time::Duration};

use log::info;

use mongodb::{
    bson::{self, doc, Bson, Document},
    error::Error,
    options::{ClientOptions, FindOneOptions, FindOptions},
    sync::Client,
};
use once_cell::sync::OnceCell;
use serde::de::DeserializeOwned;

static INSTANCE: OnceCell<Arc<Client>> = OnceCell::new();

const TABLE_NAME: &'static str = "drugbank";
pub const COLLECTION_CID_NOT_FOUND: &'static str = "cid_not_found";

const KEY_UPDATE_TIME: &'static str = "updateTime";
const KEY_CREATE_TIME: &'static str = "createTime";

#[derive(Clone, Debug)]
pub struct Db;

impl Db {
    pub fn get_instance() -> &'static Arc<Client> {
        INSTANCE.get().expect("db need init first")
    }

    pub fn find<T>(
        table: &str,
        filter: impl Into<Option<Document>>,
        options: impl Into<Option<FindOptions>>,
        call_back: &dyn Fn(T),
    ) -> Result<(), Error>
    where
        T: DeserializeOwned,
    {
        let client = Db::get_instance();
        let db = client.database(TABLE_NAME);
        let collection = db.collection(table);

        let mut cursor = collection.find(filter, options)?;

        // Iterate over the results of the cursor.
        while let Some(result) = cursor.next() {
            match result {
                Ok(document) => {
                    let result = bson::from_bson::<T>(Bson::Document(document));
                    match result {
                        Ok(app) => call_back(app),
                        Err(err) => {
                            info!("err = {:?}", err);
                        }
                    }
                }
                Err(e) => {
                    info!("error = {:?}", e);
                    return Err(e.into());
                }
            }
        }

        Ok(())
    }

    pub fn find_one(
        table: &str,
        filter: impl Into<Option<Document>>,
        options: impl Into<Option<FindOneOptions>>,
    ) -> Result<Option<Document>, Error> {
        let client = Db::get_instance();
        let db = client.database(TABLE_NAME);
        let collection = db.collection(table);

        collection.find_one(filter, options)
    }

    pub fn insert_many(table: &str, data: Vec<Document>) -> Result<(), Error> {
        let client = Db::get_instance();
        let db = client.database(TABLE_NAME);
        let collection = db.collection(table);
        let date = Bson::DateTime(mongodb::bson::DateTime::now());
        let data2: Vec<Document> = data
            .clone()
            .iter_mut()
            .map(|f| {
                f.insert(KEY_UPDATE_TIME, date.clone());
                f.insert(KEY_CREATE_TIME, date.clone());
                f.to_owned()
            })
            .collect();

        let _result = collection.insert_many(data2, None)?;

        Ok(())
    }

    pub fn delete_table(table: &str) -> Result<(), Error> {
        let client = Db::get_instance();
        let db = client.database(TABLE_NAME);
        let collection = db.collection::<Document>(table);
        let _ = collection.drop(None)?;
        Ok(())
    }

    pub fn save(table: &str, filter: Document, app: Document) -> Result<(), Error> {
        let client = Db::get_instance();
        let db = client.database(TABLE_NAME);
        let collection = db.collection(table);

        let mut update_doc = app;
        let date = Bson::DateTime(mongodb::bson::DateTime::now());
        update_doc.insert(KEY_UPDATE_TIME, date.clone());

        let result = collection.find_one_and_update(
            filter.clone(),
            doc! {"$set": update_doc.clone()},
            None,
        )?;

        if !result.is_none() {
            info!("db update");
            collection.update_one(filter.clone(), doc! {"$set": update_doc}, None)?;
        } else {
            update_doc.insert(KEY_CREATE_TIME, date);
            let result = collection.insert_one(update_doc, None)?;

            info!("db insert {:?}", result);
        }

        Ok(())
    }

    pub fn delete(table: &str, filter: Document) -> Result<(), Error> {
        let client = Db::get_instance();
        let db = client.database(TABLE_NAME);
        let collection = db.collection::<Document>(table);

        let result = collection.delete_one(filter, None)?;

        info!("db delete {:?}", result);

        Ok(())
    }

    pub fn contians(table: &str, filter: Document) -> bool {
        let client = Db::get_instance();
        let db = client.database(TABLE_NAME);
        let collection = db.collection::<Document>(table);

        let result = collection.count_documents(filter, None);

        match result {
            Ok(d) => d > 0,
            Err(_) => false,
        }
    }

    pub fn count(table: &str, filter: Document) -> u64 {
        let client = Db::get_instance();
        let db = client.database(TABLE_NAME);
        let collection = db.collection::<Document>(table);

        let result = collection.count_documents(filter, None);

        match result {
            Ok(d) => d,
            Err(_) => 0,
        }
    }
}

pub fn init_db(url: &str) {
    if INSTANCE.get().is_some() {
        return;
    }
    let mut client_options = ClientOptions::parse(url).unwrap();
    client_options.connect_timeout = Some(Duration::new(4, 0));
    // 选择超时
    client_options.server_selection_timeout = Some(Duration::new(8, 0));

    INSTANCE
        .set(Arc::new(Client::with_options(client_options).unwrap()))
        .expect("db init error");
}
