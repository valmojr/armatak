class RscText;
class RscBackground;
class RscButton;
class RscEdit;

class armatak_zeus_core_module_dialog {
  idd = 999991;
  movingEnable = 0;
  class ControlsBackground {
    class armatak_gui_module_zeus_core_dialog_main_frame: RscBackground {
      idc = 1800;
      x = "0.386562 * safezoneW + safezoneX";
      y = "0.08 * safezoneH + safezoneY";
      w = "0.216563 * safezoneW";
      h = "0.84 * safezoneH";
      colorBackground[]={0,0,0,0.45};
    };
  };
  class Controls {
    class armatak_gui_module_zeus_core_dialog_address_edit: RscEdit {
      idc = 14000;
      text = "localhost";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.185 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.044 * safezoneH";
      colorBackground[]={0,0,0,0.5};
    };
    class armatak_gui_module_zeus_core_dialog_address_port_edit: RscEdit {
      idc = 14001;
      text = "8088";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.255 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.044 * safezoneH";
      colorBackground[]={0,0,0,0.5};
    };
    class armatak_gui_module_zeus_core_dialog_transport_mode_edit: RscEdit {
      idc = 14006;
      text = "tcp";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.115 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.044 * safezoneH";
      colorBackground[]={0,0,0,0.5};
    };
    class armatak_gui_module_zeus_core_dialog_enrollment_port_edit: RscEdit {
      idc = 14007;
      text = "8446";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.325 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.044 * safezoneH";
      colorBackground[]={0,0,0,0.5};
    };
    class armatak_gui_module_zeus_core_dialog_enrollment_user_edit: RscEdit {
      idc = 14008;
      text = "";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.395 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.044 * safezoneH";
      colorBackground[]={0,0,0,0.5};
    };
    class armatak_gui_module_zeus_core_dialog_tls_name_edit: RscEdit {
      idc = 14002;
      text = "";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.465 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.044 * safezoneH";
      colorBackground[]={0,0,0,0.5};
    };
    class armatak_gui_module_zeus_core_dialog_tls_ca_edit: RscEdit {
      idc = 14003;
      text = "";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.535 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.044 * safezoneH";
      colorBackground[]={0,0,0,0.5};
    };
    class armatak_gui_module_zeus_core_dialog_tls_client_cert_edit: RscEdit {
      idc = 14004;
      text = "";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.605 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.044 * safezoneH";
      colorBackground[]={0,0,0,0.5};
    };
    class armatak_gui_module_zeus_core_dialog_tls_client_key_edit: RscEdit {
      idc = 14005;
      text = "";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.675 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.044 * safezoneH";
      colorBackground[]={0,0,0,0.5};
    };
    class armatak_gui_module_zeus_core_dialog_enrollment_password_edit: RscEdit {
      idc = 14009;
      text = "";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.745 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.044 * safezoneH";
      colorBackground[]={0,0,0,0.5};
    };
    class armatak_gui_module_zeus_core_dialog_enrollment_client_uid_edit: RscEdit {
      idc = 14010;
      text = "";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.815 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.044 * safezoneH";
      colorBackground[]={0,0,0,0.5};
    };
    class armatak_gui_module_zeus_core_dialog_transport_mode_text: RscText {
      idc = 1006;
      text = "Transport Mode (tcp/manual_mtls/enroll_mtls)";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.082 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.033 * safezoneH";
    };
    class armatak_gui_module_zeus_core_dialog_address_text: RscText {
      idc = 1000;
      text = "TAK Server Address";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.152 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.033 * safezoneH";
    };
    class armatak_gui_module_zeus_core_dialog_address_port_text: RscText {
      idc = 1001;
      text = "TAK Server Port";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.222 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.033 * safezoneH";
    };
    class armatak_gui_module_zeus_core_dialog_enrollment_port_text: RscText {
      idc = 1007;
      text = "Enrollment HTTPS Port";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.292 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.033 * safezoneH";
    };
    class armatak_gui_module_zeus_core_dialog_enrollment_user_text: RscText {
      idc = 1008;
      text = "Enrollment Username";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.362 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.033 * safezoneH";
    };
    class armatak_gui_module_zeus_core_dialog_tls_name_text: RscText {
      idc = 1002;
      text = "TLS Server Name";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.432 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.033 * safezoneH";
    };
    class armatak_gui_module_zeus_core_dialog_tls_ca_text: RscText {
      idc = 1003;
      text = "TLS CA Cert Path";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.502 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.033 * safezoneH";
    };
    class armatak_gui_module_zeus_core_dialog_tls_client_cert_text: RscText {
      idc = 1004;
      text = "TLS Client Cert Path";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.572 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.033 * safezoneH";
    };
    class armatak_gui_module_zeus_core_dialog_tls_client_key_text: RscText {
      idc = 1005;
      text = "TLS Client Key Path";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.642 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.033 * safezoneH";
    };
    class armatak_gui_module_zeus_core_dialog_enrollment_password_text: RscText {
      idc = 1009;
      text = "Enrollment Password";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.712 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.033 * safezoneH";
    };
    class armatak_gui_module_zeus_core_dialog_enrollment_client_uid_text: RscText {
      idc = 1010;
      text = "Enrollment Client UID";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.782 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.033 * safezoneH";
    };
    class armatak_gui_module_zeus_core_dialog_address_button_cancel: RscButton {
      idc = 1601;
      text = "Cancel";
      action = "closeDialog 2;";
      x = "0.551563 * safezoneW + safezoneX";
      y = "0.855 * safezoneH + safezoneY";
      w = "0.0464063 * safezoneW";
      h = "0.055 * safezoneH";
    };
    class armatak_gui_module_zeus_core_dialog_address_button_ok: RscButton {
      idc = 1600;
      text = "Ok";
      action = QUOTE(call FUNC(zeusCoreModuleConfig));
      x = "0.5 * safezoneW + safezoneX";
      y = "0.855 * safezoneH + safezoneY";
      w = "0.0464063 * safezoneW";
      h = "0.055 * safezoneH";
    };
  };
};

