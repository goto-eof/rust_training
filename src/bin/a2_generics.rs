use std::fmt::Debug;

fn main() {
    let mut wrapper: Wrapper<String, i32> = Wrapper {
        list_a: Vec::new(),
        list_b: Vec::new(),
    };

    wrapper.retrieve_list_a().push("Ciao A".to_string());
    wrapper.retrieve_list_b().push(13);
    wrapper.print_list_a();
    wrapper.print_list_b();
    Wrapper::<String, i32>::print_list(wrapper.retrieve_list_a());
}

struct Wrapper<AType, BType> {
    list_a: Vec<AType>,
    list_b: Vec<BType>,
}

impl<AType: Debug, BType: Debug> Wrapper<AType, BType> {
    fn retrieve_list_a(&mut self) -> &mut Vec<AType> {
        return &mut self.list_a;
    }

    fn retrieve_list_b(&mut self) -> &mut Vec<BType> {
        return &mut self.list_b;
    }

    fn print_list_a(&mut self) -> () {
        for ele in self.retrieve_list_a() {
            println!("OK: {:?}", ele);
        }
    }

    fn print_list_b(&mut self) -> () {
        for ele in self.retrieve_list_b() {
            println!("OK: {:?}", ele);
        }
    }

    fn print_list<T: Debug>(list: &mut Vec<T>) -> () {
        for ele in list {
            println!("OK: {:?}", ele);
        }
    }
}
