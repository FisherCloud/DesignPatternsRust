mod system;

use system::{SystemA, SystemB, SystemC};

#[derive(Debug)]
pub struct Facade {
    m_system_a: SystemA,
    m_system_b: SystemB,
    m_system_c: SystemC,
}

impl Facade {
    pub fn new() -> Facade {
        Facade {
            m_system_a: SystemA::new(),
            m_system_b: SystemB::new(),
            m_system_c: SystemC::new(),
        }
    }

    pub fn wrap_operation(self: &Self) {
        self.m_system_a.operation_a();
        self.m_system_b.operation_b();
        self.m_system_c.operation_c();
    }
}
