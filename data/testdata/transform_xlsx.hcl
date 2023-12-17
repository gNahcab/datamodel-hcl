transform = "xlsx"
sheets = [1]
sheet "1" {
  structured_by = "row"
  resource = "Person"
  assignments  {
    id = "ID"
    not_lowered = 1
    hasName = 2
    hasIdentifier = 3
    hasChildren = 4
    hasExternalLink = 5
  }

  transformations {
    lower "label" {
      input = "not_lowered"
    }
    combine "label"{
      input = [0, "lower"]
      separator = "_"
      prefix = "BIZ_"
      suffix = "_ZIP"
    }
    replace "hasIdentifier" {
      input = 1
      replace = ["DICT", "DICTIONARY"]
      condition {
        behavior = "lazy"
        target = "part"
      }
    }

  }
}
