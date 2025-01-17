use super::r_obj::RObjInterface;



pub struct RustObject {
    content: Box<dyn RObjInterface>,
}



impl RustObject {
    pub fn new(something: dyn RObjInterface) -> RustObject {
        RustObject {
            content: something,
        }
    }

    pub fn new_with_builder(builder_func: fn()->dyn RObjInterface) -> RustObject {
        RustObject {
            content: builder_func(),
        }
    }

   pub fn get_res(&self, call:String) -> String {
        &self.content.get_res(call)
     }
 
    pub  fn post_res(&self, call:String) -> String {
             &self.content.post_res(call)
     }
}


