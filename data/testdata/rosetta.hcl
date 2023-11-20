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
      object = "StillImageRepresentation"
      labels {
        en = ""
        de = ""
        fr = ""
      }
}
property "hasImage" {
  object = ":Image2D"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}

property "hasOriginalText" {
  object = "TextValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}

property "hasTranscription" {
  object = "TextValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}

property "hasTransliteration" {
  object = "TextValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}

property "hasTranslation" {
  object = "TextValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}

property "hasDescription" {
  object = "TextValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}

property "hasAuthor" {
  object = ":Person"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}

property "hasName" {
  object = "TextValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}

property "hasTitle" {
  object = "TextValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}

property "hasDate" {
  object = "DateValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}

property "hasFindspot" {
  object = "GeonameValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}

property "hasLocation" {
  object = "GeonameValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}

property "hasBibliographicReference" {
  object = "TextValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}

property "hasExternalLink" {
  object = "UriValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}

property "hasIdentifier" {
  object = "UriValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}

property "hasRelatedArtwork" {
  object = "Resource"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}


property "hasCreator" {
  object = "UriValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}


property "inInstitution" {
  object = "inInstitution"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}

property "hasInventoryNumber" {
  object = "TextValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}


property "hasPagenum" {
  object = "IntValue"
  labels {
    en = ""
    de = ""
    fr = ""
  }
}

property "partOf" {
  object = "IntValue"
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
      hasTextMedium{
        cardinality = "0-1"
        ontology = "rosetta"
      }
    hasInvalidMedium{
      cardinality = "0-1"
      ontology = "rosetta"
    }
  }

  StillImageRepresentation "Image2D" {
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




