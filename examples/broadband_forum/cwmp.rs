//! THIS IS A GENERATED FILE!
//! Take care when hand editing. Changes will be lost during subsequent runs of the code generator.
//!
//! version: 0.1.10
//!

#![allow(dead_code)]
#![allow(unused_imports)]
use log::{debug, warn};
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};

pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
pub struct Header {}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
    rename = "Fault",
    namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
    prefix = "soapenv"
)]
pub struct SoapFault {
    #[yaserde(rename = "faultcode", default)]
    pub fault_code: Option<String>,
    #[yaserde(rename = "faultstring", default)]
    pub fault_string: Option<String>,
}
impl std::error::Error for SoapFault {}

impl std::fmt::Display for SoapFault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.fault_code, &self.fault_string) {
            (None, None) => Ok(()),
            (None, Some(fault_string)) => f.write_str(fault_string),
            (Some(fault_code), None) => f.write_str(fault_code),
            (Some(fault_code), Some(fault_string)) => {
                f.write_str(fault_code)?;
                f.write_str(": ")?;
                f.write_str(fault_string)
            }
        }
    }
}
pub type SoapResponse = Result<(reqwest::StatusCode, String), reqwest::Error>;

pub mod messages {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
}

pub mod types {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ID",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct Id {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "HoldRequests",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct HoldRequests {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CPEFaultCodeType",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct CpefaultCodeType {
        #[yaserde(default)]
        pub body: u32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CPEExtensionFaultCodeType",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct CpeextensionFaultCodeType {
        #[yaserde(default)]
        pub body: u32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CPEVendorFaultCodeType",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct CpevendorFaultCodeType {
        #[yaserde(default)]
        pub body: u32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ACSFaultCodeType",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct AcsfaultCodeType {
        #[yaserde(default)]
        pub body: u32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ACSVendorFaultCodeType",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct AcsvendorFaultCodeType {
        #[yaserde(default)]
        pub body: u32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "TransferFileType",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct TransferFileType {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "DownloadFileType",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct DownloadFileType {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "UploadFileType",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct UploadFileType {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "EventCodeType",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct EventCodeType {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "TimeWindowModeValueType",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct TimeWindowModeValueType {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "TransferCompleteCPEFaultCodeType",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct TransferCompleteCPEFaultCodeType {
        #[yaserde(flatten, default)]
        pub body: CpefaultCodeType,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "TransferCompleteFaultStruct",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct TransferCompleteFaultStruct {
        #[yaserde(rename = "FaultCode", prefix = "tns", default)]
        pub fault_code: FaultCode,
        #[yaserde(rename = "FaultString", prefix = "tns", default)]
        pub fault_string: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "DeploymentUnitCPEFaultCodeType",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct DeploymentUnitCPEFaultCodeType {
        #[yaserde(flatten, default)]
        pub body: CpefaultCodeType,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "DeploymentUnitFaultStruct",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct DeploymentUnitFaultStruct {
        #[yaserde(rename = "FaultCode", prefix = "tns", default)]
        pub fault_code: FaultCode,
        #[yaserde(rename = "FaultString", prefix = "tns", default)]
        pub fault_string: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CommandKeyType",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct CommandKeyType {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ObjectNameType",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct ObjectNameType {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ParameterKeyType",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct ParameterKeyType {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ParameterNames",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct ParameterNames {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ParameterValueStruct",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct ParameterValueStruct {
        #[yaserde(rename = "Name", prefix = "tns", default)]
        pub name: String,
        #[yaserde(rename = "Value", prefix = "tns", default)]
        pub value: AnySimpleType,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ParameterValueList",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct ParameterValueList {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "MethodList",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct MethodList {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "DeviceIdStruct",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct DeviceIdStruct {
        #[yaserde(rename = "Manufacturer", prefix = "tns", default)]
        pub manufacturer: String,
        #[yaserde(rename = "OUI", prefix = "tns", default)]
        pub oui: String,
        #[yaserde(rename = "ProductClass", prefix = "tns", default)]
        pub product_class: String,
        #[yaserde(rename = "SerialNumber", prefix = "tns", default)]
        pub serial_number: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "EventStruct",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct EventStruct {
        #[yaserde(rename = "EventCode", prefix = "tns", default)]
        pub event_code: EventCodeType,
        #[yaserde(rename = "CommandKey", prefix = "tns", default)]
        pub command_key: CommandKeyType,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "EventList",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct EventList {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ParameterInfoStruct",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct ParameterInfoStruct {
        #[yaserde(rename = "Name", prefix = "tns", default)]
        pub name: String,
        #[yaserde(rename = "Writable", prefix = "tns", default)]
        pub writable: bool,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ParameterInfoList",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct ParameterInfoList {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AccessListValueType",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct AccessListValueType {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AccessList",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct AccessList {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ParameterAttributeNotificationValueType",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct ParameterAttributeNotificationValueType {
        #[yaserde(default)]
        pub body: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "SetParameterAttributesStruct",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct SetParameterAttributesStruct {
        #[yaserde(rename = "Name", prefix = "tns", default)]
        pub name: String,
        #[yaserde(rename = "NotificationChange", prefix = "tns", default)]
        pub notification_change: bool,
        #[yaserde(rename = "Notification", prefix = "tns", default)]
        pub notification: ParameterAttributeNotificationValueType,
        #[yaserde(rename = "AccessListChange", prefix = "tns", default)]
        pub access_list_change: bool,
        #[yaserde(rename = "AccessList", prefix = "tns", default)]
        pub access_list: AccessList,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "SetParameterAttributesList",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct SetParameterAttributesList {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ParameterAttributeStruct",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct ParameterAttributeStruct {
        #[yaserde(rename = "Name", prefix = "tns", default)]
        pub name: String,
        #[yaserde(rename = "Notification", prefix = "tns", default)]
        pub notification: ParameterAttributeNotificationValueType,
        #[yaserde(rename = "AccessList", prefix = "tns", default)]
        pub access_list: AccessList,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ParameterAttributeList",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct ParameterAttributeList {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "TimeWindowStruct",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct TimeWindowStruct {
        #[yaserde(rename = "WindowStart", prefix = "tns", default)]
        pub window_start: u32,
        #[yaserde(rename = "WindowEnd", prefix = "tns", default)]
        pub window_end: u32,
        #[yaserde(rename = "WindowMode", prefix = "tns", default)]
        pub window_mode: TimeWindowModeValueType,
        #[yaserde(rename = "UserMessage", prefix = "tns", default)]
        pub user_message: String,
        #[yaserde(rename = "MaxRetries", prefix = "tns", default)]
        pub max_retries: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "TimeWindowList",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct TimeWindowList {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "TransferStateType",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct TransferStateType {
        #[yaserde(default)]
        pub body: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "QueuedTransferStruct",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct QueuedTransferStruct {
        #[yaserde(rename = "CommandKey", prefix = "tns", default)]
        pub command_key: CommandKeyType,
        #[yaserde(rename = "State", prefix = "tns", default)]
        pub state: TransferStateType,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "TransferList",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct TransferList {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AllQueuedTransferStruct",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct AllQueuedTransferStruct {
        #[yaserde(rename = "CommandKey", prefix = "tns", default)]
        pub command_key: CommandKeyType,
        #[yaserde(rename = "State", prefix = "tns", default)]
        pub state: TransferStateType,
        #[yaserde(rename = "IsDownload", prefix = "tns", default)]
        pub is_download: bool,
        #[yaserde(rename = "FileType", prefix = "tns", default)]
        pub file_type: TransferFileType,
        #[yaserde(rename = "FileSize", prefix = "tns", default)]
        pub file_size: u32,
        #[yaserde(rename = "TargetFileName", prefix = "tns", default)]
        pub target_file_name: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AllTransferList",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct AllTransferList {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "DeploymentUnitUUID",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct DeploymentUnitUUID {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "DeploymentUnitState",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct DeploymentUnitState {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "DefaultDeploymentUnitOperationType",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct DefaultDeploymentUnitOperationType {
        #[yaserde(text, default)]
        pub body: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "DeploymentUnitOperationType",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct DeploymentUnitOperationType {
        #[yaserde(flatten, default)]
        pub body: DeploymentUnitOperationType,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "OperationStruct",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct OperationStruct {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "InstallOpStruct",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct InstallOpStruct {
        #[yaserde(flatten, default)]
        pub operation_struct: OperationStruct,
        #[yaserde(prefix = "xsi", rename = "type", attribute)]
        pub xsi_type: String,
        #[yaserde(rename = "URL", prefix = "tns", default)]
        pub url: String,
        #[yaserde(rename = "UUID", prefix = "tns", default)]
        pub uuid: Option<DeploymentUnitUUID>,
        #[yaserde(rename = "Username", prefix = "tns", default)]
        pub username: Option<String>,
        #[yaserde(rename = "Password", prefix = "tns", default)]
        pub password: Option<String>,
        #[yaserde(rename = "ExecutionEnvRef", prefix = "tns", default)]
        pub execution_env_ref: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "UpdateOpStruct",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct UpdateOpStruct {
        #[yaserde(flatten, default)]
        pub operation_struct: OperationStruct,
        #[yaserde(prefix = "xsi", rename = "type", attribute)]
        pub xsi_type: String,
        #[yaserde(rename = "UUID", prefix = "tns", default)]
        pub uuid: Option<DeploymentUnitUUID>,
        #[yaserde(rename = "Version", prefix = "tns", default)]
        pub version: Option<String>,
        #[yaserde(rename = "URL", prefix = "tns", default)]
        pub url: Option<String>,
        #[yaserde(rename = "Username", prefix = "tns", default)]
        pub username: Option<String>,
        #[yaserde(rename = "Password", prefix = "tns", default)]
        pub password: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "UninstallOpStruct",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct UninstallOpStruct {
        #[yaserde(flatten, default)]
        pub operation_struct: OperationStruct,
        #[yaserde(prefix = "xsi", rename = "type", attribute)]
        pub xsi_type: String,
        #[yaserde(rename = "UUID", prefix = "tns", default)]
        pub uuid: DeploymentUnitUUID,
        #[yaserde(rename = "Version", prefix = "tns", default)]
        pub version: Option<String>,
        #[yaserde(rename = "ExecutionEnvRef", prefix = "tns", default)]
        pub execution_env_ref: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "OpResultStruct",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct OpResultStruct {
        #[yaserde(rename = "UUID", prefix = "tns", default)]
        pub uuid: DeploymentUnitUUID,
        #[yaserde(rename = "DeploymentUnitRef", prefix = "tns", default)]
        pub deployment_unit_ref: String,
        #[yaserde(rename = "Version", prefix = "tns", default)]
        pub version: String,
        #[yaserde(rename = "CurrentState", prefix = "tns", default)]
        pub current_state: DeploymentUnitState,
        #[yaserde(rename = "Resolved", prefix = "tns", default)]
        pub resolved: bool,
        #[yaserde(rename = "ExecutionUnitRefList", prefix = "tns", default)]
        pub execution_unit_ref_list: String,
        #[yaserde(rename = "StartTime", prefix = "tns", default)]
        pub start_time: String,
        #[yaserde(rename = "CompleteTime", prefix = "tns", default)]
        pub complete_time: String,
        #[yaserde(rename = "Fault", prefix = "tns", default)]
        pub fault: DeploymentUnitFaultStruct,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AutonOpResultStruct",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct AutonOpResultStruct {
        #[yaserde(flatten, default)]
        pub op_result_struct: OpResultStruct,
        #[yaserde(prefix = "xsi", rename = "type", attribute)]
        pub xsi_type: String,
        #[yaserde(rename = "OperationPerformed", prefix = "tns", default)]
        pub operation_performed: DeploymentUnitOperationType,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "VoucherList",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct VoucherList {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "OptionStruct",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct OptionStruct {
        #[yaserde(rename = "OptionName", prefix = "tns", default)]
        pub option_name: String,
        #[yaserde(rename = "VoucherSN", prefix = "tns", default)]
        pub voucher_sn: u32,
        #[yaserde(rename = "State", prefix = "tns", default)]
        pub state: u32,
        #[yaserde(rename = "Mode", prefix = "tns", default)]
        pub mode: i32,
        #[yaserde(rename = "StartDate", prefix = "tns", default)]
        pub start_date: String,
        #[yaserde(rename = "ExpirationDate", prefix = "tns", default)]
        pub expiration_date: Option<String>,
        #[yaserde(rename = "IsTransferable", prefix = "tns", default)]
        pub is_transferable: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "OptionList",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct OptionList {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArgStruct",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct ArgStruct {
        #[yaserde(rename = "Name", prefix = "tns", default)]
        pub name: String,
        #[yaserde(rename = "Value", prefix = "tns", default)]
        pub value: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "FileTypeArg",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct FileTypeArg {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "SetParameterValuesFault",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        prefix = "tns"
    )]
    pub struct SetParameterValuesFault {
        #[yaserde(rename = "ParameterName", prefix = "tns", default)]
        pub parameter_name: String,
        #[yaserde(rename = "FaultCode", prefix = "tns", default)]
        pub fault_code: FaultCode,
        #[yaserde(rename = "FaultString", prefix = "tns", default)]
        pub fault_string: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Fault",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct Fault {
        #[yaserde(rename = "FaultCode", prefix = "tns", default)]
        pub fault_code: FaultCode,
        #[yaserde(rename = "FaultString", prefix = "tns", default)]
        pub fault_string: Option<String>,
        #[yaserde(rename = "SetParameterValuesFault", prefix = "tns", default)]
        pub set_parameter_values_fault: Vec<SetParameterValuesFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetRPCMethods",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetRPCMethods {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetRPCMethodsResponse",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetRPCMethodsResponse {
        #[yaserde(rename = "MethodList", prefix = "tns", default)]
        pub method_list: MethodList,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "SetParameterValues",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct SetParameterValues {
        #[yaserde(rename = "ParameterList", prefix = "tns", default)]
        pub parameter_list: ParameterValueList,
        #[yaserde(rename = "ParameterKey", prefix = "tns", default)]
        pub parameter_key: ParameterKeyType,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "SetParameterValuesResponse",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct SetParameterValuesResponse {
        #[yaserde(rename = "Status", prefix = "tns", default)]
        pub status: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetParameterValues",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetParameterValues {
        #[yaserde(rename = "ParameterNames", prefix = "tns", default)]
        pub parameter_names: ParameterNames,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetParameterValuesResponse",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetParameterValuesResponse {
        #[yaserde(rename = "ParameterList", prefix = "tns", default)]
        pub parameter_list: ParameterValueList,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetParameterNames",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetParameterNames {
        #[yaserde(rename = "ParameterPath", prefix = "tns", default)]
        pub parameter_path: String,
        #[yaserde(rename = "NextLevel", prefix = "tns", default)]
        pub next_level: bool,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetParameterNamesResponse",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetParameterNamesResponse {
        #[yaserde(rename = "ParameterList", prefix = "tns", default)]
        pub parameter_list: ParameterInfoList,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "SetParameterAttributes",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct SetParameterAttributes {
        #[yaserde(rename = "ParameterList", prefix = "tns", default)]
        pub parameter_list: SetParameterAttributesList,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "SetParameterAttributesResponse",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct SetParameterAttributesResponse {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetParameterAttributes",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetParameterAttributes {
        #[yaserde(rename = "ParameterNames", prefix = "tns", default)]
        pub parameter_names: ParameterNames,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetParameterAttributesResponse",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetParameterAttributesResponse {
        #[yaserde(rename = "ParameterList", prefix = "tns", default)]
        pub parameter_list: ParameterAttributeList,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AddObject",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct AddObject {
        #[yaserde(rename = "ObjectName", prefix = "tns", default)]
        pub object_name: ObjectNameType,
        #[yaserde(rename = "ParameterKey", prefix = "tns", default)]
        pub parameter_key: ParameterKeyType,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AddObjectResponse",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct AddObjectResponse {
        #[yaserde(rename = "InstanceNumber", prefix = "tns", default)]
        pub instance_number: u32,
        #[yaserde(rename = "Status", prefix = "tns", default)]
        pub status: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "DeleteObject",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct DeleteObject {
        #[yaserde(rename = "ObjectName", prefix = "tns", default)]
        pub object_name: ObjectNameType,
        #[yaserde(rename = "ParameterKey", prefix = "tns", default)]
        pub parameter_key: ParameterKeyType,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "DeleteObjectResponse",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct DeleteObjectResponse {
        #[yaserde(rename = "Status", prefix = "tns", default)]
        pub status: i32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Download",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct Download {
        #[yaserde(rename = "CommandKey", prefix = "tns", default)]
        pub command_key: CommandKeyType,
        #[yaserde(rename = "FileType", prefix = "tns", default)]
        pub file_type: DownloadFileType,
        #[yaserde(rename = "URL", prefix = "tns", default)]
        pub url: String,
        #[yaserde(rename = "Username", prefix = "tns", default)]
        pub username: String,
        #[yaserde(rename = "Password", prefix = "tns", default)]
        pub password: String,
        #[yaserde(rename = "FileSize", prefix = "tns", default)]
        pub file_size: u32,
        #[yaserde(rename = "TargetFileName", prefix = "tns", default)]
        pub target_file_name: String,
        #[yaserde(rename = "DelaySeconds", prefix = "tns", default)]
        pub delay_seconds: u32,
        #[yaserde(rename = "SuccessURL", prefix = "tns", default)]
        pub success_url: String,
        #[yaserde(rename = "FailureURL", prefix = "tns", default)]
        pub failure_url: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "DownloadResponse",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct DownloadResponse {
        #[yaserde(rename = "Status", prefix = "tns", default)]
        pub status: i32,
        #[yaserde(rename = "StartTime", prefix = "tns", default)]
        pub start_time: String,
        #[yaserde(rename = "CompleteTime", prefix = "tns", default)]
        pub complete_time: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Reboot",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct Reboot {
        #[yaserde(rename = "CommandKey", prefix = "tns", default)]
        pub command_key: CommandKeyType,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "RebootResponse",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct RebootResponse {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetQueuedTransfers",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetQueuedTransfers {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetQueuedTransfersResponse",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetQueuedTransfersResponse {
        #[yaserde(rename = "TransferList", prefix = "tns", default)]
        pub transfer_list: TransferList,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ScheduleInform",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct ScheduleInform {
        #[yaserde(rename = "DelaySeconds", prefix = "tns", default)]
        pub delay_seconds: u32,
        #[yaserde(rename = "CommandKey", prefix = "tns", default)]
        pub command_key: CommandKeyType,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ScheduleInformResponse",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct ScheduleInformResponse {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "SetVouchers",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct SetVouchers {
        #[yaserde(rename = "VoucherList", prefix = "tns", default)]
        pub voucher_list: VoucherList,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "SetVouchersResponse",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct SetVouchersResponse {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetOptions",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetOptions {
        #[yaserde(rename = "OptionName", prefix = "tns", default)]
        pub option_name: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetOptionsResponse",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetOptionsResponse {
        #[yaserde(rename = "OptionList", prefix = "tns", default)]
        pub option_list: OptionList,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Upload",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct Upload {
        #[yaserde(rename = "CommandKey", prefix = "tns", default)]
        pub command_key: CommandKeyType,
        #[yaserde(rename = "FileType", prefix = "tns", default)]
        pub file_type: UploadFileType,
        #[yaserde(rename = "URL", prefix = "tns", default)]
        pub url: String,
        #[yaserde(rename = "Username", prefix = "tns", default)]
        pub username: String,
        #[yaserde(rename = "Password", prefix = "tns", default)]
        pub password: String,
        #[yaserde(rename = "DelaySeconds", prefix = "tns", default)]
        pub delay_seconds: u32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "UploadResponse",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct UploadResponse {
        #[yaserde(rename = "Status", prefix = "tns", default)]
        pub status: i32,
        #[yaserde(rename = "StartTime", prefix = "tns", default)]
        pub start_time: String,
        #[yaserde(rename = "CompleteTime", prefix = "tns", default)]
        pub complete_time: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "FactoryReset",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct FactoryReset {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "FactoryResetResponse",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct FactoryResetResponse {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetAllQueuedTransfers",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetAllQueuedTransfers {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetAllQueuedTransfersResponse",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetAllQueuedTransfersResponse {
        #[yaserde(rename = "TransferList", prefix = "tns", default)]
        pub transfer_list: AllTransferList,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ScheduleDownload",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct ScheduleDownload {
        #[yaserde(rename = "CommandKey", prefix = "tns", default)]
        pub command_key: CommandKeyType,
        #[yaserde(rename = "FileType", prefix = "tns", default)]
        pub file_type: DownloadFileType,
        #[yaserde(rename = "URL", prefix = "tns", default)]
        pub url: String,
        #[yaserde(rename = "Username", prefix = "tns", default)]
        pub username: String,
        #[yaserde(rename = "Password", prefix = "tns", default)]
        pub password: String,
        #[yaserde(rename = "FileSize", prefix = "tns", default)]
        pub file_size: u32,
        #[yaserde(rename = "TargetFileName", prefix = "tns", default)]
        pub target_file_name: String,
        #[yaserde(rename = "TimeWindowList", prefix = "tns", default)]
        pub time_window_list: TimeWindowList,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ScheduleDownloadResponse",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct ScheduleDownloadResponse {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CancelTransfer",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct CancelTransfer {
        #[yaserde(rename = "CommandKey", prefix = "tns", default)]
        pub command_key: CommandKeyType,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CancelTransferResponse",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct CancelTransferResponse {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ChangeDUState",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct ChangeDUState {
        #[yaserde(rename = "Operations", prefix = "tns", default)]
        pub operations: Vec<OperationStruct>,
        #[yaserde(rename = "CommandKey", prefix = "tns", default)]
        pub command_key: Option<CommandKeyType>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ChangeDUStateResponse",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct ChangeDUStateResponse {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Inform",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct Inform {
        #[yaserde(rename = "DeviceId", prefix = "tns", default)]
        pub device_id: DeviceIdStruct,
        #[yaserde(rename = "Event", prefix = "tns", default)]
        pub event: EventList,
        #[yaserde(rename = "MaxEnvelopes", prefix = "tns", default)]
        pub max_envelopes: u32,
        #[yaserde(rename = "CurrentTime", prefix = "tns", default)]
        pub current_time: String,
        #[yaserde(rename = "RetryCount", prefix = "tns", default)]
        pub retry_count: u32,
        #[yaserde(rename = "ParameterList", prefix = "tns", default)]
        pub parameter_list: ParameterValueList,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "InformResponse",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct InformResponse {
        #[yaserde(rename = "MaxEnvelopes", prefix = "tns", default)]
        pub max_envelopes: u32,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "TransferComplete",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct TransferComplete {
        #[yaserde(rename = "CommandKey", prefix = "tns", default)]
        pub command_key: CommandKeyType,
        #[yaserde(rename = "FaultStruct", prefix = "tns", default)]
        pub fault_struct: TransferCompleteFaultStruct,
        #[yaserde(rename = "StartTime", prefix = "tns", default)]
        pub start_time: String,
        #[yaserde(rename = "CompleteTime", prefix = "tns", default)]
        pub complete_time: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "TransferCompleteResponse",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct TransferCompleteResponse {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AutonomousTransferComplete",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct AutonomousTransferComplete {
        #[yaserde(rename = "AnnounceURL", prefix = "tns", default)]
        pub announce_url: String,
        #[yaserde(rename = "TransferURL", prefix = "tns", default)]
        pub transfer_url: String,
        #[yaserde(rename = "IsDownload", prefix = "tns", default)]
        pub is_download: bool,
        #[yaserde(rename = "FileType", prefix = "tns", default)]
        pub file_type: TransferFileType,
        #[yaserde(rename = "FileSize", prefix = "tns", default)]
        pub file_size: u32,
        #[yaserde(rename = "TargetFileName", prefix = "tns", default)]
        pub target_file_name: String,
        #[yaserde(rename = "FaultStruct", prefix = "tns", default)]
        pub fault_struct: TransferCompleteFaultStruct,
        #[yaserde(rename = "StartTime", prefix = "tns", default)]
        pub start_time: String,
        #[yaserde(rename = "CompleteTime", prefix = "tns", default)]
        pub complete_time: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AutonomousTransferCompleteResponse",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct AutonomousTransferCompleteResponse {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Kicked",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct Kicked {
        #[yaserde(rename = "Command", prefix = "tns", default)]
        pub command: String,
        #[yaserde(rename = "Referer", prefix = "tns", default)]
        pub referer: String,
        #[yaserde(rename = "Arg", prefix = "tns", default)]
        pub arg: String,
        #[yaserde(rename = "Next", prefix = "tns", default)]
        pub next: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "KickedResponse",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct KickedResponse {
        #[yaserde(rename = "NextURL", prefix = "tns", default)]
        pub next_url: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "RequestDownload",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct RequestDownload {
        #[yaserde(rename = "FileType", prefix = "tns", default)]
        pub file_type: DownloadFileType,
        #[yaserde(rename = "FileTypeArg", prefix = "tns", default)]
        pub file_type_arg: FileTypeArg,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "RequestDownloadResponse",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct RequestDownloadResponse {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "DUStateChangeComplete",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct DustateChangeComplete {
        #[yaserde(rename = "Results", prefix = "tns", default)]
        pub results: Vec<OpResultStruct>,
        #[yaserde(rename = "CommandKey", prefix = "tns", default)]
        pub command_key: Option<CommandKeyType>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "DUStateChangeCompleteResponse",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct DustateChangeCompleteResponse {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AutonomousDUStateChangeComplete",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct AutonomousDUStateChangeComplete {
        #[yaserde(rename = "Results", prefix = "tns", default)]
        pub results: Vec<AutonOpResultStruct>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AutonomousDUStateChangeCompleteResponse",
        namespace = "tns: urn:dslforum-org:cwmp-1-2",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct AutonomousDUStateChangeCompleteResponse {}
}

pub mod ports {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
}

pub mod bindings {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
}

pub mod services {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
}

pub mod multiref {
    //! This module contains the `MultiRef` type which is a wrapper around `Rc<RefCell<T>>` that implements `YaDeserialize` and `YaSerialize` for `T` and allows for multiple references to the same object.
    //! Inspired by [this](https://github.com/media-io/yaserde/issues/165#issuecomment-1810243674) comment on the yaserde repository.
    //! Needs `xml-rs` and `yaserde` as dependencies.

    use std::{cell::RefCell, ops::Deref, rc::Rc};
    use yaserde::{YaDeserialize, YaSerialize};

    pub struct MultiRef<T> {
        inner: Rc<RefCell<T>>,
    }

    impl<T: YaDeserialize + YaSerialize> YaDeserialize for MultiRef<T> {
        fn deserialize<R: std::io::prelude::Read>(
            reader: &mut yaserde::de::Deserializer<R>,
        ) -> Result<Self, String> {
            let inner = T::deserialize(reader)?;
            Ok(Self {
                inner: Rc::new(RefCell::new(inner)),
            })
        }
    }

    impl<T: YaDeserialize + YaSerialize> YaSerialize for MultiRef<T> {
        fn serialize<W: std::io::prelude::Write>(
            &self,
            writer: &mut yaserde::ser::Serializer<W>,
        ) -> Result<(), String> {
            self.inner.as_ref().borrow().serialize(writer)?;
            Ok(())
        }

        fn serialize_attributes(
            &self,
            attributes: Vec<xml::attribute::OwnedAttribute>,
            namespace: xml::namespace::Namespace,
        ) -> Result<
            (
                Vec<xml::attribute::OwnedAttribute>,
                xml::namespace::Namespace,
            ),
            String,
        > {
            self.inner
                .as_ref()
                .borrow()
                .serialize_attributes(attributes, namespace)
        }
    }

    impl<T: YaDeserialize + YaSerialize + Default> Default for MultiRef<T> {
        fn default() -> Self {
            Self {
                inner: Rc::default(),
            }
        }
    }

    impl<T: YaDeserialize + YaSerialize> Clone for MultiRef<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }

    impl<T: YaDeserialize + YaSerialize + std::fmt::Debug> std::fmt::Debug for MultiRef<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.inner.as_ref().borrow().fmt(f)
        }
    }

    impl<T> Deref for MultiRef<T> {
        type Target = Rc<RefCell<T>>;
        fn deref(&self) -> &Self::Target {
            &self.inner
        }
    }
}
