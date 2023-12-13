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
      gui_element = "TimeStamp"
    }
property "hasTextMedium" {
      ontology = "rosetta"
      object = "StillImageRepresentation"
      labels {
        en = ""
        de = ""
        fr = ""
      }
      gui_element = "Searchbox"
}
property "hasImage" {
  ontology = "rosetta"
  object = ":Image2D"
  labels {
    en = ""
    de = ""
    fr = ""
  }
  gui_element = "Searchbox"
}

property "hasOriginalText" {
  ontology = "rosetta"
  object = "TextValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
  gui_element = "Textarea"
}

property "hasTranscription" {
  ontology = "rosetta"
  object = "TextValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
  gui_element = "Richtext"
}

property "hasTransliteration" {
  ontology = "rosetta"
  object = "TextValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
  gui_element = "Textarea"
}

property "hasTranslation" {
  ontology = "rosetta"
  object = "TextValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
  gui_element = "Textarea"
}

property "hasDescription" {
  ontology = "rosetta"
  object = "TextValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
  gui_element = "Richtext"
}

property "hasAuthor" {
  ontology = "rosetta"
  object = ":Person"
  labels {
    en = ""
    de = ""
    fr = ""
  }
  gui_element = "Searchbox"
}

property "hasName" {
  ontology = "rosetta"
  object = "TextValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
  gui_element = "Simpletext"
}

property "hasTitle" {
  ontology = "rosetta"
  object = "TextValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
  gui_element = "Simpletext"
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
        gui_order = "0"
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
        gui_order = "0"
        ontology = "rosetta"
      }
  }




