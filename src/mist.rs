//! Mist Event Module

use serde::{Deserialize,Serialize};
use serde_json::{Map,Value};

/// Mist Device Up/Down Payload
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct MistDeviceUpDowns {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ap      : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ap_name : Option<String>,
    pub device_name : String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_type : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_ip  : Option<String>,
    pub mac     : String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason  : Option<String>,
    pub org_id  : String,
    pub site_id : String,
    pub site_name : String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text    : Option<String>,
    pub timestamp : i64,
    pub r#type    : String,
}

/// Mist Device Payload
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct MistDeviceEvents {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ap      : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ap_name : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_id    : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub band        : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth   : Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel     : Option<u16>,
    pub device_name : String,
    pub device_type : String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_type : Option<String>,
    pub mac     : String,
    pub org_id  : String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power   : Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_bandwidth   : Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_channel     : Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_power       : Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_usage       : Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason  : Option<String>,
    pub site_id : String,
    pub site_name   : String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text    : Option<String>,
    pub timestamp   : i64,
    pub r#type      : String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage       : Option<u16>,
}

#[derive(Clone,Serialize,Deserialize,Debug)]
pub struct MistAlarmDetails {
    pub action  : String,
    pub category : String,
    pub status  : String,
    pub symptom : String,
}

#[derive(Clone,Serialize,Deserialize,Debug)]
pub struct MistEmailContent {
    #[serde(rename = "Connected switch")]
    pub connected_switch:   String,
    #[serde(rename = "Reason")]
    pub reason: String,
    #[serde(rename = "Status")]
    pub status: String,
    pub ap  : String,
}

#[derive(Clone,Serialize,Deserialize,Debug)]
pub struct MistImpactedEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_switch_mac    : Option<String>,
    pub connected_switch_name   : String,
    pub entity_mac  : String,
    pub entity_name : String,
    pub entity_type : String,
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
    pub admin_name  : String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after   : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before  : Option<String>,
    pub id      : String,
    pub message : String,
    pub org_id  : String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_name   : Option<String>,
    pub src_ip  : String,
    pub timestamp   : f64,
    pub user_agent  : String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_id  : Option<String>,
}

/// Mist Client Session Payload
#[derive(Clone,Serialize,Deserialize,Debug)]
pub struct MistClientSessions {
    pub ap  : String,
    pub ap_name : String,
    pub band    : String,
    pub bssid   : String,
    pub client_family   : String,
    pub client_manufacture : String,
    pub client_model    : String,
    pub client_os   : String,
    pub connect : i64,
    pub connect_float : f64,
    pub disconnect  : i64,
    pub disconnect_float : f64,
    pub duration    : f64,
    pub mac : String,
    pub next_ap : String,
    pub org_id  : String,
    pub random_mac  : bool,
    pub rssi    : i32,
    pub site_id : String,
    pub site_name   : String,
    pub ssid    : String,
    pub termination_reason : i32,
    pub timestamp   : i64,
    pub version : i32,
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
    pub connect : i64,
    pub connect_float : f64,
    /// Media Access Control (MAC) Address
    pub mac     : String,
    /// Organisaton Id
    pub org_id  : String,
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
    Alarms { events         : Vec<MistAlarm>        },
    /// Audit Container
    #[serde(rename = "audits")]
    Audits { events         : Vec<MistAudits>       },
    /// ClientSession Container
    #[serde(rename = "client-sessions")]
    ClientSessions { events    : Vec<MistClientSessions> },
    /// Device Up/Down Container
    #[serde(rename = "device-updowns")]
    DeviceUpDowns { events : Vec<MistDeviceUpDowns>},
    /// Device Events Container
    #[serde(rename = "device-events")]
    DeviceEvents { events  : Vec<MistDeviceEvents> },
    /// Client Join Container
    #[serde(rename = "client-join")]
    ClientJoin { events : Vec<MistClientJoin> },
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Mist {
    topic : String,
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