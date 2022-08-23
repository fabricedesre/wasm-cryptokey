use wasm_bindgen::prelude::*;
use web_sys::CryptoKey;
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum Error {
    NoPrivateKey,
    SomethingWentWrong,
}

impl Into<JsValue> for Error {
    fn into(self) -> JsValue {
        JsValue::from_str(&format!("{:?}", self))
    }
}

type Result<T> = std::result::Result<T, Error>;

#[wasm_bindgen]

struct Did {
    public_key: CryptoKey,
    private_key: Option<CryptoKey>,
}

#[wasm_bindgen]
impl Did {
    #[wasm_bindgen]
    pub async fn create() -> Result<Did> {
        Err(Error::SomethingWentWrong)
    }

    #[wasm_bindgen]
    pub fn get_private_key(&self) -> Result<CryptoKey> {
        if let Some(key) = &self.private_key {
            Ok(key.clone())
        } else {
            Err(Error::NoPrivateKey)
        }
    }

    #[wasm_bindgen]
    pub fn get_public_key(&self) -> CryptoKey {
        self.public_key.clone()
    }

    #[wasm_bindgen]
    pub async fn some_async_fn(&self) -> Result<String> {
        Ok("hello".into())
    }
}
