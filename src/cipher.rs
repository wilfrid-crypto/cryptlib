pub trait Cipher {
  pub fn new(key: &[u8]) -> Self;
  pub fn encrypt(data: &[u8]) -> Self;
  pub fn decrypt(data: &[u8]) -> Self;
}
