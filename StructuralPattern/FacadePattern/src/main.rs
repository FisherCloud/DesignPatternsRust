mod facade;

use facade::Facade;

fn main() {
    let fa = Facade::new();
    fa.wrap_operation();
}
