# List of manual changes after re-generation

* Add optional fields in XmlCommProfileType for each XmlCommProfileType extension.
E.g.

Before

```
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "xmlCommProfileType", default)]
    pub struct XmlCommProfileType {
        #[yaserde(rename = "commProfileType", default)]
        pub comm_profile_type: String,
        #[yaserde(rename = "commProfileSubType", default)]
        pub comm_profile_sub_type: Option<String>,
        #[yaserde(rename = "jobId", default)]
        pub job_id: Option<String>,
    }
```

after 

```
    #[yaserde(rename = "jobId", default)]
    pub job_id: Option<String>, 
    #[yaserde(flatten)]
    pub station: Option<XmlStationProfile>,
    #[yaserde(flatten)]
    pub ps: Option<XmlPsCommProfile>,
    #[yaserde(flatten)]
    pub sm: Option<SessionManagerCommProfXML>,
    #[yaserde(flatten)]
    pub agent: Option<XmlAgentProfile>,

```

* Remove xsi_type field and xml_comm_profile_type "back-reference" from extension (e.g. XmlStationProfile) to main XmlCommProfileType

before
```
    pub struct XmlStationProfile {
        #[yaserde(flatten)]
        pub xml_comm_profile_type: XmlCommProfileType,
        #[yaserde(prefix = "xsi", rename = "type", attribute)]
        pub xsi_type: String, // XmlCommProfileType
        #[yaserde(prefix = "ns2", rename = "cmName", default)]
        pub cm_name: String,
        
    ... 
```
    
after 
```
    pub struct XmlStationProfile {
        #[yaserde(prefix = "ns2", rename = "cmName", default)]
        pub cm_name: String,


```

* Add xsi_type field to XmlCommProfileType

before:
```
    pub struct XmlCommProfileType {
        #[yaserde(rename = "commProfileType", default)]
        pub comm_profile_type: String,
 
```

after

```
    pub struct XmlCommProfileType {
        #[yaserde(prefix = "xsi", rename = "type", attribute)]
        pub xsi_type: String, // XmlCommProfileType
        #[yaserde(rename = "commProfileType", default)]
        pub comm_profile_type: String,

```
* Add all namespaces used by extensions to Users type. You may need to delete existing namespaces if they already use same prefix
before:

```
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        root = "users",
        default,
        namespace = "http://xml.avaya.com/schema/import",
        namespace = "tns: http://xml.avaya.com/schema/import",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance"
    )]
    pub struct Users { 
```

after

```

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        root = "users",
        namespace = "tns: http://xml.avaya.com/schema/import",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "agent: http://xml.avaya.com/schema/import_csm_agent",
        namespace = "csm: http://xml.avaya.com/schema/import_csm_cm",
        namespace = "ps: http://xml.avaya.com/schema/presence",
        namespace = "asm: http://xml.avaya.com/schema/import_sessionmanager",
        namespace = "ol: http://xml.avaya.com/schema/import_mem_officelinx",
        namespace = "delta: http://xml.avaya.com/schema/deltaImport",
        prefix = "tns"
    )]
    pub struct Users {

```

* Add prefix to user vec field in Users struct

before

```
    #[yaserde(rename = "user", default)]
    pub user: Vec<XmlUser>,
```

after

```
    #[yaserde(rename = "user", prefix = "tns", default)]
    pub user: Vec<XmlUser>,
```

Finally, some fields are optional on the "update user" xsd (userdeltaimport.xsd) but they are not optional on the "create user" xsd (userimport.xsd).
This may result in xml being sent to SMGR with empty values, which SMGR does not like.
In order to fix this, we've manually changed smgr.rs, User struct, fields givenName and surname and made them Optional.

before

```
        #[yaserde(rename = "surname", default)]
        pub surname: String,
        #[yaserde(rename = "givenName", default)]
        pub given_name: String,
```  