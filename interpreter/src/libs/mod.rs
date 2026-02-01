pub mod math;

use crate::evaluator::env::Environment;
use std::collections::HashMap;

pub type LibRegisterFn = fn(&mut Environment);

pub fn get_library_registry() -> HashMap<String, LibRegisterFn> {
    let mut registry: HashMap<String, LibRegisterFn> = HashMap::new();
    
    // Standard libraries: 
    registry.insert("math".to_string(), crate::libs::math::register);
    
    registry
}