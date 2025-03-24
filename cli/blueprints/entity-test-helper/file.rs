use crate::{DBPool, entities::{{entity_plural_name}}::{{entity_struct_name}}};
use fake::{faker::name::en::*, Dummy};
use validator::Validate;

#[derive(Debug, Clone, Dummy, Validate)]
pub struct {{entity_struct_name}}Changeset {
    // these are examples only
    #[dummy(faker = "Name()")]
    #[validate(length(min = 1))]
    pub name: String,
}

pub async fn create({{entity_singular_name}}: {{entity_struct_name}}Changeset, db: &DBPool) -> Result<{{entity_struct_name}}, anyhow::Error> {
    todo!("Adapt the SQL query as necessary!");
    let record = sqlx::query!(
        "INSERT INTO {{entity_plural_name}} (name) VALUES (?) RETURNING id",
        {{entity_singular_name}}.name,
    )
    .fetch_one(db)
    .await?;

    Ok({{entity_struct_name}} { name: {{entity_singular_name}}.name })
}
