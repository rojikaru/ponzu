use crate::dto::pagination::Pagination;
use crate::services::db_repo::DatabaseRepository;
use crate::types::app_error::AppError;
use crate::utils::bson::get_object_id;
use mongodb::bson::{doc, Document};
use mongodb::options::{AggregateOptions, FindOptions, UpdateModifications};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::Arc;

/// A service that provides CRUD operations for a given entity.
///
/// # Parameters
/// - `E`: The entity type.
/// - `R`: The type of the entity's read operation (Read DTO).
/// - `C`: The type of the entity's create operation (Create DTO).
/// - `U`: The type of the entity's update operation (Update DTO).
pub trait CrudService<E, R, C, U>
where
    E: Clone + Send + Sync + DeserializeOwned + Serialize,
    R: Clone + From<E>,
    C: Clone + Into<E>,
    U: Clone + Into<UpdateModifications>,
{
    /// Creates a new instance of the `CrudService`.
    ///
    /// # Parameters
    /// - `repository`: The repository that provides the CRUD operations.
    fn new(repository: Arc<DatabaseRepository<E>>) -> Self
    where
        Self: Sized;

    /// Retrieves all entities.
    ///
    /// # Returns
    /// A `Vec` of entities.
    async fn get_all(&self) -> Result<Vec<R>, AppError>;

    /// Finds documents in the collection and paginates the results.
    ///
    /// # Parameters
    /// - `filter`: A MongoDB document specifying the query criteria.
    /// - `page`: The page number to read.
    /// - `limit`: The number of documents to read per page.
    ///
    /// # Returns
    /// A `Pagination` of read DTOs.
    async fn get_paginated(
        &self,
        filter: Option<Document>,
        page: u64,
        limit: u64,
    ) -> Result<Pagination<R>, AppError>;

    /// Filters entities in the collection.
    ///
    /// # Parameters
    /// - `filter`: A MongoDB document specifying the query criteria.
    /// - `options`: The options to apply to the query.
    ///
    /// # Returns
    /// A `Vec` of entities.
    async fn find(
        &self,
        filter: Option<Document>,
        options: Option<FindOptions>,
    ) -> Result<Vec<R>, AppError>;

    /// Retrieves an entity by its ID.
    ///
    /// # Parameters
    /// - `id`: The ID of the entity.
    ///
    /// # Returns
    /// The entity.
    async fn get_by_id(&self, id: &str) -> Result<Option<R>, AppError>;

    /// Aggregates entities by a filter.
    ///
    /// # Parameters
    /// - `filter`: The filter to apply.
    /// - `options`: The aggregation options.
    ///
    /// # Returns
    /// A `Vec` of entities.
    async fn aggregate(
        &self,
        filter: Vec<Document>,
        options: Option<AggregateOptions>,
    ) -> Result<Vec<Document>, AppError>;

    /// Counts entities by a filter.
    ///
    /// # Parameters
    /// - `filter`: The filter to apply.
    ///
    /// # Returns
    /// The count of entities.
    async fn count(&self, filter: Option<Document>) -> u64;

    /// Creates a new entity.
    ///
    /// # Parameters
    /// - `entity`: The entity to create.
    ///
    /// # Returns
    /// The created entity.
    async fn create(&self, entity: C) -> Result<R, AppError>;

    /// Updates an entity by its ID.
    ///
    /// # Parameters
    /// - `id`: The ID of the entity.
    /// - `update`: The update to apply.
    ///
    /// # Returns
    /// The updated entity.
    async fn update(&self, id: &str, update: U) -> Result<R, AppError>;

    /// Deletes an entity by its ID.
    ///
    /// # Parameters
    /// - `id`: The ID of the entity.
    ///
    /// # Returns
    /// The deleted entity.
    async fn delete(&self, id: &str) -> Result<bool, AppError>;

    /// Deletes entities by a criteria.
    ///
    /// # Parameters
    /// - `criteria`: The criteria to apply.
    ///
    /// # Returns
    /// The number of deleted entities.
    async fn delete_by_criteria(&self, criteria: Document) -> Result<u64, AppError>;
}

pub struct CrudServiceImpl<E, R, C, U>
where
    E: Clone + Send + Sync + DeserializeOwned + Serialize + 'static,
    R: Clone + From<E>,
    C: Clone + Into<E>,
    U: Clone + Into<UpdateModifications>,
{
    repository: Arc<DatabaseRepository<E>>,
    _phantom: std::marker::PhantomData<(E, R, C, U)>,
}

impl<E, R, C, U> CrudService<E, R, C, U> for CrudServiceImpl<E, R, C, U>
where
    E: Clone + Send + Sync + DeserializeOwned + Serialize + 'static,
    R: Clone + From<E>,
    C: Clone + Into<E>,
    U: Clone + Into<UpdateModifications>,
{
    fn new(repository: Arc<DatabaseRepository<E>>) -> Self {
        Self {
            repository,
            _phantom: Default::default(),
        }
    }

    async fn get_all(&self) -> Result<Vec<R>, AppError> {
        self.find(None, None).await
    }

    async fn get_paginated(
        &self,
        filter: Option<Document>,
        page: u64,
        limit: u64,
    ) -> Result<Pagination<R>, AppError> {
        let options = FindOptions::builder()
            .skip((page - 1) * limit)
            .limit(limit as i64)
            .build();
        let vec = self.find(filter.clone(), Some(options)).await?;
        let total = self.count(filter.clone()).await;
        Ok(Pagination {
            current_page: page,
            last_page: (total as f64 / limit as f64).ceil() as u64,
            per_page: limit,
            total,
            payload: vec,
        })
    }

    async fn find(
        &self,
        filter: Option<Document>,
        options: Option<FindOptions>,
    ) -> Result<Vec<R>, AppError> {
        Ok(self
            .repository
            .find(filter, options)
            .await?
            .iter()
            .map(|e| R::from(e.clone()))
            .collect::<Vec<_>>())
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<R>, AppError> {
        let oid =
            get_object_id(id).or_else(|_| Err(AppError::from(("Ill-formed MongoId", 400))))?;
        Ok(self
            .repository
            .find_one(doc! { "_id": oid })
            .await?
            .map(|e| R::from(e)))
    }

    async fn aggregate(
        &self,
        filter: Vec<Document>,
        options: Option<AggregateOptions>,
    ) -> Result<Vec<Document>, AppError> {
        self.repository.aggregate(filter, options).await
    }

    async fn count(&self, filter: Option<Document>) -> u64 {
        self.repository.count_documents(filter).await.unwrap_or(0)
    }

    async fn create(&self, entity: C) -> Result<R, AppError> {
        Ok(R::from(self.repository.insert_one(entity.into()).await?))
    }

    async fn update(&self, id: &str, update: U) -> Result<R, AppError> {
        let oid =
            get_object_id(id).or_else(|_| Err(AppError::from(("Ill-formed MongoId", 400))))?;
        self.repository
            .update_one(doc! { "_id": oid }, update.into())
            .await
            .map(|e| R::from(e))
    }

    async fn delete(&self, id: &str) -> Result<bool, AppError> {
        Ok(self
            .repository
            .delete_one(doc! { "_id": get_object_id(id)? })
            .await?
            .deleted_count
            > 0)
    }

    async fn delete_by_criteria(&self, criteria: Document) -> Result<u64, AppError> {
        Ok(self.repository.delete_many(criteria).await?.deleted_count)
    }
}
