use syn::ItemFn;

#[derive(Clone)]
pub struct Function {
    pub name: String,
    pub item: ItemFn,
}