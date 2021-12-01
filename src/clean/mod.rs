/// Clean data
pub trait Clean {
    type Data;
    fn clean(&self, data: &mut Self::Data);
}
