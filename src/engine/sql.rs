use std::sync::Arc;

pub type Ident = Arc<String>;

pub enum DataTypeX {
    Int,
    VarChar,
}

pub type DataType = Arc<DataTypeX>;

pub struct ColumnDefX {
    pub name: Ident,
    pub data_type: DataType,
}

pub type ColumnDef = Arc<ColumnDefX>;

pub struct CreateTableX {
    name: String,
    columns: Vec<ColumnDef>,
}

pub type CreateTable = Arc<CreateTableX>;

pub struct Insert {}

pub struct Select {}

pub struct Delete {}
