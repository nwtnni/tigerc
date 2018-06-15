mod frame;

use uuid::Uuid;

#[derive(Clone)]
pub struct Temp {
    id: Uuid,    
    name: String,
}

#[derive(Clone)]
pub struct Label {
    id: Uuid,    
    name: String,
}
