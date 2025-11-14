use crate::api::send_nidd;
use dioxus::prelude::*;
use thingspace_sdk::models::{DeviceID, NiddMessage};

#[component]
pub fn DeviceView(id: String) -> Element {
  // let Some(dev) = use_context::<crate::LocalSession>().devices.read().get(&id);
  rsx! {
    DeviceInfo { id: id.clone() }
    SendNiddModal { id }
  }
}

#[component]
fn DeviceInfo(id: String) -> Element {
  match use_context::<crate::LocalSession>().devices.read().get(&id) {
    Some(dev) => {
      rsx! {
        div { class: "mt-5",
          h2 { class: "text-2xl", "Devices" }
          ul { class: "list",
            li { class: "list-row",
              div { "Account Name" }
              div { "{dev.account_name}" }
            }
          }
        }
      }
    }
    None => rsx!(),
  }
}

#[component]
fn SendNiddModal(id: String) -> Element {
  rsx! {
    dialog { class: "modal", id: "send_nidd_modal",
      div { class: "modal-box relative max-w-xs md:max-w-sm",
        form { class: "absolute end-2 top-2", method: "dialog",
          button { class: "btn btn-sm btn-circle btn-ghost", "X" }
        }
        div { class: "text-center text-xl font-medium", "Send NIDD Message" }
        form {
          onsubmit: move |evt: FormEvent| {
              let imei = id.to_owned();
              async move {
                  evt.prevent_default();
                  let mut msg = NiddMessage::default();
                  for (key, val) in evt.values() {
                      if let FormValue::Text(val) = val {
                          if key == "maximum_delivery_time" {
                              msg.maximum_delivery_time = val
                                  .parse()
                                  .expect("Not a valid number");
                          } else if key == "message" {
                              msg.message = val;
                          }
                      }
                  }
                  msg.device_ids = vec![
                      DeviceID {
                          kind: "imei".to_string(),
                          id: imei,
                      },
                  ];
                  send_nidd(&msg).await;
              }
          },
          fieldset { class: "fieldset mt-5",
            legend { class: "fieldset-legend", "Maximum Delivery Time" }
            input {
              class: "input validator w-full focus:outline-0",
              name: "maximum_delivery_time",
              r#type: "number",
              min: "2",
              max: "2592000",
              placeholder: "400",
              required: true,
            }
            p { class: "validator-hint",
              "The allowed range is between 2 secs and 2592000 secs (30 days)."
            }
          }
          fieldset { class: "fieldset mt-5",
            legend { class: "fieldset-legend", "Message" }
            label { class: "input w-full focus:outline-0",
              input {
                class: "grow focus:outline-0",
                name: "message",
                placeholder: "Message",
                r#type: "text",
                required: true,
              }
            }
          }
          div { class: "mt-5 flex items-center justify-end gap-3",
            button { class: "btn btn-primary", r#type: "submit",
              // onsubmit: move |evt: FormEvent| async move {
              //     evt.prevent_default();
              // },
              "Send"
            }
          }
        }
      }
      form { class: "modal-backdrop", method: "dialog",
        button { "close" }
      }
    }
  }
}

