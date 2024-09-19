//! Mist Event Module

use serde::{Deserialize,Serialize};
use serde_json::{Map,Value};

/// Mist Device Up/Down Payload
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct MistDeviceUpDowns {
    /// Access Point
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ap      : Option<String>,
    /// Access Point Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ap_name : Option<String>,
    /// Device Name
    pub device_name : String,
    /// Device Type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type : Option<String>,
    /// Event Type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_type : Option<String>,
    /// External IP Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_ip  : Option<String>,
    /// MAC Address
    pub mac     : String,
    /// Reason
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason  : Option<String>,
    /// Organisation Id
    pub org_id  : String,
    /// Site Id
    pub site_id : String,
    /// Site Name
    pub site_name : String,
    /// Text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text    : Option<String>,
    /// Timestamp
    pub timestamp : i64,
    /// Type
    pub r#type    : String,
}

/// Mist Device Payload
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct MistDeviceEvents {
    /// Access Point
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ap      : Option<String>,
    /// Access Point Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ap_name : Option<String>,
    /// Audit Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_id    : Option<String>,
    /// Radio Band
    #[serde(skip_serializing_if = "Option::is_none")]
    pub band        : Option<String>,
    /// Radio Bandwidth
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth   : Option<u16>,
    /// Radio Channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel     : Option<u16>,
    /// Device Name
    pub device_name : String,
    /// Device Type
    pub device_type : String,
    /// Event Type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_type : Option<String>,
    /// MAC Address
    pub mac     : String,
    /// Organization Id
    pub org_id  : String,
    /// Radio Power Level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power   : Option<u16>,
    /// PRE Bandwidth
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_bandwidth   : Option<u16>,
    /// PRE Channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_channel     : Option<u16>,
    /// PRE Power
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_power       : Option<u16>,
    /// PRE Usage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_usage       : Option<u16>,
    /// Event Reason
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason  : Option<String>,
    /// Site Id
    pub site_id : String,
    /// Site Name
    pub site_name   : String,
    /// Text (Details)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text    : Option<String>,
    /// Event Timestamp
    pub timestamp   : i64,
    /// Type
    pub r#type      : String,
    /// Usage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage       : Option<u16>,
}

/// Mist Alarm Details
#[derive(Clone,Serialize,Deserialize,Debug)]
pub struct MistAlarmDetails {
    /// Action
    pub action  : String,
    /// Category
    pub category : String,
    /// Status
    pub status  : String,
    /// Symptom
    pub symptom : String,
}

/// Mist Email Content
#[derive(Clone,Serialize,Deserialize,Debug)]
pub struct MistEmailContent {
    /// Conntected Switch
    #[serde(rename = "Connected switch")]
    pub connected_switch:   String,
    /// Event Reason
    #[serde(rename = "Reason")]
    pub reason: String,
    /// Status
    #[serde(rename = "Status")]
    pub status: String,
    /// Access Point
    pub ap  : String,
}

/// Mist - Impacted Entity
#[derive(Clone,Serialize,Deserialize,Debug)]
pub struct MistImpactedEntity {
    /// Connected Switch - MAC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_switch_mac    : Option<String>,
    /// Connected Switch - Name
    pub connected_switch_name   : String,
    /// Entity MAC
    pub entity_mac  : String,
    /// Entity Name
    pub entity_name : String,
    /// Entity Type
    pub entity_type : String,
    /// Port Id
    pub port_id : String,
}

