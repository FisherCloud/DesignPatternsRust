#[derive(Debug)]
pub struct SystemA{
}

impl SystemA {
    pub fn new() -> SystemA {
        SystemA{}
    }

    pub fn operation_a(&self) {
        println!("operation_a");
    }
}

#[derive(Debug)]
pub struct SystemB{
}

impl SystemB {
    pub fn new() -> SystemB {
        SystemB{}
    }

    pub fn operation_b(&self) {
        println!("operation_b");
    }
}

#[derive(Debug)]
pub struct SystemC{
}

impl SystemC {
    pub fn new() -> SystemC {
        SystemC{}
    }

    pub fn operation_c(&self) {
        println!("operation_c");
    }
}