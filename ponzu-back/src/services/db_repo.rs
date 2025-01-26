use crate::models::app_error::AppError;
use crate::utils::bson::get_object_id;
use futures::TryStreamExt;
use mongodb::bson::{doc, Document};
use mongodb::error::Error;
use mongodb::options::{AggregateOptions, FindOptions, UpdateModifications};
use mongodb::results::DeleteResult;
use mongodb::Collection;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// A generic repository for interacting with a MongoDB collection.
///
/// The `DatabaseRepository` struct provides methods to perform common database operations
/// such as finding, inserting, updating, and deleting documents in a MongoDB collection.
///
/// # Type Parameters
/// - `T`: The type of documents stored in the collection. This type must implement
///        `Send`, `Sync`, `DeserializeOwned`, and `Serialize` traits.
pub struct DatabaseRepository<T: Send + Sync + DeserializeOwned + Serialize> {
    collection: Collection<T>,
}

impl<T: Send + Sync + DeserializeOwned + Serialize> DatabaseRepository<T> {
    /// Creates a new instance of `DatabaseRepository`.
    ///
    /// # Parameters
    /// - `collection`: A MongoDB collection of type `T`.
    ///
    /// # Returns
    /// A new `DatabaseRepository` instance.
    pub fn new(collection: Collection<T>) -> Self {
        DatabaseRepository { collection }
    }

    /// Finds documents in the collection that match the provided filter.
    ///
    /// # Parameters
    /// - `filter`: A MongoDB document specifying the query criteria.
    /// - `options`: Optional `FindOptions` to configure the find operation.
    ///
    /// # Returns
    /// A `Result` containing a `Vec<T>` of documents if successful, or an `AppError` if the operation fails.
    pub async fn find(
        &self,
        filter: Option<Document>,
        options: Option<FindOptions>,
    ) -> Result<Vec<T>, AppError> {
        // Execute the query
        self.collection
            .find(filter.unwrap_or(doc! {}))
            .with_options(options)
            .await?
            // Collect the results into a Vec<T>
            .try_collect()
            .await
            .or_else(|e| Err(AppError::from(e)))
    }

    /// Finds documents in the collection and paginates the results.
    ///
    /// # Parameters
    /// - `filter`: A MongoDB document specifying the query criteria.
    /// - `page`: The page number to read.
    /// - `limit`: The number of documents to read per page.
    ///
    /// # Returns
    /// A `Result` containing a `Vec<T>` of documents if successful, or an `AppError` if the operation fails.
    pub async fn find_paginated(
        &self,
        filter: Option<Document>,
        page: u64,
        limit: u64,
    ) -> Result<Vec<T>, AppError> {
        let options = FindOptions::builder()
            .skip((page - 1) * limit)
            .limit(limit as i64)
            .build();
        self.find(filter, Some(options)).await
    }

    /// Finds a single document in the collection that matches the provided filter.
    ///
    /// # Parameters
    /// - `filter`: A MongoDB document specifying the query criteria.
    ///
    /// # Returns
    /// A `Result` containing an `Option<T>` if successful, or an `AppError` if the operation fails.
    /// The `Option<T>` will be `Some(T)` if a document is found, otherwise `None`.
    pub async fn find_one(&self, filter: Document) -> Result<Option<T>, AppError> {
        match self.collection.find_one(filter).await? {
            Some(doc) => Ok(Some(doc)),
            None => Ok(None),
        }
        .or_else(|e: Error| Err(AppError::from(e)))
    }

    /// Finds a single document in the collection by its `_id` field.
    ///
    /// # Parameters
    /// - `id`: The `_id` of the document to find.
    ///
    /// # Returns
    /// A `Result` containing an `Option<T>` if successful, or an `AppError` if the operation fails.
    /// The `Option<T>` will be `Some(T)` if a document is found, otherwise `None`.
    pub async fn find_by_id(&self, id: &str) -> Result<Option<T>, AppError> {
        let oid = get_object_id(id)
            .or_else(|_| Err(AppError::from(("Ill-formed MongoId", 400))))?;
        self.find_one(doc! { "_id": oid })
            .await
            .or_else(|e| {
                if e.to_string().contains("not found") {
                    Err(AppError::NotFound(format!(
                        "Document with ID {} not found",
                        id
                    )))
                } else {
                    Err(e)
                }
            })
    }

    /// Returns all documents in the collection.
    ///
    /// # Returns
    /// A `Result` containing a `Vec<T>` of all documents if successful, or an `AppError` if the operation fails.
    pub async fn find_all(&self) -> Result<Vec<T>, AppError> {
        self.find(None, None).await
    }

    /// Inserts a single document into the collection.
    ///
    /// # Parameters
    /// - `doc`: The document to insert.
    ///
    /// # Returns
    /// A document if successful, or an `AppError` if the operation fails.
    pub async fn insert_one(&self, doc: T) -> Result<T, AppError> {
        match self.collection.insert_one(doc).await?.inserted_id.as_str() {
            Some(id) => self.find_by_id(id).await?.ok_or_else(|| {
                AppError::from(format!("Document not found after insert: {:?}", id))
            }),
            None => Err(AppError::from("No _id returned after insert")),
        }
    }

