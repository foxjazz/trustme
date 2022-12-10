
use crypto;
use crypto::digest::Digest;
pub struct Fazer {
    // ...
    // Path: src\monitor\monitor.rs
    iv: Vec<u8>,
    key: Vec<u8>,
    // ...
}
impl Fazer{
    pub fn new (&mut self, password: &str) -> &mut Fazer {
        let mut hasher = crypto::Digest::new(crypto::digest::SHA256);
        //let mut hasher = Sha256::digest::new();
        hasher.input_str(password);
        self.key = hasher.result_str().as_bytes().to_vec();
        self.iv = hasher.result_str().as_bytes().to_vec();
        self
    }

    pub fn enc(&self, plain_text : &str) -> String {
        let aes = AesSafe256Encryptor::new(&self.key);
        aes.encrypt_cbc(&self.iv, plain_text.as_bytes())
    }
    pub fn dec(&self, cypher_text : &str) -> String {

        let aes = AesSafe256Encryptor::new(&self.key);
        aes.decrypt_cbc(&self.iv, cypher_text.as_bytes())
    }
}