/// Mist Alarm Payload
#[derive(Clone,Serialize,Deserialize,Debug)]
pub struct MistAlarm {
    /// Alert Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_id    : Option<String>,
    /// Alert Category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category    : Option<String>,
    /// Optional list of access point IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aps     : Option<Vec<String>>,
    /// Count of alarms seen
    pub count   : u32,
    /// Optional Detail
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail  : Option<String>,
    /// Optional Details object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details  : Option<MistAlarmDetails>,
    /// Email Content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_content : Option<MistEmailContent>,
    /// Firmware Version (Optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fw_version  : Option<String>,
    /// Group
    pub group   : String,
    /// Optional list of hostnames
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostnames : Option<Vec<String>>,
    /// Alarm Id
    pub id      : String,
    /// Timestamp of last occurance
    /// Impacted Entities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impacted_entities : Option<Vec<MistImpactedEntity>>,
    /// Impacted Entity Count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impacted_entity_count   : Option<u16>,
    /// Last Seen Timestamp
    pub last_seen: String,
    /// Model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model   : Option<String>,
    /// Organisation Id
    pub org_id  : String,
    /// Organisation Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_name    : Option<String>,
    /// Peer (Optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer    :  Option<Map<String,Value>>,
    /// Port Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_id     : Option<String>,
    /// Port Ids
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_ids    : Option<Vec<String>>,
    /// Optional reason if known
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasons : Option<Vec<String>>,
    /// Severity: CRITICAL, MAJOR, MINOR, WARN, INFO
    /// Root Cause (if known)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_cause  : Option<String>,
    /// Alarm Severity
    pub severity: String,
    /// Site Id
    pub site_id : String,
    /// Site Name
    pub site_name: String,
    /// Status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status  : Option<String>,
    /// Suggestion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion  : Option<String>,
    /// Optional list of switches
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switches : Option<Vec<String>>,
    /// ISO Timestamp of this event
    pub timestamp: f64,
    /// Categorisation of alarm
    pub r#type   : String,
}

/// Mist Audit Payload
#[derive(Clone,Serialize,Deserialize,Debug)]
pub struct MistAudits {
    /// Admin Name
    pub admin_name  : String,
    /// After
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after   : Option<String>,
    /// Before
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before  : Option<String>,
    /// Identifier
    pub id      : String,
    /// Message
    pub message : String,
    /// Organization Id
    pub org_id  : String,
    /// Site Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id : Option<String>,
    /// Site Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_name   : Option<String>,
    /// SRC IP Address
    pub src_ip  : String,
    /// Audit Timestamp
    pub timestamp   : f64,
    /// User Agent
    pub user_agent  : String,
    /// Webhook Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_id  : Option<String>,
}

/// Mist Client Session Payload
#[derive(Clone,Serialize,Deserialize,Debug)]
pub struct MistClientSessions {
    /// Access Point Id
    pub ap  : String,
    /// Access Point Name
    pub ap_name : String,
    /// Frequency Band, e.g. 2.4GHz or 5Ghz etc.
    pub band    : String,
    /// Basic SSID (Service Set ID)
    pub bssid   : String,
    /// Client Family
    pub client_family   : String,
    /// Client Manufacturer
    pub client_manufacture : String,
    /// Client Model
    pub client_model    : String,
    /// Client OS
    pub client_os   : String,
    /// Connection
    pub connect : i64,
    /// Connection Float
    pub connect_float : f64,
    /// Disconnect
    pub disconnect  : i64,
    /// Disconnect Float
    pub disconnect_float : f64,
    /// Connection Duration
    pub duration    : f64,
    /// Client MAC
    pub mac : String,
    /// Next Access Point
    pub next_ap : String,
    /// Organization Id
    pub org_id  : String,
    /// Random MAC?
    pub random_mac  : bool,
    /// Recieved Signal Strength Indicator (RSSI)
    pub rssi    : i32,
    /// Site Id
    pub site_id : String,
    /// Site Name
    pub site_name   : String,
    /// Service Set Id (SSID)
    pub ssid    : String,
    /// Termination Reason
    pub termination_reason : i32,
    /// Connect Timestamp
    pub timestamp   : i64,
    /// Version
    pub version : i32,
    /// Wireless LAN Id
    pub wlan_id : String,
}

