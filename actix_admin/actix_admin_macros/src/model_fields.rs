use syn::{
    Visibility, Type
};

pub struct ModelField {
    pub visibility: Visibility,
    pub ident: proc_macro2::Ident,
    pub ty: Type,
    //  struct field is option<>
    pub inner_type: Option<Type>,
    pub primary_key: bool
}

impl ModelField {
    pub fn is_option(&self) -> bool { 
        self.inner_type.is_some()
    }
}