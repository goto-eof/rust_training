use super::fruit::FruitStruct;

pub struct FruitsContainerStruct {
    list: Vec<FruitStruct>,
}

impl FruitsContainerStruct {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }
}

pub trait FruitsContainerTrait {
    fn add(&mut self, fruit: FruitStruct) -> ();
    fn remove(&mut self, idx: usize) -> ();
    fn get(&self, idx: usize) -> Option<&FruitStruct>;
    fn get_all(&self) -> &Vec<FruitStruct>;

    fn print_content(&self) -> ();
}

impl FruitsContainerTrait for FruitsContainerStruct {
    fn add(&mut self, fruit: FruitStruct) -> () {
        self.list.push(fruit);
    }

    fn get(&self, idx: usize) -> Option<&FruitStruct> {
        return self.list.get(idx);
    }

    fn get_all(&self) -> &Vec<FruitStruct> {
        return &self.list;
    }

    fn remove(&mut self, idx: usize) -> () {
        self.list.remove(idx);
    }

    fn print_content(&self) -> () {
        for fruit in &self.list {
            println!("{} | {} | {}", fruit.name, fruit.weight, fruit.color);
        }
    }
}
