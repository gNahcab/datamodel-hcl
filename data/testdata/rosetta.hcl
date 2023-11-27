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
      gui_element = "facultativ?"
    }
property "hasTextMedium" {
      ontology = "rosetta"
      object = "StillImageRepresentation"
      labels {
        en = ""
        de = ""
        fr = ""
      }
      gui_element = "facultativ?"
}
property "hasImage" {
  ontology = "rosetta"
  object = ":Image2D"
  labels {
    en = ""
    de = ""
    fr = ""
  }
  gui_element = "facultativ?"
}

property "hasOriginalText" {
  ontology = "rosetta"
  object = "TextValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
  gui_element = "facultativ?"
}

property "hasTranscription" {
  ontology = "rosetta"
  object = "TextValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
  gui_element = "facultativ?"
}

property "hasTransliteration" {
  ontology = "rosetta"
  object = "TextValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
  gui_element = "facultativ?"
}

property "hasTranslation" {
  ontology = "rosetta"
  object = "TextValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
  gui_element = "facultativ?"
}

property "hasDescription" {
  ontology = "rosetta"
  object = "TextValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
  gui_element = "facultativ?"
}

property "hasAuthor" {
  ontology = "rosetta"
  object = ":Person"
  labels {
    en = ""
    de = ""
    fr = ""
  }
  gui_element = "facultativ?"
}

property "hasName" {
  ontology = "rosetta"
  object = "TextValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
  gui_element = "facultativ?"
}

property "hasTitle" {
  ontology = "rosetta"
  object = "TextValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
  gui_element = "facultativ?"
}

property "hasDate" {
  ontology = "rosetta"
  object = "DateValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
  gui_element = "facultativ?"
}

property "hasFindspot" {
  ontology = "rosetta"
  object = "GeonameValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
  gui_element = "facultativ?"
}

property "hasLocation" {
  ontology = "rosetta"
  object = "GeonameValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
  gui_element = "facultativ?"
}

property "hasBibliographicReference" {
  ontology = "rosetta"
  object = "TextValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
  gui_element = "facultativ?"
}

property "hasExternalLink" {
  ontology = "rosetta"
  object = "UriValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
  gui_element = "facultativ?"
}

property "hasIdentifier" {
  ontology = "rosetta"
  object = "UriValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
  gui_element = "facultativ?"
}

property "hasRelatedArtwork" {
  ontology = "rosetta"
  object = "Resource"
  labels {
    en = ""
    de = ""
    fr = ""
  }
  gui_element = "facultativ?"
}


property "hasCreator" {
  ontology = "rosetta"
  object = "UriValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
  gui_element = "facultativ?"
}


property "inInstitution" {
  ontology = "rosetta"
  object = "inInstitution"
  labels {
    en = ""
    de = ""
    fr = ""
  }
  gui_element = "facultativ?"
}

property "hasInventoryNumber" {
  ontology = "rosetta"
  object = "TextValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
  gui_element = "facultativ?"
}


property "hasPagenum" {
  ontology = "rosetta"
  object = "IntValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
  gui_element = "facultativ?"
}

property "partOf" {
  ontology = "rosetta"
  object = "IntValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
  gui_element = "facultativ?"
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




