<h1> Validate Datamodels, Manipulate Excel-Data using the Rust Programming Language and HCL (Hashi Corp Language)</h1>

### Purpose of project
When DaSCH receives data from projects, the RDU (Research-Data-Unit) has to interpret the data and write scripts to import the data into DSP. To do that it has to manipulate the original files. 
But this is not an optimal way of dealing with the data we receive. For every project we write new import-scripts, and we might start manipulating the original data without knowing what we changed.
This project shows how and to what extent we could avoid these kinds of problems. It is a proof of concept. 
This project shows a way to export data from the original files without manipulating the original files. The data that are exported can also be manipulated.
This is how the original files and the exported data are strictly separated.

What is needed except the original file?
1. A transform-file written in HCL (Hashi Corp Language)
2. A datamodel-file written in HCL (Hashi Corp Language)

Both are written in HCL. The advantage of HCL is that its syntax is easier to read from a human perspective than JSON and allows to use declarative logic. Declarative logic declares what should be, rather than describing what is.
The idea is that this declarative style could be useful when writing a transform-file and a datamodel-file, because it would be more reader-friendly to declare how data should be exported (transform-file) or what a resource or a property should contain (datamodel-file).

A *transform-file* describes what part of the original should be exported and if the exported data should be manipulated.
This gives us two advantages:

- we receive the export of data we described in the transform-file, and everyone else can see what we exported by having a look at the transform-file
- the export can be reproduced by having the original file, the transform-file and the datamodel-file.

A file that describes how the original file has to be imported is not enough though, we also need a datamodel-file.
A *datamodel-file* describes the ontologies, properties and resources for the project. The datamodel-file in HCL replaces the datamodel that is usually written in JSON. But it only replaces the datamodel-part, it doesn't replace 
- lists
- any project-info (like users or project-name).

It doesn't contain lists or project-info because they are not part of a datamodel. Rather they should be written into separate files.

The aim of this project was to show that this is possible for xlsx-files. At a later stage it might be shown that is also possible for other formats like sql, filemaker-databases. 

The file that describes how to import the xlsx-file is written in HCL (Hashi-Corp-Language), the file that describes the datamodel is written in HCL as well.
The whole program that reads those files and imports the xlsx is written in the Rust Programming Language. The imported data is internally stored as Strings. If there is an error in the datamodel-file, transform-file or in the original data, the program should stop and a useful error-message should be returned. There is still potential to make it a lot clearer.

A remark concerning the structure of xlsx-files: The author decided that a xlsx-file that is readable is organized by rows or columns. Every row or every column describe a certain kind of data of whatever is described. This is the way of looking at the organization of an xlsx-file (or any file that is organized as a table).