class armatak_zeus_custom_marker_dialog {
  idd = 990991;
  movingEnable = 0;

  class Controls {
    class RscFrame_1800: RscBackground
    {
      idc = 1800;
      x = "0.37625 * safezoneW + safezoneX";
      y = "0.357 * safezoneH + safezoneY";
      w = "0.237187 * safezoneW";
      h = "0.275 * safezoneH";
    };
    class RscEdit_1400: RscEdit
    {
      idc = 1400;
      x = "0.386562 * safezoneW + safezoneX";
      y = "0.423 * safezoneH + safezoneY";
      w = "0.216563 * safezoneW";
      h = "0.044 * safezoneH";
    };
    class RscText_1000: RscText
    {
      idc = 1000;
      text = "Entity Callsign";
      x = "0.386562 * safezoneW + safezoneX";
      y = "0.379 * safezoneH + safezoneY";
      w = "0.216563 * safezoneW";
      h = "0.044 * safezoneH";
    };
    class RscText_1001: RscText
    {
      idc = 1001;
      text = "Entity Type (only for vehicles)";
      x = "0.386562 * safezoneW + safezoneX";
      y = "0.467 * safezoneH + safezoneY";
      w = "0.216563 * safezoneW";
      h = "0.044 * safezoneH";
    };
    class RscEdit_1401: RscEdit
    {
      idc = 1401;
      x = "0.386562 * safezoneW + safezoneX";
      y = "0.511 * safezoneH + safezoneY";
      w = "0.216563 * safezoneW";
      h = "0.044 * safezoneH";
    };
    class RscButton_1600: RscButton
    {
      idc = 1600;
      text = "Cancel";
      x = "0.551562 * safezoneW + safezoneX";
      y = "0.566 * safezoneH + safezoneY";
      w = "0.0515625 * safezoneW";
      h = "0.055 * safezoneH";
    };
    class RscButton_1601: RscButton
    {
      idc = 1601;
      text = "OK";
      x = "0.489687 * safezoneW + safezoneX";
      y = "0.566 * safezoneH + safezoneY";
      w = "0.0515625 * safezoneW";
      h = "0.055 * safezoneH";
    };
  };
};
