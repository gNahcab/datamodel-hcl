use crate::domain::password;
use crate::DatamodelHCLError;

// The ProjectInfo struct represents the general information about a project ( shortcode, shortname, longname, descriptions, keywords,
// password(or no password here?))  of a Datamodel

pub struct ProjectInfo {
   // pub names: ProjectNames,
   // pub descriptions: Descriptions,
   // pub keywords: Vec<Keyword>,
    //pub password: Password,

}


impl TryFrom<&hcl::Block> for ProjectInfo {

  type Error = DatamodelHCLError;

  fn try_from(project_block: &hcl::Block) -> Result<Self, Self::Error> {
    if project_block.identifier.as_str() != "project_info" {
      return Err(DatamodelHCLError::ParseProjectInfo(
        String::from("I couldn't find the block 'project_info', write block 'project_info'."),
      ));
    }
    Ok(ProjectInfo {})
  }
}
#[cfg(test)]
mod test{
use hcl::block;
  use crate::domain::project_info::ProjectInfo;

  #[test]
    fn test_read_project_info() {
        let input_datamodel_block = block!(
                project_info {
                  shortcode = "082E"
                  shortname = "rosetta"
                  longname = "Rosetta:DSP example project"
                  descriptions {
                    de = "Rosetta ist das Beispielprojekt für die <em>DaSCH Service Platform</em>. Es soll einerseits die Möglichkeiten illustrieren, die die Plattform aktuell bietet, andererseits aber auch intern aufzeigen, wo noch Verbesserungsbedarf besteht."
                    fr = "Rosetta est le projet exemplaire de la <em>DaSCH Service Platform</em>. D'une part, il vise à illustrer les possibilités actuellement offertes par la plate-forme, mais d'autre part, il montre également en interne les domaines où il est encore possible d'apporter des améliorations."
                    en = "Rosetta is the sample project for the <em>DaSCH Service Platform</em>. On one hand, it is intended to illustrate the possibilities currently offered by the platform, but on the other hand, it also shows internally where there is still room for improvement."
                  }
                  keywords = [
                    "Textquellen",
                    "Bilder",
                    "Audio",
                    "Sonderzeichen",
                    "XML",
                    "Markup",
                    "Annotation",
                    "textual sources",
                    "images",
                    "audio",
                    "special characters",
                    "sources",
                    "caractères spéciaux",
                    "Data and Service Center for the Humanities (DaSCH)"
                  ]
                  password = "rosetta1234"
                });

      let project =  ProjectInfo::try_from(&input_datamodel_block).unwrap();
    }
}
