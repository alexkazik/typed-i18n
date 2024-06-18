use crate::messages::param_type::ParamType;

#[derive(Debug)]
pub(crate) enum Piece<'a> {
    Text(&'a str),
    Param(&'a str, ParamType),
}
