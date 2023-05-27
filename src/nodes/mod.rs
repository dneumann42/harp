use self::functions::Function;

pub mod environment;
pub mod functions;

#[derive(Clone, Debug)]
pub enum Node {
    Nothing,
    Fun(Function),
    Mod(),
}
