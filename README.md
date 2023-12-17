# Parse and validate DaSCH-Datamodels from HCL using Rust

 - Information about the project (Shortname, Longname, Shortcode, Keywords, Descriptions, Users etc.) and Lists is not part of HCL-Datamodel(Properties with Listvalues will be dealt with later).
 - In the HCL-Datamodel we care only about ontologies, properties and resources. 
## Cli Commands
- evaluate data-model:
- evaluate transform-file:
- manipulate xlsx-data:
- manipulate csv-data:

## Structure of datamodel-hcl
- Ontologies, properties and resources don't have to be in a fixed order.
 
##### ontologies
- a complete ontology looks like this:
- multiple ontologies in one datamodel are possible
 
```hcl
ontology "rosetta" {
  label = "rosetta"
}
```

##### properties
- every property consists of: 
 <ol style="padding-left: 40px">
<li>the <em>name</em> which is provided outside the brackets</li>
<li>the <em>ontology</em> it is part of</li>
 <li>the <em>object</em> which describes the value the property contains</li>
 <li>the <em>labels</em> that describe the property in multiple languages</li>
</ol>

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
- gui_element: only necessary for object "TextValue" ('Simpletext', 'Textarea', 'Richtext'), "IntValue" or "DecimalValue" (both 'Simpletext' or 'Spinbox')
##### resources
- every resource consists of:
<ol style="padding-left: 40px">
<li>one <em>ontology</em></li>
<li>one<em>labels</em></li>
<li>zero (?) or more <em>res-props</em></li>
</ol>

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
- possible additional features: other type of Resource, instead of normal Resource: StillImageRepresentation
- a StillImageRepresentation-Resource would look like this:
```hcl
 StillImageRepresentation "Image2D"{
   ontology = "rosetta"
   labels {
    en = "2-dimensional image"
    de = "add"
    fr = "add"
    it = "add"
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
- the name of the res-prop should correspond to a property [see section "properties"]
- every res-prop consists of:
 <ol style="padding-left: 40px">
  <li>cardinality which describes the number of values that can be attached</li>
  <li>gui_order which defines the position in the resource</li>
  <li>the ontology the property is part of</li>
</ol>

- only a few values are allowed in <em>cardinality</em>: 
  - 0-1
  - 0-n
  - 1
  - 1-n

# structure of transform-hcl file 
- Transform-HCL is used to modify import data

### methods:
- lower
- upper
- combine 
- replace
- to_date

#### lower
 string to lowercase

#### upper
 string to uppercase

#### combine 
- two variables and fixed string elements possible (prefix, middle, suffix)

 -> e.g. combine($a_$b) where $a, $b are variables and "_" is a fixed string element in the middle

#### replace
- replace elements of a string by another string
- only fixed strings can be replaced
- words get replaced, not parts of words
- all occurrences get replaced
- multiple replace need to have a different name (e.g. replace "a" and replace "b")

#### to_date
- tries to return a DSP-Date according to data provided
