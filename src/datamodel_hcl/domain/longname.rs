pub struct Longname {
    pub longname: std::string::String
}


impl TryFrom<&hcl::Attribute> for Longname {
    type Error = crate::errors::DatamodelHCLError;
    fn try_from(attribute: &hcl::Attribute) -> Result<Self, Self::Error> {
        type Error = crate::errors::DatamodelHCLError;

        return if attribute.key() != "longname" {
            Err(Error::ParseLongname(std::string::String::from("longname attribute is not provided.")))
        } else {
            let result = match attribute.expr() {
                hcl::Expression::String(value) => Ok(Longname { longname: value.to_string() }),
                _ => Err(Error::ParseLongname(std::string::String::from("longname needs to be a string"))),
            };
            result
        }
    }
}

#[cfg(test)]
mod test{
    use hcl::{attribute, block};
    use crate::domain::longname::Longname;

    #[test]
    fn test_read_longname() {
        let longname_attribute = attribute!(longname = "rosetta");
        let longname =  Longname::try_from(&longname_attribute).unwrap();


    }
}
