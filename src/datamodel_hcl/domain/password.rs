use crate::DatamodelHCLError;
use hcl::{attribute, block, Error};
use hcl::Value::String;

pub struct Password {
    pub password: std::string::String
}


impl TryFrom<&hcl::Attribute> for Password {
    type Error = crate::errors::DatamodelHCLError;
    fn try_from(attribute: &hcl::Attribute) -> Result<Self, Self::Error> {
        type Error = crate::errors::DatamodelHCLError;

        return if attribute.key() != "password" {
            Err(Error::ParsePassword(std::string::String::from("password attribute is not provided.")))
        } else {
            let result = match attribute.expr() {
                hcl::Expression::String(value) => Ok(Password { password: value.to_string() }),
                _ => Err(Error::ParsePassword(std::string::String::from("password needs to be a string"))),
            };
            result
        }
    }
}
#[cfg(test)]
mod test{
    use hcl::{attribute, block};
    use crate::domain::password::Password;

    #[test]
    fn test_read_password() {
        let password_attribute = attribute!(password = "rosetta1234");
        let password =  Password::try_from(&password_attribute).unwrap();


    }
}
