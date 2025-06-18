pub struct Echo{
    pub message: String
}
impl Echo{
    pub fn echo(&self){
        println!("{}", self.message);
    }
}