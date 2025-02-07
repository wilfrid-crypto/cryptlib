pub trait Hash {
  pub fn new( data: &[u8] ) -> Self;
  pub fn digest(&self) -> Vec<u8>;
  pub fn hexdigest(&self) -> String;
}
