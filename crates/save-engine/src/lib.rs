pub mod binary;
pub mod botw;
pub mod error;
pub mod hashtable;
pub mod totk;

pub use error::SaveError;

pub enum Save {
    Botw(botw::BotwSave),
}

impl Save {
    pub fn detect(bytes: Vec<u8>) -> Result<Save, SaveError> {
        botw::BotwSave::load(bytes).map(Save::Botw)
    }

    pub fn to_bytes(self) -> Vec<u8> {
        match self {
            Save::Botw(save) => save.to_bytes(),
        }
    }
}