// Device {
//     account_name: "0742644905-00001",
//     billing_cycle_end_date: DateTime {
//         date: YMD {
//             year: 2025,
//             month: 11,
//             day: 22,
//         },
//         time: Time {
//             hour: 12,
//             minute: 0,
//             second: 0,
//             millisecond: 0,
//             tz_offset_hours: 0,
//             tz_offset_minutes: 0,
//         },
//     },
//     carrier_informations: [
//         CarrierInformation {
//             carrier_name: "Verizon Wireless",
//             service_plan: "NB IOT UNL EVENT FIXED",
//             state: "active",
//         },
//     ],
//     connected: false,
//     created_at: DateTime {
//         date: YMD {
//             year: 2024,
//             month: 11,
//             day: 27,
//         },
//         time: Time {
//             hour: 22,
//             minute: 48,
//             second: 26,
//             millisecond: 0,
//             tz_offset_hours: 0,
//             tz_offset_minutes: 0,
//         },
//     },
//     device_ids: [
//         DeviceID {
//             id: "4062914013",
//             kind: "mdn",
//         },
//         DeviceID {
//             id: "311270028205048",
//             kind: "imsi",
//         },
//         DeviceID {
//             id: "350457799502610",
//             kind: "imei",
//         },
//         DeviceID {
//             id: "89148000008531108276",
//             kind: "iccId",
//         },
//         DeviceID {
//             id: "14062914013",
//             kind: "msisdn",
//         },
//         DeviceID {
//             id: "4062912483",
//             kind: "min",
//         },
//     ],
//     extended_attributes: [
//         ExtendedAttribute {
//             key: "PrimaryPlaceOfUseTitle",
//             value: None,
//         },
//         ExtendedAttribute {
//             key: "PrimaryPlaceOfUseFirstName",
//             value: Some(
//                 "JUSTINS ENGINEERING",
//             ),
//         },
//         ExtendedAttribute {
//             key: "PrimaryPlaceOfUseMiddleName",
//             value: Some(
//                 "",
//             ),
//         },
//         ExtendedAttribute {
//             key: "PrimaryPlaceOfUseLastName",
//             value: Some(
//                 "JUSTINS ENGINEERING SERVI",
//             ),
//         },
//         ExtendedAttribute {
//             key: "PrimaryPlaceOfUseSuffix",
//             value: Some(
//                 "",
//             ),
//         },
//         ExtendedAttribute {
//             key: "PrimaryPlaceOfUseAddressLine1",
//             value: Some(
//                 "30 VIRGINIA AVE",
//             ),
//         },
//         ExtendedAttribute {
//             key: "PrimaryPlaceOfUseAddressLine2",
//             value: Some(
//                 "",
//             ),
//         },
//         ExtendedAttribute {
//             key: "PrimaryPlaceOfUseCity",
//             value: Some(
//                 "WEST SPRINGFIELD",
//             ),
//         },
//         ExtendedAttribute {
//             key: "PrimaryPlaceOfUseState",
//             value: Some(
//                 "MA",
//             ),
//         },
//         ExtendedAttribute {
//             key: "PrimaryPlaceOfUseCountry",
//             value: Some(
//                 "USA",
//             ),
//         },
//         ExtendedAttribute {
//             key: "PrimaryPlaceOfUseZipCode",
//             value: Some(
//                 "01089",
//             ),
//         },
//         ExtendedAttribute {
//             key: "PrimaryPlaceOfUseZipCode4",
//             value: Some(
//                 "2251",
//             ),
//         },
//         ExtendedAttribute {
//             key: "PrimaryPlaceOfUseCBRPhone",
//             value: None,
//         },
//         ExtendedAttribute {
//             key: "PrimaryPlaceOfUseCBRPhoneType",
//             value: None,
//         },
//         ExtendedAttribute {
//             key: "PrimaryPlaceOfUseEmailAddress",
//             value: Some(
//                 "",
//             ),
//         },
//         ExtendedAttribute {
//             key: "AccountNumber",
//             value: Some(
//                 "0742644905-00001",
//             ),
//         },
//         ExtendedAttribute {
//             key: "SmsrOid",
//             value: None,
//         },
//         ExtendedAttribute {
//             key: "ProfileStatus",
//             value: None,
//         },
//         ExtendedAttribute {
//             key: "SkuNumber",
//             value: Some(
//                 "VZW200001820001",
//             ),
//         },
//         ExtendedAttribute {
//             key: "CostCenterCode",
//             value: Some(
//                 "",
//             ),
//         },
//         ExtendedAttribute {
//             key: "PreIMEI",
//             value: Some(
//                 "350457799502610",
//             ),
//         },
//         ExtendedAttribute {
//             key: "PreSKU",
//             value: None,
//         },
//         ExtendedAttribute {
//             key: "SIMOTADate",
//             value: Some(
//                 "2024-11-27T23:02:38Z",
//             ),
//         },
//         ExtendedAttribute {
//             key: "RoamingStatus",
//             value: Some(
//                 "Unavailable",
//             ),
//         },
//         ExtendedAttribute {
//             key: "LastRoamingStatusUpdate",
//             value: Some(
//                 "2025-10-27T17:02:02Z",
//             ),
//         },
//         ExtendedAttribute {
//             key: "DeviceId",
//             value: Some(
//                 "398362701",
//             ),
//         },
//     ],
//     group_names: [
//         "Default: 0742644905-00001",
//     ],
//     last_activation_by: "Justin Forgue",
//     last_activation_date: DateTime {
//         date: YMD {
//             year: 2024,
//             month: 11,
//             day: 27,
//         },
//         time: Time {
//             hour: 22,
//             minute: 50,
//             second: 10,
//             millisecond: 0,
//             tz_offset_hours: 0,
//             tz_offset_minutes: 0,
//         },
//     },
//     last_connection_date: DateTime {
//         date: YMD {
//             year: 2025,
//             month: 10,
//             day: 27,
//         },
//         time: Time {
//             hour: 17,
//             minute: 2,
//             second: 2,
//             millisecond: 0,
//             tz_offset_hours: 0,
//             tz_offset_minutes: 0,
//         },
//     },
// }
