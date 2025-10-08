use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

/// The name of the callback service to subscribe to. Set this to one of the following values:
#[derive(Clone, Copy, Debug, Deserialize, Display, EnumIter, EnumString, Serialize)]
pub enum ServiceName {
  /// Callback messages sent when trigger conditions are met.
  AlertService,
  /// Asynchronous responses for all requests that change a device’s state and metadata, including:
  /// activate, suspend, restore, and deactivate. Note: the Assign and Unassign SIM Secure APIs
  /// will respond on this callback service.
  CarrierService,
  /// Asynchronous responses containing current device PRL values, in response to
  /// POST /devices/actions/prl/list requests.
  DevicePRLInformation,
  /// Asynchronous responses for all requests that change a device’s profile status,
  /// including download, enable, disable, and delete.
  DeviceProfileService,
  /// Asynchronous responses to POST /devices/availability/actions/list and
  /// POST /devices/actions/upload requests.
  DeviceService,
  /// Asynchronous responses containing information about suspended devices in response to
  /// POST /devices/suspension/status requests.
  DeviceSuspensionStatus,
  /// Asynchronous responses about device usage in response to
  /// POST /devices/usage/actions/list/aggregate requests.
  DeviceUsage,
  /// Asynchronous responses for all requests that allow registering,
  /// status check for Service Capabilities Exposure Function (SCEF) devices for
  /// notifications when there is a change in device’s state (awake/sleep).
  DiagnosticsService,
  /// This callback service provides two types of messages:
  /// The contents of SMS messages sent from your devices to 750075007500 or to 900060005010.
  /// Notification of when messages sent through POST /sms requests are sent by the network to devices.
  EnhancedConnectivityService,
  /// Receive callback messages when provisioning changes are made outside of the ThingSpace APIs,
  /// such as when a user performs one of the following provisioning actions from an interactive
  /// Verizon system: Activate, Deactivate, Suspend, Resume or Change MDN.
  ExternalProvisioningChanges,
  /// Receive callback notifications from the ThingSpace Intelligence service such as real-time
  /// network conditions, static coverage, FWA coverage, site proximity and device experience score.
  IntelligenceService,
  /// Asynchronous responses (second callback) for all requests that change a device’s state
  /// and metadata, including: activate, suspend, restore, deactivate, changedeviceserviceplan and
  /// NIDD Configuration success/failure (for NB-IoT devices only). Asynchronous responses for all
  /// requests that allow sending Non-IP Data Delivery (NIDD) data (mobile terminating or MT) to
  /// the device; and asynchronous callbacks for the NIDD data messages (mobile originating or MO)
  /// coming from the device.
  NiddService,
  /// Receive unsolicited callbacks for changes to promotional codes. A `PromoChange` callback
  /// message is sent shortly after the end of a device’s billing cycle if a promotional package
  /// was removed during the billing cycle.
  PromoChanges,
  /// Receive callback messages to notify about suspended devices that are automatically
  /// returned to active status. ThingSpace sends a callback message 7 days before a suspended
  /// device will auto-resume.
  ResumeTrackingNotification,
  /// Indicates that an SMS Message sent from a POST /sms request was received and acknowledged
  /// by the device.
  SMSDeliveryConfirmation,
  /// Aynchronous responses from PUT /devices/actions/gotostate requests.
  /// Note: A callback service cannot be registered through the REST API if the same
  /// callback service has been registered through the SOAP API.
  StateService,
  /// Notification that data will be partially or completely "Throttled".
  SubscriptionNotificationService,
  Extended,
  DevicePromoUsage,
  VIPCallbackService,
  PwnService,
  QoS,
  NetworkEventService,
}
