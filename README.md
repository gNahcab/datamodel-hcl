<h1> Validate Datamodels, Manipulate Excel-Data using the Rust Programming Language and HCL (Hashi Corp Language)</h1>

### purpose of project
When DaSCH receives data from project, the RDU(Research-Data-Unit) has to interpret and write scripts to import the data or even has to manipulate the original files. 
But this is not an optimal way of dealing with the data we receive. The idea of this project is to proof that it is possible to find a way to describe how the original data has to be manipulated in order to export it from the original files and import it into DSP without manipulating the original data.
In this way there should be a strict separation between the original files we receive and the import we create. The import from the original files is always reproducible, because we describe how it has to be imported. 
A file that describes how the original-files has to be imported is not enough tough, we also need a datamodel so that we don't need to specify everything (e.g. all the properties of a resource-class) in the transform-file.
The aim of this project was to show that this is possible for xlsx-files (and thus for csv as well) and could be for other formats like sql, filemaker-databases etc. as well (proof of concept). 

The file that describes how to import the xlsx-file is written in HCL(Hashi-Corp-Language) the file that describes the datamodel is written in HCL as well.
The whole program that reads those files and imports the xlsx is written in the Rust Programming Language.

At the moment it is possible
- to evaluate a data-model
- to evaluate a transform-file (a file that describes how to import the xlsx)
- return the xlsx-data as a csv-file or as a parquet-file (https://parquet.apache.org/) whereas every file contains one class of resource-instances.

Why parquet?

parquet allows to store metadata such as the name of the resource-class, the name of columns etc. separated from the data unlike in csv, xlsx etc. 
It can be read in python using 'pyarrow'(https://pypi.org/project/pyarrow/)
to import a parquet-file in python is easy:
```python
import pyarrow.parquet as pq
path = 'file.parquet'
table = pq.read_table(path)
```

The rest of this Mark-down describes: 
- how to run the cli-commands
- the cli-commands
- the structure of a datamodel in HCL
- the structure of a transform-HCL file 
- and the different commands to manipulate data-vectors with transform-hcl.

### how to run commands 
install Rust:
- follow the steps here: https://www.rust-lang.org/tools/install (if you prefer a different way, please make sure that you install 'Cargo', the build tool of Rust as well)
compile the files:
1. download the repository
2. open a terminal 
3. go to the top-folder 'datamodel-hcl' and run ```cargo build```
4. now you will be able to run the CLI Commands


### Cli Commands 
 #### how to use Cli commands -> todo
- evaluate data-model:```validate {path} datamodel```
- evaluate transform-file:```validate {path} transform```
- manipulate xlsx-data: returns one csv-or parquet-file(return-format is either 'csv' or 'parquet') per xlsx-sheet:  ```xlsx {return-format} {xlsx-path} {datamodel-path} {transform-path}``` 
- manipulate csv-data: not implemented 

1. for every command: open a terminal
2. go to the top-folder of the project 'datamodel-hcl'
3. type in ```./target/debug/datamodel-hcl-cli {command}```

e.g. a full command to transform xlsx-data to parquet would look like this:  ```./target/debug/datamodel-hcl-cli xlsx parquet ../data.xlsx ../datamodel.hcl ../transform.hcl```

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

## Transform HCL to manipulate Data 

- Data is imported by copying data

for parquet: see here

- Transform-HCL is used to manipulate imported data
### structure xlsx
#### first-level attributes:
- transform: what will be transformed
- sheets: the sheets that shall be described, e.g. if sheet 1, 2, 3 should be described we write sheets= [1,2,3]. If all sheets should be described we just write sheets = "all"
- sheet "nr": the description of the sheet

```hcl
transform = "xlsx"
sheets = "all"
sheet "1" {
}
``` 
#### sheet
```hcl
sheet "1" {
  structured_by = "column"
  headers       = true
  resource      = "Person"
  assignments {
  }
  transformations{
  }
}
```
first-level:
- structured_by: "column" or "row". Is the data organized by column or row?
- headers: true or false, do headers exist or not?
- resource: a resource-name, which should match a resource in the data-model
- assignments: headers or numbers of columns or rows are assigned to a property-name or id or label
- transformations: methods with which we write new columns with manipulated values

#### assignments
```hcl
  assignments  {
    id = "ID"
    not_lowered = 1
    hasName = 2
    hasChildren = 4
    hasExternalLink = 5
  }
```
- on the left side is the assigned name and on the right side is the Header-String (if headers are set to 'true') or the number of the column/row (headers can be set to 'true' or 'false').
- a column can only be assigned once.
- a column/row has to be assigned to 'id' and 'label' or has to be defined as output in a method in transformations, because a resource has to have an 'id' and a 'label'.
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
