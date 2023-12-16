use hcl::{Attribute, Block, block, BlockLabel};
use crate::errors::ParseError;
use crate::datamodel_parse::domain::label::{Label, LabelBlockWrapper, LabelWrapper};
use crate::datamodel_parse::remove_useless_quotation_marks;


#[derive(Debug, PartialEq)]
pub struct Property {
    pub name: String,
    pub ontology: String,
    pub object: String,
    pub labels: Vec<Label>,
    pub gui_element: Option<String>,
}

impl Property {
    fn new(transient_structure: TransientStructureProperty) -> Property{
        Property{
            name: transient_structure.propname,
            ontology: transient_structure.ontology.unwrap(),
            object: transient_structure.object.unwrap(),
            labels: transient_structure.labels,
            gui_element: transient_structure.gui_element,
        }
    }
}

pub(crate) struct PropertyWrapper(pub(crate) hcl::Block);

struct TransientStructureProperty {
    propname: String,
    object: Option<String>,
    ontology: Option<String>,
    labels: Vec<Label>,
    gui_element: Option<String>,
}

impl TransientStructureProperty {
    fn new() -> TransientStructureProperty {
        TransientStructureProperty {
            propname: "".to_string(),
            object: None,
            ontology: None,
            labels: vec![],
            gui_element: None,
        }
    }
    pub(crate) fn add_propname(&mut self, labels: Vec<BlockLabel>) -> Result<(), ParseError> {
        if labels.len() > 1 {
           return Err(ParseError::ValidationError(String::from(format!("too many propnames in '{:?}'", labels))));
        }
        if labels.len()  == 0 {
            return Err(ParseError::ValidationError(String::from(format!("not enough propnames: '{:?}'", labels))));
        }
        let propname = labels.get(0).expect(&*format!("cannot parse propname {:?}", labels.get(0))).clone().into_inner();
        self.propname = propname;
        Ok(())
    }

    pub(crate) fn add_gui_element(&mut self, new_gui_element: String) -> Result<(), ParseError> {
        if !self.gui_element.is_none() {
            return Err(ParseError::ValidationError(String::from(format!("it was tried to add more than one gui_element: '{:?}'", new_gui_element))));
        }
        let gui_element = remove_useless_quotation_marks(new_gui_element);
        self.gui_element = Option::from(gui_element);
        Ok(())
    }
    pub(crate) fn add_ontology(&mut self, new_ontology: String) ->  Result<(), ParseError> {
        if !self.ontology.is_none() {
            return Err(ParseError::ValidationError(String::from(format!("it was tried to add more than one ontology: '{:?}'", new_ontology))));
        }
        let ontology = remove_useless_quotation_marks(new_ontology);
        self.ontology = Option::from(ontology);
        Ok(())
    }
    pub(crate) fn add_object(&mut self, new_object: String) ->  Result<(), ParseError> {
        if !self.object.is_none() {
            return Err(ParseError::ValidationError(String::from(format!("it was tried to add more than one object: '{:?}'", new_object))));
        }
        let object = remove_useless_quotation_marks(new_object);
        self.object = Option::from(object);
        Ok(())
    }
    pub(crate) fn add_labels(&mut self, blocks: Vec<&Block>) -> Result<(), ParseError> {
        if blocks.len() > 1 {
            return Err(ParseError::ParseProjectModel(String::from(
                format!("found '{}' block(s) in '{}'. One block is necessary but not more blocks are allowed",blocks.len(), self.propname))));
        }
        if blocks.len() == 0 {
            return Err(ParseError::ParseProjectModel(String::from(format!("found '{}' block(s) in '{}'. One and only one block is necessary", blocks.len(), self.propname))));
        }
        if blocks.get(0).as_ref().unwrap().identifier.as_str() != "labels" {
            return Err(ParseError::ParseProjectModel(String::from(
                format!("wrong identifier in property '{}' for labels, expected 'labels', found '{}'", self.propname, blocks.get(0).as_ref().unwrap().identifier.as_str()))));
        }
        let label_block = blocks.get(0).expect("expected to get first and only block");
        self.labels = LabelBlockWrapper(label_block.to_owned().to_owned()).to_labels()?;

        Ok(())
    }
    pub(crate) fn is_complete(&self) -> Result<(), ParseError> {
        if self.propname.is_empty() {
            return Err(ParseError::ValidationError(format!("propname doesn't exist or isn't provided correctly.")));
        }
        // one object
        if self.object.is_none() {
            return Err(ParseError::ValidationError(format!("object in '{:?}' doesn't exist or isn't provided correctly.", self.propname)));
        }
        // one ontology
        if self.ontology.is_none() {
            return Err(ParseError::ValidationError(format!("ontology in '{:?}' doesn't exist or isn't provided correctly.", self.propname)));
        }
        // gui-element only necessary for TextValue
        /*
        if self.gui_element.is_none() {
            return Err(ParseError::ValidationError(format!("gui_element in '{:?}' doesn't exist or isn't provided correctly.", self.propname)));
        }
         */

        if self.labels.len() < 1 {
            return Err(ParseError::ValidationError(format!("labels in '{:?}' seem to be empty.", self.propname)));
        }
        Ok(())
    }
    pub(crate) fn is_correct(&self) -> Result<(), ParseError> {
        // check that property is formally correct
        // object-test
        let object: String = self.object.unwrap().to_string();
        if !object.contains(":") {
            // if object starts with ':' or contains ':' (e.g. 'my_ontology:has_prop') the object is part of the data-model and it cannot be checked it exists or not, but references of dsp-base-properties can be tested
            let valid_objects = [ "BooleanValue","ColorValue", "DateValue","DecimalValue","GeonameValue","IntValue","IntervalValue","ListValue","TextValue","TimeValue","UriValue","AudioRepresentation", "MovingImageRepresentation"];
            return Err(ParseError::ValidationError(format!("property '{:?}' has 'object' 'TextValue' and gui_element is '{:?}' but it should be one of these: '{:?}'", self.propname, object, text_value_gui_elements)));
        }
        // gui-element
        let gui_element_objects = ["DecimalValue", "IntValue", "TextValue"];
        if gui_element_objects.contains(&&*object) && self.gui_element.is_none() {
            return Err(ParseError::ValidationError(format!("property '{:?}' has 'object' '{:?}' but gui_element doesn't exist.", self.propname, object)));
        }
        if !gui_element_objects.contains(&&*object) && self.gui_element.is_some() {
            return Err(ParseError::ValidationError(format!("property '{:?}' has 'object' '{:?}' but gui_element exists, gui_element is only used with these objects: '{:?}'", self.propname, object, gui_element_objects)));
        }
        let text_value_gui_elements = ["Richtext", "Textarea", "Simpletext"];
        if object == "TextValue" && !text_value_gui_elements.contains(&&*self.gui_element.unwrap()) {
            return Err(ParseError::ValidationError(format!("property '{:?}' has 'object' '{:?}' and gui_element is '{:?}' but it should be one of these: '{:?}'", self.propname, object, self.gui_element.unwrap(), text_value_gui_elements)));
        }
        let int_value_gui_elements = ["Spinbox", "Simpletext"];
        if object == "DecimalValue" || object == "IntValue" && !int_value_gui_elements.contains(&&*self.gui_element.unwrap()) {
            return Err(ParseError::ValidationError(format!("property '{:?}' has 'object' '{:?}' and gui_element is '{:?}' but it should be one of these: '{:?}'", self.propname, object, self.gui_element.unwrap(), text_value_gui_elements)));
        }
        Ok(())
    }
}


