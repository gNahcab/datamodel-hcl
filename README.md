<h1> Validate Datamodels, Manipulate Excel-Data using the Rust Programming Language and HCL (Hashi Corp Language)</h1>

### how to install rust -> todo


### Cli Commands 
 #### how to use Cli commands -> todo
- evaluate data-model:
- evaluate transform-file:
- manipulate xlsx-data:
- manipulate csv-data:

<h2> Validate HCL-Datamodels using Rust </h2>

 - Information about the project (Shortname, Longname, Shortcode, Keywords, Descriptions, Users etc.) and Lists is not part of HCL-Datamodel(Properties with Listvalues will be dealt with later).
 - In the HCL-Datamodel we care only about ontologies, properties and resources. 


## Structure of datamodel-hcl
- Ontologies, properties and resources don't have to be in a fixed order.
 
##### ontologies
-> alle Methoden, die ein Datenmodell transformieren unter directory datamodel_parse
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
<li>the <em>name of the ontology</em> which it is part of</li>
 <li>the <em>object</em> which describes the type of value the property contains</li>
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
<li>one <em>name</em> of ontology</li>
<li>one pair of <em>labels</em></li>
<li>zero or more <em>res-props</em></li> TODO: allow ZERO
</ol>

- a resource is a container for a bunch of properties
- currently a resource can have the following types: Resource, StillImageRepresentation, TODO add all
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
  - 0-1: zero or one value
  - 0-n: zero or n values
  - 1: one value is mandatory
  - 1-n: one value is mandatory but more are allowed

## Manipulate Data

- Transform-HCL is used to manipulate imported data

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
- only CE-Dates can be transformed, but BC-Dates could be added
- months can be submitted as words and parsed. But at the moment only ASCII-Characters can be parsed
pattern is 
```
{day1/month1/year1}W/{1,2}{day1/month1/year1}W/{1,2}{day1/month1/year1}
W/{3,4}
{day2/month2/year2}W/{1,2}{day2/month2/year2}W/{1,2}{day2/month2/year2}
```
