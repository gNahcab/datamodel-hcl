# Parse and validate DaSCH-Datamodels from HCL using Rust

 - Information about the project (Shortname, Longname, Shortcode, Keywords, Descriptions, Users etc.) and Lists is separated from Resources, Properties and their Ontologies
 - In the datamodel we care only about ontologies, properties and resources 

## Structure of the file
- Ontologies, properties and resources don't have to be in a fixed order
 
##### ontologies
- the datamodel must contain at least one ontology with the name and the label of the ontology, e.g:
 
```hcl
ontology "rosetta" {
  label = "rosetta"
}
```

##### properties


##### resources
 - ###### Resource
 
 - ###### StillImageRepresentation