    /// Updates a single document in the collection that matches the provided filter.
    ///
    /// # Parameters
    /// - `filter`: A MongoDB document specifying the query criteria.
    /// - `update`: The update operations to apply to the matching document.
    ///
    /// # Returns
    /// A `Result` containing the updated document if successful, or an `AppError` if the operation fails.
    pub async fn update_one(
        &self,
        filter: Document,
        update: impl Into<UpdateModifications>,
    ) -> Result<T, AppError> {
        match self
            .collection
            .update_one(filter.clone(), update)
            .await?
            .upserted_id
        {
            Some(id) => self.find_by_id(id.as_str().unwrap()).await?.ok_or_else(|| {
                AppError::from(format!(
                    "Document not found after update: {:?}",
                    filter.clone()
                ))
            }),
            None => Err(AppError::from(format!(
                "Document not found: {:?}",
                filter.clone()
            ))),
        }
    }

    /// Updates a single document in the collection by its `_id` field.
    ///
    /// # Parameters
    /// - `id`: The `_id` of the document to update.
    /// - `update`: The update operations to apply to the matching document.
    ///
    /// # Returns
    /// A `Result` containing an updated document if successful, or an `AppError` if the operation fails.
    pub async fn update_by_id(
        &self,
        id: &str,
        update: impl Into<UpdateModifications>,
    ) -> Result<T, AppError> {
        self.update_one(doc! { "_id": get_object_id(id)? }, update)
            .await
    }

    /// Deletes a single document from the collection that matches the provided filter.
    ///
    /// # Parameters
    /// - `filter`: A MongoDB document specifying the query criteria.
    ///
    /// # Returns
    /// A `Result` containing a `DeleteResult` if successful, or an `AppError` if the operation fails.
    pub async fn delete_one(&self, filter: Document) -> Result<DeleteResult, AppError> {
        self.collection
            .delete_one(filter)
            .await
            .or_else(|e| Err(AppError::from(e)))
    }

    /// Deletes a single document from the collection by its `_id` field.
    ///
    /// # Parameters
    /// - `id`: The `_id` of the document to delete.
    ///
    /// # Returns
    /// A `Result` containing a `DeleteResult` if successful, or an `AppError` if the operation fails.
    pub async fn delete_by_id(&self, id: &str) -> Result<DeleteResult, AppError> {
        self.delete_one(doc! { "_id": get_object_id(id)? }).await
    }

    /// Deletes multiple documents from the collection that match the provided filter.
    ///
    /// # Parameters
    /// - `filter`: A MongoDB document specifying the query criteria.
    ///
    /// # Returns
    /// A `Result` containing a `DeleteResult` if successful, or an `AppError` if the operation fails.
    pub async fn delete_many(&self, filter: Document) -> Result<DeleteResult, AppError> {
        self.collection
            .delete_many(filter)
            .await
            .or_else(|e| Err(AppError::from(e)))
    }

    /// Counts the number of documents in the collection that match the provided filter.
    ///
    /// This method is useful for determining the size of a query result or checking
    /// the existence of documents that match specific criteria.
    ///
    /// # Parameters
    /// - `filter`: A MongoDB document specifying the query criteria. If `None`, counts all documents in the collection.
    ///
    /// # Returns
    /// A `Result` containing the count of matching documents as a `u64` if successful,
    /// or an `AppError` if the operation fails.
    ///
    /// # Example
    /// ```rust
    /// use mongodb::bson::doc;
    /// use mongodb::Collection;
    ///
    /// async fn example(repo: DatabaseRepository<Document>) {
    ///     let filter = doc! { "status": "active" };
    ///     let result = repo.count_documents(Some(filter)).await;
    ///     match result {
    ///         Ok(count) => println!("Number of active documents: {}", count),
    ///         Err(e) => eprintln!("Counting failed: {}", e),
    ///     }
    /// }
    /// ```
    pub async fn count_documents(&self, filter: Option<Document>) -> Result<u64, AppError> {
        match filter {
            Some(filter) => self.collection.count_documents(filter).await,
            None => self.collection.estimated_document_count().await,
        }
        .or_else(|e| Err(AppError::from(e)))
    }

    /// Executes an aggregation pipeline on the collection.
    ///
    /// Aggregation pipelines allow for complex data processing and transformations
    /// directly within the database. This method executes the provided pipeline
    /// and returns the results as a vector of documents.
    ///
    /// # Parameters
    /// - `pipeline`: A vector of MongoDB documents representing the aggregation pipeline stages.
    /// - `options`: Optional `AggregateOptions` to configure the aggregation operation.
    ///
    /// # Returns
    /// A `Result` containing a `Vec<Document>` with the results of the aggregation if successful,
    /// or an `AppError` if the operation fails.
    ///
    /// # Example
    /// ```rust
    /// use mongodb::bson::{doc, Document};
    /// use mongodb::Collection;
    /// use futures::stream::StreamExt;
    ///
    /// async fn example(repo: DatabaseRepository<Document>) {
    ///     let pipeline = vec![
    ///         doc! { "$match": { "status": "active" } },
    ///         doc! { "$group": { "_id": "$category", "total": { "$sum": 1 } } },
    ///     ];
    ///     let result = repo.aggregate(pipeline, None).await;
    ///     match result {
    ///         Ok(docs) => println!("Aggregation result: {:?}", docs),
    ///         Err(e) => eprintln!("Aggregation failed: {}", e),
    ///     }
    /// }
    /// ```
    pub async fn aggregate(
        &self,
        pipeline: Vec<Document>,
        options: Option<AggregateOptions>,
    ) -> Result<Vec<Document>, AppError> {
        self.collection
            .aggregate(pipeline)
            .with_options(options)
            .await?
            .try_collect()
            .await
            .or_else(|e| Err(AppError::from(e)))
    }
}
