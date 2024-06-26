pub mod task;

#[async_trait::async_trait]
pub trait Repository: Send + Sync {
    type Model: sea_orm::entity::ModelTrait;
    type Id;
    type CreateDTO;
    type UpdateDTO;

    async fn create(&self, data: Self::CreateDTO) -> Self::Model;
    async fn find_one(&self, id: &Self::Id) -> Option<Self::Model>;
    async fn find_all(&self) -> Vec<Self::Model>;
    async fn delete(&self, id: &Self::Id);
    async fn update(&self, id: &Self::Id, data: Self::UpdateDTO);
}
