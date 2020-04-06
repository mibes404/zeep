# List of manual changes after re-generation

* Add optional fields in XmlCommProfileType for each XmlCommProfileType extension.
E.g.

```
 
    #[yaserde(flatten)]
    pub station: Option<XmlStationProfile>,
    #[yaserde(flatten)]
    pub ps: Option<XmlPsCommProfile>,
    #[yaserde(flatten)]
    pub sm: Option<SessionManagerCommProfXML>,

```

* Move xsi_type field from on XmlCommProfileType extension (e.g. XmlStationProfile) to XmlCommProfileType to indicate which extension instance we're dealing with

```
    #[yaserde(prefix = "xsi", rename = "type", attribute)]
    pub xsi_type: String, // XmlCommProfileType
```

* Remove "back-reference" from extension (e.g. XmlStationProfile) to main XmlCommProfileType

```

    #[yaserde(flatten)]
    pub xml_comm_profile_type: XmlCommProfileType,

```

* Add all namespaces used by extensions to Users type. You may need to delete existing namespaces if they already use same prefix
```

    namespace = "ns1: http://xml.avaya.com/schema/import_csm_agent",
    namespace = "ns2: http://xml.avaya.com/schema/import_csm_cm",
    namespace = "ns3: http://xml.avaya.com/schema/presence",
    namespace = "ns7: http://xml.avaya.com/schema/import_sessionmanager",

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