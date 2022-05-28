mod link;
use link::*;


fn main() {
    let mut var = Var::new(1.0);

    let func = Func::new(
        Some(Box::new(|| {
            var.value()
        })),
        None
    );
}