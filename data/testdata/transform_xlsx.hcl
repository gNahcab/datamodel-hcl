transform = "xlsx"
sheets = "all"
sheet "1" {
  structured_by = "column"
  headers = true
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
    to_date "hasDate" {
      input = 6
      calendar_type= "Gregorian"
      pattern "1" {
        // e.g. 1.12 - 23.12.1991
        // e.g. 1 Dez - 23 Dezember 1991
        first {
          month = 1
          year = 2
        }
        date {
          day = 1
          month = 2
          month_word = true
          year = 3
        }
      }
    }

  }
}
