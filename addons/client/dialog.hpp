class RscText;
class RscBackground;
class RscButton;
class RscEdit;

class armatak_udp_socket_start_dialog {
  idd = 999091;
  movingEnable = 0;
  class ControlsBackground {
    class armatak_gui_module_udp_socket_dialog_main_frame: RscBackground {
      idc = 16960;
      x = "0.386562 * safezoneW + safezoneX";
      y = "0.357 * safezoneH + safezoneY";
      w = "0.216563 * safezoneW";
      h = "0.495 * safezoneH";
      colorBackground[]={0,0,0,0.45};
    };
  };
  class Controls {
    class armatak_gui_module_udp_socket_dialog_address_edit: RscEdit {
      idc = 16961;
      text = "192.168.15.121";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.401 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.044 * safezoneH";
      colorBackground[]={0,0,0,0.5};
    };
    class armatak_gui_module_udp_socket_dialog_gnss_port_edit: RscEdit {
      idc = 16962;
      text = "4349";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.478 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.044 * safezoneH";
      colorBackground[]={0,0,0,0.5};
    };
    class armatak_gui_module_udp_socket_dialog_mavlink_port_edit: RscEdit {
      idc = 16967;
      text = "14550";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.555 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.044 * safezoneH";
      colorBackground[]={0,0,0,0.5};
    };
    class armatak_gui_module_udp_socket_dialog_video_feed_url_edit: RscEdit {
      idc = 16969;
      text = "";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.709 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.044 * safezoneH";
      colorBackground[]={0,0,0,0.5};
      tooltip = "Optional shared feed URL. If empty, the UAV 3DEN URL is used first, then a local RTP fallback.";
    };
    class armatak_gui_module_udp_socket_dialog_lrf_port_edit: RscEdit {
      idc = 16971;
      text = "17211";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.632 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.044 * safezoneH";
      colorBackground[]={0,0,0,0.5};
      tooltip = "ATAK local Laser Range Finder UDP input. Leave empty to disable.";
    };
    class armatak_gui_module_udp_socket_dialog_address_text: RscText {
      idc = 16963;
      text = "EUD's Address";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.368 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.033 * safezoneH";
    };
    class armatak_gui_module_udp_socket_dialog_gnss_port_text: RscText {
      idc = 16964;
      text = "Network GNSS Port";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.445 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.033 * safezoneH";
    };
    class armatak_gui_module_udp_socket_dialog_mavlink_port_text: RscText {
      idc = 16968;
      text = "Mavlink Port";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.522 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.033 * safezoneH";
    };
    class armatak_gui_module_udp_socket_dialog_video_feed_url_text: RscText {
      idc = 16970;
      text = "Video Feed URL (Optional)";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.676 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.033 * safezoneH";
    };
    class armatak_gui_module_udp_socket_dialog_lrf_port_text: RscText {
      idc = 16972;
      text = "Laser Range Finder Port";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.599 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.033 * safezoneH";
    };
    class armatak_gui_module_udp_socket_dialog_address_button_cancel: RscButton {
      idc = 16965;
      text = "Cancel";
      action = "closeDialog 2;";
      x = "0.551563 * safezoneW + safezoneX";
      y = "0.786 * safezoneH + safezoneY";
      w = "0.0464063 * safezoneW";
      h = "0.055 * safezoneH";
    };
    class armatak_gui_module_udp_socket_dialog_address_button_ok: RscButton {
      idc = 16966;
      text = "Ok";
      action = QUOTE(call FUNC(startUDPSocket));
      x = "0.5 * safezoneW + safezoneX";
      y = "0.786 * safezoneH + safezoneY";
      w = "0.0464063 * safezoneW";
      h = "0.055 * safezoneH";
    };
  };
};
