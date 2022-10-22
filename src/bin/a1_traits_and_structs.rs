use a1c::fruit_basket::{FruitsContainerStruct, FruitsContainerTrait};
use a1c::fruit_factory::{FruitFactoryStruct, FruitFactoryTrait};

mod a1c;

fn main() {
    let orange =
        FruitFactoryStruct::generate_fruit("orange".to_string(), 3.2, "orange".to_string());
    let apple = FruitFactoryStruct::generate_fruit("apple".to_string(), 2.2, "green".to_string());
    let lemon = FruitFactoryStruct::generate_fruit("lemon".to_string(), 3.4, "yellow".to_string());

    let mut fruits_basket = FruitsContainerStruct::new();
    fruits_basket.add(orange);
    fruits_basket.add(apple);
    fruits_basket.add(lemon);

    fruits_basket.print_content();
}
