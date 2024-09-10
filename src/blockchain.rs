struct Block {
    index: u64,
    timestamp: u64,
    data: String,
    hash: String,
    prev_hash: String
}

impl Block {

    fn new(data: Block) -> Block{
        Block {
            index: data.index, 
            timestamp: data.timestamp, 
            data: data.data, 
            hash: data.hash, 
            prev_hash: data.prev_hash,
        }
    }

    fn date_size(&self) -> usize{
        self.data.len()
    }

    fn creation_time(&self) -> u64 {
        self.timestamp / 1000
    }

}


pub fn create_block(){
    let data = Block{index: 0, timestamp:  1605991464000, data: "dados do bloco".to_string(), hash: "hash".to_string(), prev_hash: "hash anterior".to_string()};
    let block = Block::new(data);
    let size = block.date_size();
    let time = block.creation_time();
    println!("O tamanho do dado do bloco é {} e foi criado em: {}" , size, time);
}