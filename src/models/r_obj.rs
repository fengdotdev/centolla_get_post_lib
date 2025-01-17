pub trait RObjInterface {
    fn get_r_obj(&self) -> dyn RObjInterface;
    fn get_res(&self, call:String) -> String;
    fn post_res(&self, call:String) -> String;
}






