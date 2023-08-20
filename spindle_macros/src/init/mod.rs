mod parse;
#[cfg(test)]
mod test;
mod tokens;

pub struct Attrs;
pub struct InputInitFn;
pub struct OutputInitFn;

pub fn init(attrs: Attrs, init_map: InputInitFn) -> OutputInitFn {
    todo!()
}
