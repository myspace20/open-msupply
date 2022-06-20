use async_graphql::*;
use repository::FormSchema;

pub struct JSONSchemaNode {
    pub schema: FormSchema,
}

#[Object]
impl JSONSchemaNode {
    pub async fn id(&self) -> &str {
        &self.schema.id
    }

    pub async fn json_schema(&self) -> &serde_json::Value {
        &self.schema.json_schema
    }
}

pub struct FormSchemaNode {
    pub schema: FormSchema,
}

#[Object]
impl FormSchemaNode {
    pub async fn id(&self) -> &str {
        &self.schema.id
    }

    pub async fn r#type(&self) -> &str {
        &self.schema.r#type
    }

    pub async fn json_schema(&self) -> &serde_json::Value {
        &self.schema.json_schema
    }

    pub async fn ui_schema(&self) -> &serde_json::Value {
        &self.schema.ui_schema
    }
}
