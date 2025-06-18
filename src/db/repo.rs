use async_trait::async_trait;
use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, from_document, oid::ObjectId, to_document, Document},
    error::{Error, Result},
    Collection,
};
use serde::{de::DeserializeOwned, Serialize};

#[async_trait]
pub trait MongoRepository<T, I>
where
    T: Serialize + DeserializeOwned + Unpin + Send + Sync,
    I: Send + Sync,
{
    fn collection(&self) -> &Collection<Document>;
    fn to_entity(&self, input: &I) -> T;

    async fn create(&self, input: &I) -> Result<T> {
        let entity = self.to_entity(input);
        let mut doc = to_document(&entity)?;
        doc.remove("_id");

        let result = self.collection().insert_one(doc.clone()).await?;
        let inserted_id = result
            .inserted_id
            .as_object_id()
            .ok_or_else(|| Error::custom("Inserted ID is not ObjectId"))?;

        let inserted_doc = self
            .collection()
            .find_one(doc! { "_id": inserted_id })
            .await?
            .ok_or_else(|| Error::custom("Inserted doc not found"))?;

        Ok(from_document(inserted_doc)?)
    }

    async fn find_all(&self) -> Result<Vec<T>> {
        let mut cursor = self.collection().find(doc! {}).await?;
        let mut results = Vec::new();
        while let Some(doc) = cursor.next().await {
            results.push(from_document(doc?)?);
        }
        Ok(results)
    }

    async fn find_one(&self, filter: Document) -> Result<Option<T>> {
        if let Some(doc) = self.collection().find_one(filter).await? {
            Ok(Some(from_document(doc)?))
        } else {
            Ok(None)
        }
    }

    async fn find_by_id(&self, id: ObjectId) -> Result<Option<T>> {
        self.find_one(doc! { "_id": id }).await
    }

    async fn update(&self, id: ObjectId, update: &T) -> Result<T> {
        let mut update_doc = to_document(update)?;
        update_doc.remove("_id"); // never update the ID

        self.collection()
            .update_one(doc! { "_id": id }, doc! { "$set": update_doc })
            .await?;

        let updated_doc = self
            .collection()
            .find_one(doc! { "_id": id })
            .await?
            .ok_or_else(|| Error::custom("Updated document not found"))?;

        Ok(from_document(updated_doc)?)
    }

    async fn delete(&self, id: ObjectId) -> Result<bool> {
        let result = self.collection().delete_one(doc! { "_id": id }).await?;

        Ok(result.deleted_count > 0)
    }
}