impl PropertyWrapper {
    pub fn to_property(self) -> Result<Property, ParseError> {
        // one propname
        let mut transient_structure_property = TransientStructureProperty::new();
        transient_structure_property.add_propname(self.0.labels)?;


        let attributes: Vec<&hcl::Attribute> = self.0.body.attributes().collect();

        for attribute in attributes {
            match attribute.key.as_str() {
                "object" => transient_structure_property.add_object(attribute.expr.to_string())?,
                "ontology" => transient_structure_property.add_ontology(attribute.expr.to_string())?,
                "gui_element" => transient_structure_property.add_gui_element(attribute.expr.to_string())?,
                _ => return Err(ParseError::ParseProjectModel(String::from(format!("found unknown attribute {} of property {}. Valid attributes are: object, ontology, gui_element", attribute.key.as_str(), transient_structure_property.propname))))
            }
        }
        let blocks: Vec<&hcl::Block> = self.0.body.blocks().collect();
        transient_structure_property.add_labels(blocks)?;

        transient_structure_property.is_complete()?;
        transient_structure_property.is_correct()?;

        Ok(Property::new(transient_structure_property))
    }
}

#[cfg(test)]

mod test {
    use hcl::{block};
    use crate::datamodel_parse::domain::label::Label;
    use crate::datamodel_parse::domain::property::{Property, PropertyWrapper};
    use crate::errors::ParseError;

    #[test]
    fn test_into_property() {
        let property_block = block!(
            property "hasTextMedium" {
                object = "StillImageRepresentation"
                ontology = "rosetta"
                labels {
                    en = "my text medium"
                    de = "mein Schriftmedium"
                    fr = "mon médium d'écriture"
                }
            }
        );
        let property_wrapper = PropertyWrapper{ 0: property_block };
        let property:Result<Property, ParseError> = property_wrapper.to_property();
        println!("{:?}", property);
        assert!(property.as_ref().is_ok());
        assert_eq!(property.as_ref().unwrap().name, "hasTextMedium");
        assert_eq!(property.as_ref().unwrap().object, "StillImageRepresentation");
        assert_eq!(property.as_ref().unwrap().ontology, "rosetta");
        assert_eq!(property.as_ref().unwrap().labels.get(0).unwrap().language_abbr,"en");
        assert_eq!(property.as_ref().unwrap().labels.get(0).unwrap().text,"my text medium");
        assert_eq!(property.as_ref().unwrap().labels.get(1).unwrap().language_abbr,"de");
        assert_eq!(property.as_ref().unwrap().labels.get(1).unwrap().text,"mein Schriftmedium");
        assert_eq!(property.as_ref().unwrap().labels.get(2).unwrap().language_abbr,"fr");
        assert_eq!(property.as_ref().unwrap().labels.get(2).unwrap().text,"mon médium d'écriture");

    }
}
