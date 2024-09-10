#[derive(Debug)]
pub struct Produto {
    nome: String,
    preco:f64,
    quantitdade: i32
}

impl Produto {

    pub fn new(n: String, p: f64, q: i32) -> Produto {

        Produto {nome: n, preco: p, quantitdade: q}
    }

    pub fn new_name(&mut self, n: String) {
        
        self.nome = n
    }

    pub fn new_price(&mut self, p: f64) {
        self.preco = p
    }

    pub fn new_quantity(&mut self, q: i32) {
        self.quantitdade = q
    }

    pub fn get_product(&self) -> Produto {
        
        let product = Produto{nome: self.nome.clone(), preco: self.preco, quantitdade: self.quantitdade};

        product
    }
    
}