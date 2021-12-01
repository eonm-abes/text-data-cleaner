pub trait Normalize {
    type Data;
    fn normalize(&self, data: &mut Self::Data);
}
