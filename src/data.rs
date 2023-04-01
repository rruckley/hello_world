// Some data

#[derive(Debug)]
pub struct Data {
    qty : i32,
    terms : Vec<String>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            qty : 0,
            terms: vec![]
        }
    }

    pub fn add_term(&mut self, term : String) -> Result<(),String> {
        self.terms.push(term);
        Ok(())
    }
}