/// Mist Client Join Payload
#[derive(Clone,Serialize,Deserialize,Debug)]
pub struct MistClientJoin {
    /// Access Point ID
    pub ap  : String,
    /// Access Point Name
    pub ap_name : String,
    /// Radio Band
    pub band    : String,
    /// Basic Service Set Identifier (SSID)
    pub bssid   : String,
    /// Connect Count
    pub connect : i64,
    /// Connect Float
    pub connect_float : f64,
    /// Media Access Control (MAC) Address
    pub mac     : String,
    /// Organisaton Id
    pub org_id  : String,
    /// Random MAC
    pub random_mac  : bool,
    /// Recieved Signal Strength Identifier
    pub rssi    : i32,
    /// Site Identier
    pub site_id : String,
    /// Site Name
    pub site_name   : String,
    /// Service Set Identifier
    pub ssid    : String,
    /// ISO Timestamp
    pub timestamp   : i64,
    /// Payload version
    pub version : i32,
    /// Wireless LAN Id
    pub wlan_id : String,
}

/// Mist Uber Payload
#[derive(Debug,Clone,Serialize,Deserialize)]
#[serde(tag = "topic")]
pub enum MistMessage {
    /// Alarm Container
    #[serde(rename = "alarms")]
    Alarms {
        /// Vec of MistAlarm payloads
        events         : Vec<MistAlarm>        
    },
    /// Audit Container
    #[serde(rename = "audits")]
    Audits {
        /// Vec of MistAudit payloads
        events         : Vec<MistAudits>       
    },
    /// ClientSession Container
    #[serde(rename = "client-sessions")]
    ClientSessions { 
        /// Vec of MistClientSessions payloads
        events    : Vec<MistClientSessions> 
    },
    /// Device Up/Down Container
    #[serde(rename = "device-updowns")]
    DeviceUpDowns { 
        /// Vec of MistDeviceUpDowns payloads
        events : Vec<MistDeviceUpDowns>
    },
    /// Device Events Container
    #[serde(rename = "device-events")]
    DeviceEvents { 
        /// Vec of MistDeviceDevents payloads
        events  : Vec<MistDeviceEvents> 
    },
    /// Client Join Container
    #[serde(rename = "client-join")]
    ClientJoin { 
        /// Vec of MistClientJoin payloads
        events : Vec<MistClientJoin> 
    },
}

/// Legacy Mist schema
#[derive(Serialize,Deserialize,Debug)]
pub struct Mist {
    /// Topic indicator
    topic : String,
    /// Vec of MistAlarm payloads
    pub events : Vec<MistAlarm>,
}

impl Default for Mist {
    fn default() -> Self {
        Mist {
            topic : "undefined".to_string(),
            events : vec![],
        }
    }
}

impl MistAlarm {
    /// Split an event by hostname
    /// # Input
    /// A normal MistAlarm event payload
    /// # Output
    /// Vector of MistAlarm payloads each with only a single hostname.
    pub fn split<F>(event : &MistAlarm, get_ci : F) -> Option<Vec<MistAlarm>> 
    where F : FnOnce(String) -> String {
        // take a given MistEvent and return a vector split on hostnames
        let mut output :Vec<MistAlarm> = vec![];
        // Since hostname is optional
        let mut match_count = 0;
        match &event.hostnames {
            Some(v) => {
                // We have some hostnames, clone ourselves for each hostname
                let cv = v.clone();
                cv.iter().for_each(|e| {
                    let mut new_event = event.clone();
                    new_event.hostnames = Some(vec![e.to_string()]);
                    new_event.id.push_str(format!(".{match_count}").as_ref());
                    output.push(new_event);
                    match_count += 1;
                });
                Some(output)
            },
            None => {
                // No hostnames, try for impacted_entities instead
                match &event.impacted_entities {
                    Some(ie) => {
                        ie.iter().for_each(|e| {
                            let mut new_event = event.clone();
                            new_event.hostnames = Some(vec![e.clone().entity_name]);
                            output.push(new_event);
                            match_count += 1;
                        });
                        Some(output)
                    },
                    None => {
                        let mut new_event = event.clone();
                        new_event.hostnames = Some(vec![get_ci(event.org_id.clone())]);
                        output.push(new_event);
                        Some(output)
                    },
                }
            },
        }
    }
}