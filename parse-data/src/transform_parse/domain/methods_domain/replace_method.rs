use std::collections::HashMap;
use hcl::{Attribute, Expression};
use hcl::ser::Block;
use crate::errors::ParsingError;
use crate::expression_trait::ExpressionTransform;
use crate::transform_parse::domain::header_value::{HeaderMethods, HeaderValue};
use crate::transform_parse::domain::methods_domain::behavior_type::BehaviorType;
use crate::transform_parse::domain::methods_domain::target_type::TargetType;
use crate::transform_parse::domain::methods_domain::to_date_method::ToDateMethod;
use crate::transform_parse::domain::methods_domain::wrapper_trait::Wrapper;

pub struct WrapperReplaceMethod(pub(crate) hcl::Block);

#[derive(Debug)]
struct TransientStructureReplaceMethod {
    output: String,
   input: Option<HeaderValue>,
    old: Option<String>,
    new: Option<String>,
    behavior: Option<String>,
    target: Option<String>,
}

impl TransientStructureReplaceMethod {
    fn new(output: String) -> TransientStructureReplaceMethod {
        TransientStructureReplaceMethod{
            output,
            input: None,
            old: None,
            behavior: None,
            target: None,
            new: None,
        }
    }
    fn add_input(&mut self, expression: Expression) -> Result<(), ParsingError> {
        if self.input.is_some() {
            return Err(ParsingError::ValidationError(format!("found multiple input-attributes  in method '{:?}'.", self.output)));
        }
        let header_value = expression.to_header_value()?;
        self.input = Option::from(header_value);
        Ok(())
    }
    fn add_new(&mut self, expression: Expression) -> Result<(), ParsingError> {
        if self.new.is_some() {
            return Err(ParsingError::ValidationError(format!("found multiple new-attributes  in method '{:?}'.", self.output)));
        }
        self.new = Option::from(expression.to_string_2()?);
        Ok(())
    }
    fn add_old(&mut self, expression: Expression) -> Result<(), ParsingError> {
        if self.old.is_some() {
            return Err(ParsingError::ValidationError(format!("found multiple old-attributes  in method '{:?}'.", self.output)));
        }
        self.old = Option::from(expression.to_string_2()?);
        Ok(())
    }
    fn add_condition(&mut self, block: hcl::Block) -> Result<(), ParsingError> {
        block.no_blocks()?;
        for attribute in block.attributes() {
            match attribute.key.as_str() {
                "behavior" => {
                    if self.behavior.is_some() {
                        return Err(ParsingError::ValidationError(format!("found multiple behavior-attributes  in method '{:?}'.", self.output)));
                    }
                    self.behavior = Option::from(attribute.expr.to_string_2()?);
                }
                "target" => {
                    if self.target.is_some() {
                        return Err(ParsingError::ValidationError(format!("found multiple target-attributes  in method '{:?}'.", self.output)));
                    }
                    self.target = Option::from(attribute.expr.to_string_2()?);
                }
                _ => {
                    return Err(ParsingError::ValidationError(format!("found 'condition'-attribute that is unknown in method '{:?}', found: {:?}.", self.output, attribute)));
                }
            }
        }
        Ok(())
    }
    fn is_consistent(&self) -> Result<(), ParsingError> {
        if self.input.is_none() {
            return Err(ParsingError::ValidationError(format!("replace-method '{:?}' doesn't have an input-attribute provided", self)));
        }
        if self.old.is_none() {
            return Err(ParsingError::ValidationError(format!("replace-method '{:?}' doesn't have a old-attribute provided", self)));
        }
        if self.new.is_none() {
            return Err(ParsingError::ValidationError(format!("replace-method '{:?}' doesn't have a new-attribute provided", self)));
        }
        if self.behavior.is_none() {
            return Err(ParsingError::ValidationError(format!("replace-method '{:?}' doesn't have a behavior-attribute in 'condition'-block provided", self)));
        }
        if self.target.is_none() {
            return Err(ParsingError::ValidationError(format!("replace-method '{:?}' doesn't have a target-attribute in 'condition'-block provided", self)));
        }

        Ok(())
    }
}
impl WrapperReplaceMethod{
    pub(crate) fn to_replace_method(&self) -> Result<ReplaceMethod, ParsingError> {
        let mut transient_structure = TransientStructureReplaceMethod::new(self.0.get_output()?);
        for attribute in self.0.attributes() {
            match attribute.key.as_str() {
                "input" => {
                    transient_structure.add_input(attribute.expr.to_owned())?;
                }
                "old" => {
                    transient_structure.add_old(attribute.expr.to_owned())?;
                }
                "new" => {
                    transient_structure.add_new(attribute.expr.to_owned())?;
                }
                _ => {
                    return Err(ParsingError::ValidationError(format!("found this unknown attribute '{:?}' in method '{:?}'.", attribute, transient_structure.output)));
                }
            }
        }
        for block in self.0.blocks() {
            match block.identifier.as_str() {
                "condition" => {
                    transient_structure.add_condition(block.to_owned())?;
                }
                _ => {
                    return Err(ParsingError::ValidationError(format!("found this unknown block '{:?}' in method '{:?}'.", block, transient_structure.output)));
                }
            }
        }
        transient_structure.is_consistent()?;

        let replace_method = ReplaceMethod::new(transient_structure)?;
        Ok(replace_method)
    }
}
#[derive(Debug, Clone)]
pub struct ReplaceMethod {
    pub output: String,
    pub input: HeaderValue,
    pub old: String,
    pub new: String,
    pub behavior: BehaviorType,
    pub target: TargetType,
}

impl ReplaceMethod {
}

impl ReplaceMethod {
    fn new(transient_structure: TransientStructureReplaceMethod) -> Result<ReplaceMethod, ParsingError> {

       let behavior_type: BehaviorType = BehaviorType::behavior_type(transient_structure.behavior.unwrap())?;
        let target_type: TargetType = TargetType::target_type(transient_structure.target.unwrap())?;

        Ok(ReplaceMethod{
            output: transient_structure.output,
            input: transient_structure.input.unwrap(),
            old: transient_structure.old.unwrap(),
            new: transient_structure.new.unwrap(),
            behavior: behavior_type,
            target: target_type,
        })
    }
    pub(crate) fn is_correct(&self) -> Result<(), ParsingError> {
        if self.input.is_equal(&self.output) {
            return Err(ParsingError::ValidationError(format!("method has the same in- and output-String, which is forbidden: '{:?}'", self.input)));
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use hcl::block;
    use crate::transform_parse::domain::methods_domain::replace_method::WrapperReplaceMethod;

    #[test]
    fn test_replace_method() {
        let block = block!(replace "replacement"{
            input = 3
            old = "Dict"
            new = "Dictionary"
            condition {
                behavior = "lazy"
                target = "part" // target = "word"
            }
        });

        let result = WrapperReplaceMethod(block.to_owned()).to_replace_method();
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}
