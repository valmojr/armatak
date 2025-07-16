class RscText;
class RscBackground;
class RscButton;
class RscShortcutButton;
class RscEdit;

class armatak_udp_socket_start_dialog {
  idd = 999091;
  movingEnable = 0;
  class ControlsBackground {
    class armatak_gui_module_udp_socket_dialog_main_frame: RscBackground {
      idc = 16960;
      x = "0.386562 * safezoneW + safezoneX";
      y = "0.401 * safezoneH + safezoneY";
      w = "0.216563 * safezoneW";
      h = "0.242 * safezoneH";
      colorBackground[]={0,0,0,0.45};
    };
  };
  class Controls {
    class armatak_gui_module_udp_socket_dialog_address_edit: RscEdit {
      idc = 16961;
      text = "168.15.0.3";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.445 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.044 * safezoneH";
      colorBackground[]={0,0,0,0.5};
    };
    class armatak_gui_module_udp_socket_dialog_address_port_edit: RscEdit {
      idc = 16962;
      text = "4349";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.522 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.044 * safezoneH";
      colorBackground[]={0,0,0,0.5};
    };
    class armatak_gui_module_udp_socket_dialog_address_text: RscText {
      idc = 16963;
      text = "Phone's Socket Local Address";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.412 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.033 * safezoneH";
    };
    class armatak_gui_module_udp_socket_dialog_address_port_text: RscText {
      idc = 16964;
      text = "Phone's Socket Local Port";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.489 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.033 * safezoneH";
    };
    class armatak_gui_module_udp_socket_dialog_address_button_cancel: RscButton {
      idc = 16965;
      text = "Cancel";
      action = "closeDialog 2;";
      x = "0.551563 * safezoneW + safezoneX";
      y = "0.577 * safezoneH + safezoneY";
      w = "0.0464063 * safezoneW";
      h = "0.055 * safezoneH";
    };
    class armatak_gui_module_udp_socket_dialog_address_button_ok: RscButton {
      idc = 16966;
      text = "Ok";
      action = QUOTE(call FUNC(startUDPSocket));
      x = "0.5 * safezoneW + safezoneX";
      y = "0.577 * safezoneH + safezoneY";
      w = "0.0464063 * safezoneW";
      h = "0.055 * safezoneH";
    };
  };
};
