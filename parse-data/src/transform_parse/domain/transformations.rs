use std::collections::HashSet;
use hcl::{Attribute, Block};
use crate::errors::ParsingError;
use crate::transform_parse::domain::header_value::HeaderValue;
use crate::transform_parse::domain::methods_domain::combine_method::{CombineMethod, WrapperCombineMethod};
use crate::transform_parse::domain::methods_domain::lower_upper_method::{LowerMethod, UpperMethod, WrapperLowerUpperMethod};
use crate::transform_parse::domain::methods_domain::method::Method;
use crate::transform_parse::domain::methods_domain::replace_method::{ReplaceMethod, WrapperReplaceMethod};
use crate::transform_parse::domain::methods_domain::to_date_method::{ToDateMethod, WrapperToDateMethod};

#[derive(Debug)]
pub struct TransformationsWrapper (pub(crate) Block);
#[derive(Debug, Clone)]
pub struct Transformations{
    pub lower_methods:Vec<LowerMethod>,
    pub upper_methods:Vec<UpperMethod>,
    pub combine_methods:Vec<CombineMethod>,
    pub replace_methods:Vec<ReplaceMethod>,
    pub to_date_methods:Vec<ToDateMethod>,
}

impl Transformations {
}

impl Transformations {
}

impl Transformations {
    fn new() -> Transformations {
        return Transformations {
            lower_methods: vec![],
            upper_methods: vec![],
            combine_methods: vec![],
            replace_methods: vec![],
            to_date_methods: vec![],
        }
    }
    pub(crate) fn add_lower_method(&mut self, lower_method: LowerMethod) -> Result<(), ParsingError> {
        lower_method.is_correct()?;
        self.lower_methods.push(lower_method);
        Ok(())
    }
    pub(crate) fn add_upper_method(&mut self, upper_method: UpperMethod) -> Result<(), ParsingError> {
        upper_method.is_correct()?;
        self.upper_methods.push(upper_method);
        Ok(())
    }
    pub(crate) fn add_combine_method(&mut self, combine_method: CombineMethod) -> Result<(), ParsingError> {
        combine_method.is_correct()?;
        self.combine_methods.push(combine_method);
        Ok(())
    }
    pub(crate) fn add_replace_method(&mut self, replace_method: ReplaceMethod) -> Result<(), ParsingError> {
        replace_method.is_correct()?;
        self.replace_methods.push(replace_method);
        Ok(())
    }
    pub(crate) fn add_to_date_method(&mut self, to_date_method: ToDateMethod) -> Result<(), ParsingError> {
        to_date_method.is_correct()?;
        self.to_date_methods.push(to_date_method);
        Ok(())
    }
    pub(crate) fn output_values(&self) -> Vec<&String> {
        let mut vec:Vec<&String> = vec![];
        vec.extend(
            self.lower_methods.iter().map(|lower|&lower.output).collect::<Vec<&String>>(),
        );
        vec.extend(
            self.upper_methods.iter().map(|upper|&upper.output).collect::<Vec<&String>>(),
        );
        vec.extend(
            self.combine_methods.iter().map(|combine|&combine.output).collect::<Vec<&String>>(),
        );
        vec.extend(
            self.replace_methods.iter().map(|replace|&replace.output).collect::<Vec<&String>>(),
        );
        vec.extend(
            self.to_date_methods.iter().map(|to_date|&to_date.output).collect::<Vec<&String>>(),
        );
        vec

    }

