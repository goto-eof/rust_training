use fruit_basket::FruitsContainerTrait;
use fruit_factory::{FruitFactoryStruct, FruitFactoryTrait};

mod fruit;
mod fruit_basket;
mod fruit_factory;

fn main() {
    let orange =
        FruitFactoryStruct::generate_fruit("orange".to_string(), 3.2, "orange".to_string());
    let apple = FruitFactoryStruct::generate_fruit("apple".to_string(), 2.2, "green".to_string());
    let lemon = FruitFactoryStruct::generate_fruit("lemon".to_string(), 3.4, "yellow".to_string());

    let mut fruits_basket = fruit_basket::FruitsContainerStruct::new();
    fruits_basket.add(orange);
    fruits_basket.add(apple);
    fruits_basket.add(lemon);

    fruits_basket.print_content();
}
