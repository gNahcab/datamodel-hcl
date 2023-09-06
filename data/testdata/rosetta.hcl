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
}
list "faculties-of-the-university-of-basel" {
  labels {
    de = ""
    en = ""
    fr = ""
    it = ""
  }
  comments {
    de = ""
    en = ""
  }
  nodes {
    faculty-of-science {
      labels {
        en = ""
        de = ""
      }
      nodes {
        physics-department {
          labels {
            en = ""
            de = ""
          }
          nodes {
            astrophysics {
              labels {
                en = ""
                de = ""
              }
            }
          }
        }
      }
    }
    faculty-of-business-and-economics {
      labels {
        en = ""
        de = ""
      }
    }
  }

}
ontology "rosetta" {
  label = "rosetta"
  properties {
    hasTime {
       object = "TimeValue"
       labels {
         en = ""
         de = ""
         fr = ""
       }
      gui_element = "facultativ?"
    }
    hasTextMedium {
      object = "StillImageRepresentation"
      labels {
        en = ""
        de = ""
        fr = ""
      }
    }
  }
}

resources {
  Text{
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
}




