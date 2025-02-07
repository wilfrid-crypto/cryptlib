pub trait Hash {
  fn new( data: &[u8] ) -> Self;
  fn digest(&self) -> Vec<u8>;
  fn hexdigest(&self) -> String;
}