At the moment the following cli-commands are available:
- evaluate a data-model
- evaluate a transform-file (a file that describes how to import the xlsx)
- read a xlsx-file and export the data as a csv-file or as a parquet-file (https://parquet.apache.org/) whereas every file contains one class of resource-instances.

#### Why parquet?

Parquet-files allow to store metadata such as the name of the resource-class, the name of columns etc. separated from the data unlike in csv or xlsx (https://parquet.apache.org/docs/overview/motivation/).
It can be read in python using 'pyarrow' (https://pypi.org/project/pyarrow/).
To import a parquet-file in Python is easy:
```python
import pyarrow.parquet as pq
path = 'path_to_my_file.parquet'
table = pq.read_table(path)
```

The rest of this Mark-down describes: 
- how to run the cli-commands.
- the cli-commands.
- the structure of a datamodel-HCL file.
- the structure of a transform-HCL file.
- and the different commands to manipulate data-vectors with transform-HCL.

### How to run commands 
Install Rust:
- follow the steps here: https://www.rust-lang.org/tools/install (if you prefer a different way, please make sure that you install 'Cargo', the build tool of Rust, as well).

Compile the files:
1. download the repository.
2. open a terminal.
3. go to the top-folder 'datamodel-hcl' and run ```cargo build```.
4. now you will be able to run the cli-commands in this folder.


### Cli-commands 
 #### How to use cli-commands:
- validate data-model:```validate --type datamodel {path}```.
- validate transform-file:```validate --type transform {path}```.
- manipulate xlsx-data: returns one csv-or parquet-file (return-format is either 'csv' or 'parquet') per xlsx-sheet: ```xlsx --return-format {return-format} {datamodel-path} {transform-path} {xlsx-path}```.

> example-files to run: 
> 
> - /data/testdata/rosetta.hcl
> - /data/testdata/transform_xlsx.hcl
> - /data/testdata/test_file_xlsx_col.xlsx

1. for every command: open a terminal.
2. go to the top-folder of the project 'datamodel-hcl'.
3. type in ```./target/debug/datamodel-hcl-cli {command}```.

e.g. a full command to transform xlsx-data to parquet would look like this:  ```./target/debug/datamodel-hcl-cli xlsx parquet ../data.xlsx ../datamodel.hcl ../transform.hcl```.

<mark>important:</mark> write absolute paths, not relative paths.

<h2> Validate datamodel in HCL using Rust </h2>

 - Information about the project (shortname, longname, shortcode, keywords, descriptions, users etc.) and lists is not part of datamodel-HCL (properties with list-values will be dealt with later).
 - In the Datamodel-HCL we care only about ontologies, properties and resources. 


## Structure of datamodel-HCL
- Ontologies, properties and resources don't have to be in a fixed order.
 
##### Ontologies
- A complete ontology looks like this:
 
```hcl
ontology "rosetta" {
  label = "rosetta"
}
```
- Multiple ontologies in one datamodel are possible.

##### Properties
- every property consists of: 
 <ol style="padding-left: 40px">
<li>the <em>name</em> which is provided outside the brackets.</li>
<li>the <em>name of the ontology</em> which it is part of.</li>
 <li>the <em>object</em> which describes the type of value the property contains.</li>
 <li>the <em>labels</em> that describe the property in multiple languages.</li>
</ol>

- this complete property with the name *hasPagenum* looks like this:
```hcl
property "hasPagenum" {
  ontology = "rosetta"
  object = "IntValue"
  labels {
    en = "pagination"
    de = "pagination"
    fr = "pagination"
  }
  gui_element = "Simpletext"
}
```
- gui_element: only necessary for object "TextValue" ('Simpletext', 'Textarea', 'Richtext'), "IntValue" or "DecimalValue" (both 'Simpletext' or 'Spinbox'). In other cases it can be deleted.

 <mark>important</mark>: some adjustments have to be made in order to make gui_element function, that's why  gui_element is ignored at the moment.

##### Resources
- every resource consists of:
<ol style="padding-left: 40px">
<li>one <em>name</em> of ontology.</li>
<li>one pair of <em>labels.</em></li>
<li>zero or more <em>res-props.</em></li>
</ol>
<mark>important</mark>: zero props is not allowed at the moment.

- a resource is a container for a fixed number of properties.
- currently a resource can have the following types: Resource, StillImageRepresentation.
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

###### Labels
 should contain one or more language-tags. Currently the following languages are supported:
- English (en)
- German (de)
- French (fr)
- Italian (it)
- Romansh (ro)

###### Res-props
- a res-prop is a representation of a property within the resource.
- the name of the res-prop should correspond to a property (see section *properties*).
- every res-prop consists of:
 <ol style="padding-left: 40px">
  <li>the <em>cardinality</em>-tag which describes the number of values that can be attached.</li>
  <li>the <em>gui_order</em>-tag which defines the position of the res-prop in the resource-order.</li>
  <li>the <em>ontology</em>-tag the property is part of.</li>
</ol>

- only a few values are allowed for the <em>cardinality</em>-tag: 
  - 0-1: zero or one value.
  - 0-n: zero or n values.
  - 1: one value is mandatory.
  - 1-n: one value is mandatory but more are allowed.

## Transform HCL to export data 

- to export the data from the original file we use a transform-file written in HCL (Hashi Corp Language).
- the data we export can be manipulated during export.
- only xlsx works at the moment, that's why we restrict this readme to how the xlsx-transform file works.

### Structure of xlsx transform-file
- the structure of the transform-file that exports from xlsx is described here. At the moment, an export is only possible from xlsx.
- a xlsx-transform-file has two levels: base level and the sheets.
- on the base level we describe what kind of file we transform (xlsx, csv, Filemaker-Database, Postgres-Database etc.) and, in the case of xlsx, we additionally describe which xlsx-sheets of the file we export from.
- on the level sheet we describe what and how we want to export from a sheet.

#### First-level attributes:
- <em>transform</em>: what file format will be transformed (only xlsx at the moment).

#### Additional tags for xlsx:
- <em>sheets</em>: the sheets that shall be described, e.g. if sheet 1, 2, 3 should be described we write sheets = [1,2,3]. If all sheets should be described we just write sheets = "all".

```hcl
transform = "xlsx"
sheets = "all"
``` 
#### Sheet
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
Sheet:
- structured_by: "column" or "row". Is the data organized by column or row?
- headers: true or false, do headers exist or not?
- resource: a resource-name, which should match a resource in the data-model.
- assignments: headers or numbers of columns or rows are assigned to a property-name or id or label.
- transformations: methods with which we write new columns with manipulated values.

#### Assignments
```hcl
  assignments  {
    id = "ID"
    not_lowered = 1
    hasName = 2
    hasChildren = 4
    hasExternalLink = 5
  }
```
- on the left hand side, the assigned name is noted, and on the right hand side, the Header-String (if headers are set to 'true') or the number of the column/row (headers can be set to 'true' or 'false') can be found.
- a column can only be assigned once.
- a column/row has to be assigned to 'id' and 'label' or has to be defined as output in a method in transformations, because a resource has to have an 'id' and a 'label'.
### Methods:
> no method manipulates the original column(s), the manipulated values are always saved as a copy under a new header.
- lower
- upper
- combine 
- replace
- to_date

#### Lower
 Values of a variable to lowercase.
 - input-variable: defines which column should be copied and lowered.
 - output-variable: defines the name of the lowered column.
```hcl
 lower "output-variable" {
 input = "input-variable"
 }
```
---
#### Upper
 Values of a variable to uppercase.
- input-variable: defines which column should be copied and rewritten with upper-case letters.
- output-variable: defines the name of the column with the lines rewritten with upper-case letters.
```hcl
 upper "output-variable" {
 input = "input-variable"
 }
```
---
#### Combine 
 Combines the values of two variables, prefix-, separator- (middle part) and suffix-element can be defined.
- array of two input-variables: defines which columns should be combined.
- output-variable: defines the name of the new combined column.
- separator (not mandatory).
- prefix (not mandatory).
- suffix (not mandatory).

```hcl
 combine "output-variable" {
  input = [input-variable_1, input-variable_2]
  separator = "_"
  prefix = "BIZ_"
  suffix = "_ZIP"
}
```
---

#### Replace 
Replace values or parts of values of a variable.
- input-variable: defines in which column the values to be replaced can be found.
- output-variable: defines the name of the column with replaced values.
- condition:
  - behavior: defines if every occurrence (greedy) or only one occurrence of a value should be replaced (lazy).
  - target: only whole words (whole) or whole words and parts of words (part) get replaced.
  - example: we want to remove "rosetta" from the value "rosetta_Image2D_rosetta". Concerning <em>behavior</em>-tag we can decide if we want to remove every occurrence of "rosetta" (greedy) or only the first occurrence (lazy). Concerning <em>target</em> we can decide if we only target the whole word or also a part of a word. So if we decide only whole words should be targeted, nothing would change in this example. Because "rosetta_image2D_rosetta" is one word, it wouldn't match the word "rosetta", we would have to choose <em>part</em> instead of whole.
  
```hcl
replace "output-variable" {
  input = "input-variable"
  old = "before"
  new = "after"
  condition {
    behavior = "lazy"
    target = "part"
  }
    }
```
---
#### To_date

##### Date
 Return a DSP-Date or DSP-Period according to provided model(s).
- every column with dates can contain different date-patterns, every pattern must be described and caught.
- "Julian" and "Gregorian" as calender-type.
- only CE-Dates can be transformed for now.
- months can be submitted as words and parsed. But at the moment only ASCII-Characters can be parsed.
- every date in a pattern must tell in which order day, month and year are represented, for this we use numbers.

The pattern that is used allows usually around 1-2 not-aA-aZ-characters, the pattern usually looks like that:
```
{day1/month1/year1}W/{1,2}{day1/month1/year1}W/{1,2}{day1/month1/year1}
W/{3,4}
{day2/month2/year2}W/{1,2}{day2/month2/year2}W/{1,2}{day2/month2/year2}
```
- the simplest case would be a date with only a year, e.g. 2001. Described in a pattern, this would look like this:
```hcl
  date {
  year = 1
}
```
- a month and a year, e.g. 02.2001, would look like that: 
```hcl
  date {
  month = 1
  year = 2
}
```
- if month and year are reversed, e.g. 2001.02, we would write:
```hcl
  date {
  month = 2
  year = 1
}
```
- for a date with day, month, year, e.g. 01.10.187, we would write:
```hcl
  date {
  day = 1
  month = 2
  year = 3
}
```
- in some cases, the month is written as a word ("January", "Jan.", "Jan" etc.), e.g. 01. February 1887, in this case we would write:
```hcl
  date {
  day = 1
  month = 2
  month_word = true
  year = 3
}
```
- note: at the moment, only ASCII-Characters can be used, so any UTF8-Characters like ä, é in März or Février cannot be parsed:
#### Period
- a period is described as two dates, e.g. the following period 01 - 03.02.1992 would be described like this:
```hcl
first {
  day = 1
}
date {
  day = 1
  month = 2
  year = 3
}
```
#### Procedure of patterns
- as mentioned, every pattern in a column has to be explicitly described. The program cannot know which pattern it has to use for which value, instead it will try with every pattern until it succeeds or runs out of patterns (in that case, the program stops and an error is returned, which should highlight what the problem is)
- the order in which the patterns are consulted is fixed by the number after the word "pattern", pattern "1" is used first, then pattern "2", and so on. 

a full example looks like this:

```hcl
to_date "hasDate" {
  input = 6
  calendar_type= "Gregorian"
  pattern "1" {
      // e.g. 1.1 - 23 Dezember 1991
      first {
          month = 1
          day = 2
      }
      date {
          day = 1
          month = 2
          month_word = true
          year = 3
      }
  }
  pattern "2" {
      // e.g. 1 - 23 Dezember 1991
      first {
          month = 1
      }
      date {
          day = 1
          month = 2
          month_word = true
          year = 3
      }
  }
  pattern "3" {
      // e.g. 23 Dezember 1991
      date {
          day = 1
          month = 2
          month_word = true
          year = 3
      }
  }
  pattern "4" {
  // e.g. 23 12 1991
      date {
          day = 1
          month = 2
          year = 3
      }
  }
}
- ```


