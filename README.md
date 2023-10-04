# Versify - A simple CLI tool for managing the versions made wiht Rust 

## Usage
<i>Help</i>
````bash
versify --help 
````

<i>Example</i>
````bash
 versify -p C:\<PATH>\packages.txt -d SSC,SCE,SATK -b 4.0.8.333,1.6.4.5,5.3.33
````
You can use to change on or several versions for different apps (comma separated).   
The number of versions <strong>MUST</strong> match the number of domains to be changed, otherwise an error will be thrown.

<i>Options</i>
- -p, --path <PATH>: Path to the file directory
- -d, --domain <DOMAIN>: The name of the domain you want to modify. This is a list of valid domains: SATK, Mashup, SSC, SSIV, SCE, HCS, ImageImport, ImageDiscovery, SciStream, Metastore
- -b, --build-number <BUILD_NUMBER>: The build number of the apps, e.g. 4.0.8.10268
-  -h, --help: Print help
-  -V, --version: Print version

## TODO:
- Create a new branch and a pull request using GitHub API
