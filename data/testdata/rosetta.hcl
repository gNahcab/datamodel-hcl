ontology "rosetta" {
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
    }
property "hasTextMedium" {
      ontology = "rosetta"
      object = "StillImageRepresentation"
      labels {
        en = ""
        de = ""
        fr = ""
      }
}
property "hasImage" {
  ontology = "rosetta"
  object = ":Image2D"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}

property "hasOriginalText" {
  ontology = "rosetta"
  object = "TextValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}

property "hasTranscription" {
  ontology = "rosetta"
  object = "TextValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}

property "hasTransliteration" {
  ontology = "rosetta"
  object = "TextValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}

property "hasTranslation" {
  ontology = "rosetta"
  object = "TextValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}

property "hasDescription" {
  ontology = "rosetta"
  object = "TextValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}

property "hasAuthor" {
  ontology = "rosetta"
  object = ":Person"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}

property "hasName" {
  ontology = "rosetta"
  object = "TextValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}

property "hasTitle" {
  ontology = "rosetta"
  object = "TextValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}

property "hasDate" {
  ontology = "rosetta"
  object = "DateValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}

property "hasFindspot" {
  ontology = "rosetta"
  object = "GeonameValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}

property "hasLocation" {
  ontology = "rosetta"
  object = "GeonameValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}

property "hasBibliographicReference" {
  ontology = "rosetta"
  object = "TextValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
  gui_element = "Simpletext"
}

property "hasExternalLink" {
  ontology = "rosetta"
  object = "UriValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}

property "hasIdentifier" {
  ontology = "rosetta"
  object = "UriValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}

property "hasRelatedArtwork" {
  ontology = "rosetta"
  object = "Resource"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}


property "hasCreator" {
  ontology = "rosetta"
  object = "UriValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}


property "inInstitution" {
  ontology = "rosetta"
  object = ":Institution"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}

property "hasInventoryNumber" {
  ontology = "rosetta"
  object = "TextValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
  gui_element = "Simpletext"
}


property "hasPagenum" {
  ontology = "rosetta"
  object = "IntValue"
  labels {
    en = "Page number"
    de = "Seitenzahl"
    fr = "Numéro de page"
  }
}

property "partOf" {
  ontology = "rosetta"
  object = "IntValue"
  labels {
    en = "is part of"
    de = "ist Teil von"
    fr = "fait partie de"
  }
}

property "hasChildren" {
  ontology = "rosetta"
  object = "IntValue"
  labels {
    en = "Number of children"
    de = "Anzahl Kinder"
    fr = "Nombre d'enfants"
  }
}

  Resource "Text"{
    ontology = "rosetta"
    labels {
      en = ""
      de = ""
      fr = ""
      it = ""
    }
      hasTextMedium{
        cardinality = "0-1"
        gui_order = "1"
        ontology = "rosetta"
      }
  }

  Resource "Person"{
    ontology = "rosetta"
    labels {
      en = "Person"
      de = "Person"
      fr = "Personne"
      it = "Persona"
    }
    hasName {
      cardinality = "1"
      gui_order = "1"
      ontology = "rosetta"
    }
    hasIdentifier {
      cardinality = "0-1"
      gui_order = "2"
      ontology = "rosetta"
    }
    hasChildren {
      cardinality = "0-1"
      gui_order = "3"
      ontology = "rosetta"
    }
    hasExternalLink {
      cardinality = "0-n"
      gui_order = "4"
      ontology = "rosetta"
    }
  }

Resource "Institution"{
  ontology = "rosetta"
  labels {
    en = "Institution"
    de = "Institution"
    fr = "Institution"
    it = "Istituzione"
  }
  hasName {
    cardinality = "1"
    gui_order = "1"
    ontology = "rosetta"
  }
  hasIdentifier {
    cardinality = "0-1"
    gui_order = "2"
    ontology = "rosetta"
  }
  hasChildren {
    cardinality = "0-1"
    gui_order = "3"
    ontology = "rosetta"
  }
  hasExternalLink {
    cardinality = "0-n"
    gui_order = "4"
    ontology = "rosetta"
  }
}
  StillImageRepresentation "Image2D" {
    ontology = "rosetta"
    labels {
      en = ""
      de = ""
      fr = ""
      it = ""
    }
      hasTitle {
        cardinality = "1"
        gui_order = "1"
        ontology = "rosetta"
      }
  }




