pub use surrealize_derive::Surrealize;

extern crate surrealize_derive;

#[cfg(test)]
mod test {
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    use crate::Surrealize;

    #[derive(Serialize, Deserialize, Surrealize)]
    struct TestStruct {
        id: Uuid,
        a: i32,
        b: i32,
    }
}
