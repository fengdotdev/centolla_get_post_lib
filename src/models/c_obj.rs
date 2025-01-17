
use super::r_obj::R_ObjInterface;


pub trait C_ObjInterface {
    fn get_c_obj(&self) -> C_Obj;

    fn get_res(&self, call:String) -> String;
    fn post_res(&self, call:String) -> String;
}

pub struct C_Obj {
    r_obj:Box<dyn C_ObjInterface>,
}







impl C_Obj {
    pub fn new() -> C_Obj {
        C_Obj {
        }
    }
}


impl C_ObjInterface for C_Obj {
    fn get_c_obj(&self) -> C_Obj {
        C_Obj::new()
    }

    fn get_res(&self, call:String) -> String {
       "get_res".to_string()
    }

    fn post_res(&self, call:String) -> String {
         "post_res".to_string()
    }
}


