fn main() {
    let obj = Test {};
    func(&obj);
}

trait Test1 {
    fn print(&self, aa: i32) -> ();
}
trait Test2 {
    fn print2(&self, aa: i32, bb: i32) -> ();
    fn print2_2(aa: i32, bb: i32) -> ();
}

struct Test {}

impl Test1 for Test {
    fn print(&self, aa: i32) -> () {
        println!("1={}", aa);
    }
}

impl Test2 for Test {
    fn print2(&self, aa: i32, bb: i32) -> () {
        println!("1={}-{}", aa, bb);
    }

    fn print2_2(aa: i32, bb: i32) {
        println!("1={}-{}", aa, bb);
    }
}

fn func<T: Test1 + Test2>(obj: &T) -> () {
    obj.print(1);
    obj.print2(3, 2);
    T::print2_2(3, 3);
}
