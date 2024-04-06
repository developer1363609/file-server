use rustls::Connection;

pub(crate) const DB_CREATE_TABLE:&str = "CREATE TABLE IF NOT EXISTS metadata (
    id                INTEGER PRIMARY KEY,
    filename          TEXT,
    owner             TEXT,
    link              TEXT,
    size              INTEGER,
    sha256            TEXT,
    filepath          TEXT,
    encrypt_key       TEXT,
    permissions       TEXT,
    type              TEXT,
    classification    TEXT,
    create_time       INTEGER,
    update_time       INTEGER,
    delete_time       INTEGER
)";

pub(crate) struct Metadata{
    #[allow(unused)]
    pub(crate) id:i64,
    pub(crate) filename:String,
    pub(crate) owner:String,
    pub(crate) link:String,
    pub(crate) size:i64,
    pub(crate) sha256:String,
    pub(crate) filepath:String,
    pub(crate) encrypt_key:String,
    pub(crate) permissions:String,
    pub(crate) r#type:String,
    pub(crate) classification:String,
    pub(crate) create_time:i64,
    pub(crate) update_time:i64,
    pub(crate) delete_time:i64
}

pub(crate) struct MetadataDB{
    conn:Connection
}

impl MetadataDB{

}