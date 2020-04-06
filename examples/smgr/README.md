# List of manual changes after re-generation

* Add optional fields for each commprofile extension.
E.g.

```
 
#[yaserde(flatten)]
pub station: Option<XmlStationProfile>,
#[yaserde(flatten)]
pub ps: Option<XmlPsCommProfile>,
#[yaserde(flatten)]
pub sm: Option<SessionManagerCommProfXML>,

```

* Remove "back-reference" from extension (e.g. XmlStationProfile) to main XmlCommProfileType

```

#[yaserde(flatten)]
pub xml_comm_profile_type: XmlCommProfileType,

```