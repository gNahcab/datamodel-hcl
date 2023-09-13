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

  Resource "Text"{
    labels {
      en = ""
      de = ""
      fr = ""
      it = ""
    }
      hasTextMedium{
        cardinality = "0-1"
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
      }
  }




