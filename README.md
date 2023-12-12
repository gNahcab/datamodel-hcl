# Parse and validate DaSCH-Datamodels from HCL using Rust

 - Information about the project (Shortname, Longname, Shortcode, Keywords, Descriptions, Users etc.) and Lists is not part of HCL-Datamodel.
 - In the HCL-Datamodel we care only about ontologies, properties and resources. 

## Structure of the file
- Ontologies, properties and resources don't have to be in a fixed order.
 
##### ontologies
- the datamodel must contain at least one ontology with the name and the label of the ontology.
- a complete ontology looks like this:
 
```hcl
ontology "rosetta" {
  label = "rosetta"
}
```

##### properties
- properties are used as properties of a resource, since they can be used in different resources, they are described separately from the resources.
- a complete property looks like this:
```hcl
property "hasPagenum" {
  ontology = "rosetta"
  object = "IntValue"
  labels {
    en = "pagination"
    de = "pagination"
    fr = "pagination"
  }
  gui_element = "0-1"
}
```

##### resources
- a resource is first a container for a bunch of properties and second it can have additional features that define what it represents
- a resource that doesn't have any additional features:
```hcl 
Resource "Text"{
    ontology = "rosetta"
    labels {
      en = ""
      de = ""
      fr = ""
      it = ""
    }
  }
```
- additional features: only StillImageRepresentation supported at the moment

##### res-props
- a res-prop is a representation of a property within the resource


# Transform HCL to modify import data


### methods:
- lower
- upper
- add
- replace
- to_date

#### lower
 string to lowercase

#### upper
 string to uppercase

#### new
- multiple variables and fixed string elements possible
 -> e.g. new($a_$b) where $a, $b are variables and "_" is a fixed string element

#### replace
- replace elements of a string by another string
- only fixed strings can be replaced
- words get replaced, not parts of words
- all occurrences get replaced
- multiple replace need to have a different name (e.g. replace "a" and replace "b")

#### to_date
- tries to return a DSP-Date according to data provided