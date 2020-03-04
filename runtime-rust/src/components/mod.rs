use yarrow_validator::errors::*;
use crate::base::NodeArguments;
use yarrow_validator::base::Value;
use yarrow_validator::utilities::serial::parse_value;

//mod bin;
mod cast;
mod clamp;
//mod count;
mod impute;
mod index;
//mod kth_raw_sample_moment;
mod materialize;
mod mean;
//mod median;
mod mechanisms;
mod resize;
//mod row_max;
//mod row_min;
//mod sum;
//mod transform;

use yarrow_validator::proto;


pub trait Evaluable {
    fn evaluate(&self, arguments: &NodeArguments) -> Result<Value>;
}

impl Evaluable for proto::component::Value {
    fn evaluate(&self, arguments: &NodeArguments) -> Result<Value> {
        macro_rules! evaluate {
            ($self:ident, $arguments:ident, $( $variant:ident ),*) => {
                {
                    $(
                       if let proto::component::Value::$variant(x) = $self {
                            return x.evaluate($arguments)
                       }
                    )*
                }
            }
        }

        evaluate!(self, arguments,
            // INSERT COMPONENT LIST
            Cast, Clamp, Impute, Index, Materialize, Mean, LaplaceMechanism, Resize
        );

        Err(format!("Component type not implemented: {:?}", self).into())

    }
}


impl Evaluable for proto::Constant {
    fn evaluate(&self, _arguments: &NodeArguments) -> Result<Value> {
        parse_value(&self.to_owned().value.unwrap())
    }
}