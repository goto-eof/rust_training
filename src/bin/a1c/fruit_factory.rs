use super::fruit::FruitStruct;

pub trait FruitFactoryTrait {
    fn generate_fruit(name: String, weight: f32, color: String) -> FruitStruct;
}

pub struct FruitFactoryStruct();

impl FruitFactoryTrait for FruitFactoryStruct {
    fn generate_fruit(name: String, weight: f32, color: String) -> FruitStruct {
        FruitStruct {
            name,
            weight,
            color,
        }
    }
}
