use crate::json::crawler::reading_objects::readable_type::ReadableType;


pub trait IReadingObjectBase {

    fn is_finalized(&self) -> bool;

    fn get_previous_crawler_context(&self) -> ReadableType;

    fn set_previous_crawler_context(&mut self, new_type:ReadableType);
}

///
/// Should not be accessed outside of 'extension' methods
pub trait IReadingObject<T> : IReadingObjectBase {
    fn built_value(&mut self) -> T;
}

pub fn initialize_interface(this:&mut dyn IReadingObjectBase, new_previous_crawler_context:ReadableType) {
    if this.get_previous_crawler_context() != ReadableType::None { panic!("JSON crawler: Incorrect previous context!!!"); }


    this.set_previous_crawler_context(new_previous_crawler_context);
}

impl<T> dyn IReadingObject<T> // I don't think I understand how this works..... Doesnt seem to work like extension methods
    where T: Default {
    fn try_get_built_value(&mut self, out_value: &mut T) -> bool {
        if self.is_finalized() {
            *out_value = self.built_value();
            true
        } else {
            *out_value = Default::default();
            false
        }
    }

    pub fn get_built_value(&mut self) -> T {
        let mut built_object:T = Default::default();
        if !self.try_get_built_value(&mut built_object) { panic!("JSON Crawler: Could not build value."); }


        built_object
    }
}