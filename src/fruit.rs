pub struct FruitStruct {
    pub name: String,
    pub weight: f32,
    pub color: String,
}

trait ColorRetrieverTrait {
    fn get_color(&self) -> &str;
}

trait WeightRetrieverTrait {
    fn get_weight(&self) -> f32;
}
trait NameRetrieverTrait {
    fn get_name(&self) -> &str;
}

impl ColorRetrieverTrait for FruitStruct {
    fn get_color(&self) -> &str {
        self.color.as_str()
    }
}

impl WeightRetrieverTrait for FruitStruct {
    fn get_weight(&self) -> f32 {
        return self.weight;
    }
}

impl NameRetrieverTrait for FruitStruct {
    fn get_name(&self) -> &str {
        return self.name.as_str();
    }
}
