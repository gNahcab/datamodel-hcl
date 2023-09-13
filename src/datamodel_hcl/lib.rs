pub mod errors;
pub mod domain;
use crate::errors::DatamodelHCLError;
pub mod operations;
use std::path::Path;
use clap::builder::Str;
use hcl::Error;
use hcl::Value::String;
use crate::domain::project_model::ProjectModel;





pub fn load_datamodel<P: AsRef<Path>>(path: P) -> () {
    let input = std::fs::read_to_string(path);
    let inputstr = match input {
        Ok(str_) => str_,
        Err(_) => std::string::String::from("found error"),
    };
    let body:hcl::Body = hcl::from_str(&inputstr).expect("couldn't parse body");
    // call parser method
    let result: ProjectModel= body.try_into().unwrap();
}



#[cfg(test)]
mod tests {
    use std::result;
    use hcl::body;
    use crate::domain::project_model::ProjectModel;
    use crate::errors::DatamodelHCLError;

    #[test]
    fn into_datamodel_test() {
        let datamodel = body!(ontology "rosetta" {
  label = "rosetta"
}
property "hasTime" {
      ontology = "rosetta"
       object = "TimeValue"
       labels {
         en = ""
         de = ""
         fr = ""
       }
      gui_element = "facultativ?"
    }
property "hasTextMedium" {
      object = "StillImageRepresentation"
      labels {
        en = ""
        de = ""
        fr = ""
      }
}

  Resource "Text"{
    labels {
      en = ""
      de = ""
      fr = ""
      it = ""
    }
    cardinalities {
      hasTextMedium{
        cardinality = "0-1"
      }
    }
  }

  StillImageRepresentation "Image2D" {
    labels {
      en = ""
      de = ""
      fr = ""
      it = ""
    }
    cardinalities {
      hasTitle {
        cardinality = "1"
        gui_order = "0"
      }
    }
  }
);

    }

    }


