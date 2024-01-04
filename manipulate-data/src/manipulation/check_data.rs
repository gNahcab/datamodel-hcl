use std::collections::HashMap;
use parse_data::datamodel_parse::domain::property::Property;
use parse_data::errors::ParsingError;

pub(crate) fn check_data(property_to_data: &HashMap<String, Vec<String>>, properties: &Vec<Property>) -> Result<(), ParsingError> {
    //todo later
    return Ok(());
    /*
    for (prop_name, data) in property_to_data.iter() {
        let property = properties.iter().find(prop_name).unwrap();
        //todo check lists here
        //todo check if dates are really dates
        // do not check relations between resources
        //
        match property.object.as_str() {
            "TimeValue" => {
                check_time_values(data)?;
            },
            "DateValue" => {
                check_date_values(data)?;
            },
            "UriValue" => {
                check_uri_values(data)?;
            },
            "IntValue" =>{
                check_int_values(data)?;
            },
            "BooleanValue"=> {
                check_boolean_values(data)?;
            },
            "GeonameValue" => {
                check_geoname_values(data)?;
            },
            "ListValue" => {
                check_list_values(data)?;
            },
            "ColorValue" => {
                check_color_values(data)?;
            }
            ,
            _ =>  {}//do nothing,
        }
    }
    Ok(())

     */
}

fn check_color_values(p0: &Vec<String>) -> Result<(), ParsingError> {
    todo!()
}

fn check_list_values(p0: &Vec<String>) -> Result<(), ParsingError> {
    todo!()
}

fn check_geoname_values(p0: &Vec<String>) -> Result<(), ParsingError>{
    todo!()
}

fn check_boolean_values(p0: &Vec<String>) -> Result<(), ParsingError>{
    todo!()
}

fn check_int_values(p0: &Vec<String>) -> Result<(), ParsingError>{
    todo!()
}

fn check_uri_values(p0: &Vec<String>) -> Result<(), ParsingError>{
    todo!()
}

fn check_date_values(p0: &Vec<String>) -> Result<(), ParsingError>{
    todo!()
}

fn check_time_values(p0: &Vec<String>) -> Result<(), ParsingError>{
    todo!()
}
