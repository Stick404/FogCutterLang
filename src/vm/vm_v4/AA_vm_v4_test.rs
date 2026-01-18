#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::vm::vm_v4::{core::VMState, object::{Object, ObjectType}};

    #[test]
    pub fn object_init_test() {
        let mut vm= VMState::new();
        let typ: &ObjectType = ObjectType::new_primitive(1, &mut vm);
        println!("{typ:?}");
        let mut object = Object::new_object(&typ);

        object.set_data(vec![0xFF]);

        assert_eq!(object.get_data()[0], 0xFF);
        
        let z = vm.get_type(0).unwrap();
        println!("{z:?}");
        assert_eq!(z, typ)
    }

    #[test]
    pub fn rust_why_do_you_do_this_i_am_in_so_much_pain_right_now() {

    }
}