    pub(crate) fn input_values(&self) -> Vec<&HeaderValue> {
        let mut vec:Vec<&HeaderValue> = vec![];
        vec.extend(
            self.lower_methods.iter().map(|lower|&lower.input).collect::<Vec<&HeaderValue>>(),
        );
        vec.extend(
            self.upper_methods.iter().map(|upper|&upper.input).collect::<Vec<&HeaderValue>>(),
        );
        vec.extend(
         self.combine_methods.iter().map(|combine|&combine.input).into_iter().flatten().collect::<Vec<&HeaderValue>>()
        );
        vec.extend(
            self.replace_methods.iter().map(|replace|&replace.input).collect::<Vec<&HeaderValue>>(),
        );
        vec.extend(
            self.to_date_methods.iter().map(|to_date|&to_date.input).collect::<Vec<&HeaderValue>>(),
        );
        vec

    }
    pub fn is_consistent(&self, sheet_nr: usize) -> Result<(), ParsingError> {
        // check that output-values are unique
        self.output_values().sort();
        let mut uniq = HashSet::new();
        for output_value in self.output_values() {
            if uniq.insert(output_value) == false {
                return Err(ParsingError::ValidationError(format!("found a duplicated value '{:?}' in output of 'transform' in sheet-nr '{:?}'", output_value, sheet_nr)));
            }
        }
        // check that output/input-values don't form a closed cycle

        // 1. find all input-values that are not output-values (i.e. all those that connect to assignments or spreadsheet)
        let input_values_no_numbers: Vec<&String> = self.input_values().iter().flat_map(|value|match value {
            HeaderValue::Name(name) => {Some(name)}
            HeaderValue::Number(_) => {None}
        }).collect();
        let input_equals_output: Vec<&&String> = input_values_no_numbers.iter().filter(|input|self.output_values().contains(input)).collect();
        if self.input_values().len() == input_equals_output.len() {
            return Err(ParsingError::ValidationError(format!("transform seems to build a perfect closed cycle, such that every input is depending on an output and vice versa.")))
        }
        // 2.
        // there could still be small cycles: check that every input-value that is an output-value as well, can be traced back to an input-value that is not an output-value
        // todo: check this by going through (input, output)-pairs of every method,
        // filter them first if they are part of set 'input_equals_output'
        // then hop from output-value to equal input-value
        // follow every method till I find a input that is not part of set 'input_equals_output'(no cycle) or I find a input I already visited( = cycle)
        // repeat for every method I didn't visit

        Ok(())
    }
    pub fn methods(&self) -> Vec<Method> {
        let mut methods: Vec<Method> = vec![];
        methods.extend(self.lower_methods.iter().map(|method|Method::LowerMethod(method.to_owned())).collect::<Vec<Method>>());
        methods.extend(self.upper_methods.iter().map(|method|Method::UpperMethod(method.to_owned())).collect::<Vec<Method>>());
        methods.extend(self.combine_methods.iter().map(|method|Method::CombineMethod(method.to_owned())).collect::<Vec<Method>>());
        methods.extend(self.replace_methods.iter().map(|method|Method::ReplaceMethod(method.to_owned())).collect::<Vec<Method>>());
        methods.extend(self.to_date_methods.iter().map(|method|Method::ToDateMethod(method.to_owned())).collect::<Vec<Method>>());
        return methods;
    }
}
impl TransformationsWrapper {
    pub fn to_transformations(&self) -> Result<Transformations, ParsingError> {
        let mut transformations: Transformations = Transformations::new();
        let attributes: Vec<&Attribute> = self.0.body.attributes().collect();
        if attributes.len() !=0 {
            return Err(ParsingError::ValidationError(format!("found attributes in transformations, but only blocks allowed. Found attributes are: '{:?}'", attributes)));
        }
        let blocks: Vec<&Block> = self.0.body.blocks().collect();
        if blocks.len() == 0 {
            return Err(ParsingError::ValidationError(format!("found zero blocks in transformations, but blocks should exist in: '{:?}'", self.0)));
        }
        for block in blocks {
             match block.identifier.as_str() {
                "lower" => {
                   let lower_method = WrapperLowerUpperMethod(block.to_owned()).to_lower_method()?;
                   transformations.add_lower_method(lower_method)?;
                }
                "upper" => {
                    let upper_method = WrapperLowerUpperMethod(block.to_owned()).to_upper_method()?;
                    transformations.add_upper_method(upper_method)?;
                }
                "combine"=> {
                    let combine_method = WrapperCombineMethod(block.to_owned()).to_combine_method()?;
                    transformations.add_combine_method(combine_method)?;
                }
                "replace"=> {
                    let replace_method = WrapperReplaceMethod(block.to_owned()).to_replace_method()?;
                    transformations.add_replace_method(replace_method)?;
                }
                "to_date"=> {
                    let to_date_method = WrapperToDateMethod(block.to_owned()).to_date_method()?;
                    transformations.add_to_date_method(to_date_method)?;
                }
                _ => {
                    return Err(ParsingError::ValidationError(format!("unknown method found in transformations: can't find '{:?}'", block.identifier)));
                }
            }
        }
        Ok(transformations)
    }
}
#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use crate::transform_parse::domain::header_value::HeaderValue;
    use crate::transform_parse::domain::methods_domain::behavior_type::BehaviorType;
    use crate::transform_parse::domain::methods_domain::combine_method::CombineMethod;
    use crate::transform_parse::domain::methods_domain::date_type::DateType;
    use crate::transform_parse::domain::methods_domain::lower_upper_method::{LowerMethod, UpperMethod};
    use crate::transform_parse::domain::methods_domain::replace_method::ReplaceMethod;
    use crate::transform_parse::domain::methods_domain::target_type::TargetType;
    use crate::transform_parse::domain::methods_domain::to_date_method::ToDateMethod;
    use crate::transform_parse::domain::transformations::Transformations;

    #[test]
    fn test_check_full_cycle() {
        let replace_method1 = ReplaceMethod {
            output: "hasExternalLink2".to_string(),
            input: HeaderValue::Name("hasExternalLink".to_string()),
            old: "http".to_string(),
            new: "https".to_string(),
            behavior: BehaviorType::Lazy,
            target: TargetType::Part,
        };
        let replace_method2 = ReplaceMethod {
            output: "hasValue".to_string(),
            input: HeaderValue::Name("hasExternalLink".to_string()),
            old: "http".to_string(),
            new: "https".to_string(),
            behavior: BehaviorType::Lazy,
            target: TargetType::Part,
        };
        let combine_method = CombineMethod {
            input: vec![HeaderValue::Name("hasExternalLink2".to_string()), HeaderValue::Name("hasValue".to_string())],
            output: "hasExternalLink".to_string(),
            separator: Option::from("_".to_string()),
            prefix: Option::from("my_project".to_string()),
            suffix: None,
        };
        let transformations = Transformations {
            lower_methods: vec![],
            upper_methods: vec![],
            combine_methods: vec![combine_method],
            replace_methods: vec![replace_method1, replace_method2],
            to_date_methods: vec![],
        };
        let result = transformations.is_consistent(100);
        println!("result {:?}", result);
        assert!(result.is_err());
    }
    #[test]
    fn test_check_is_consistent() {
        let lower_method = LowerMethod { output: "hasLowerValue".to_string(), input: HeaderValue::Number(0) };
        let upper_method = UpperMethod { output: "hasUpperValue".to_string(), input: HeaderValue::Name("hasValue".to_string()) };
        let replace_method = ReplaceMethod {
            output: "hasExternalLink2".to_string(),
            input: HeaderValue::Name("hasExternalLink".to_string()),
            old: "http".to_string(),
            new: "https".to_string(),
            behavior: BehaviorType::Lazy,
            target: TargetType::Part,
        };
        let combine_method = CombineMethod {
            input: vec![HeaderValue::Name("hasExternalLink2".to_string()), HeaderValue::Number(4)],
            output: "hasExternalLink".to_string(),
            separator: Option::from("_".to_string()),
            prefix: Option::from("my_project".to_string()),
            suffix: None,
        };
        let to_date_method = ToDateMethod {
            output: "hasDate".to_string(),
            input: HeaderValue::Name("hasValue".to_string()),
            date_type: DateType::Gregorian,
            date_patterns: vec![],
        };
        let transformations = Transformations {
            lower_methods: vec![lower_method],
            upper_methods: vec![upper_method],
            combine_methods: vec![combine_method],
            replace_methods: vec![replace_method],
            to_date_methods: vec![to_date_method],
        };
        let result = transformations.is_consistent(100);
        //todo: hasExternalLink and hasExternalLink2 build a small cycle, should return an error!
        println!("result {:?}", result);
        assert!(result.is_err());
    }
}
