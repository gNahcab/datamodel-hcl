# Parse and validate DaSCH-Datamodels from HCL using Rust

 - Information about the project (Shortname, Longname, Shortcode, Keywords, Descriptions, Users etc.) and Lists is not part of HCL-Datamodel(Properties with Listvalues will be dealt with later).
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
}
```
- gui_element: necessary for TextValue-objects (Simpletext, Textarea, Richtext)
##### resources
- every resource contains:
1. one ontology
2. one labels
3. zero (?) or more res-props

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
    hasTitle {
      cardinality = "1"
      gui_order = "0"
      ontology = "rosetta"
   }
  }
```
- additional features: only StillImageRepresentation
```hcl
 StillImageRepresentation "Image2D"{
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
```
###### labels
 should contain one or more language-tags
 currently the following languages are supported:
- English (en)
- German (de)
- French (fr)
- Italian (it)
- Romansh (ro)
###### res-props
- a res-prop is a representation of a property within the resource
- the name of the res-prop should correspond to a property [see "properties"]
- within the res-prop container: cardinality, gui_order, ontology


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