class RscText;
class RscBackground;
class RscButton;
class RscEdit;

class armatak_zeus_tcp_module_dialog {
  idd = 999991;
  movingEnable = 0;
  class ControlsBackground {
    class main_frame: RscBackground {
      idc = 1800;
      x = "0.386562 * safezoneW + safezoneX";
      y = "0.29 * safezoneH + safezoneY";
      w = "0.216563 * safezoneW";
      h = "0.32 * safezoneH";
      colorBackground[] = {0,0,0,0.45};
    };
  };
  class Controls {
    class address_text: RscText {
      idc = 1000;
      text = "TAK Server Address";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.332 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.033 * safezoneH";
    };
    class address_edit: RscEdit {
      idc = 14000;
      text = "localhost";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.365 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.044 * safezoneH";
      colorBackground[] = {0,0,0,0.5};
    };
    class port_text: RscText {
      idc = 1001;
      text = "TAK Server Port";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.425 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.033 * safezoneH";
    };
    class port_edit: RscEdit {
      idc = 14001;
      text = "8088";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.458 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.044 * safezoneH";
      colorBackground[] = {0,0,0,0.5};
    };
    class button_cancel: RscButton {
      idc = 1601;
      text = "Cancel";
      action = "closeDialog 2;";
      x = "0.551563 * safezoneW + safezoneX";
      y = "0.535 * safezoneH + safezoneY";
      w = "0.0464063 * safezoneW";
      h = "0.055 * safezoneH";
    };
    class button_ok: RscButton {
      idc = 1600;
      text = "Ok";
      action = QUOTE(call FUNC(ZeusTcpModuleConfig));
      x = "0.5 * safezoneW + safezoneX";
      y = "0.535 * safezoneH + safezoneY";
      w = "0.0464063 * safezoneW";
      h = "0.055 * safezoneH";
    };
  };
};

class armatak_zeus_enroll_module_dialog {
  idd = 999992;
  movingEnable = 0;
  class ControlsBackground {
    class main_frame: RscBackground {
      idc = 1810;
      x = "0.386562 * safezoneW + safezoneX";
      y = "0.2 * safezoneH + safezoneY";
      w = "0.216563 * safezoneW";
      h = "0.52 * safezoneH";
      colorBackground[] = {0,0,0,0.45};
    };
  };
  class Controls {
    class address_text: RscText {
      idc = 1010;
      text = "TAK Server Address";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.242 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.033 * safezoneH";
    };
    class address_edit: RscEdit {
      idc = 14100;
      text = "localhost";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.275 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.044 * safezoneH";
      colorBackground[] = {0,0,0,0.5};
    };
    class enroll_port_text: RscText {
      idc = 1011;
      text = "Enrollment HTTPS Port";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.335 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.033 * safezoneH";
    };
    class enroll_port_edit: RscEdit {
      idc = 14101;
      text = "8446";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.368 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.044 * safezoneH";
      colorBackground[] = {0,0,0,0.5};
    };
    class username_text: RscText {
      idc = 1012;
      text = "Enrollment Username";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.428 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.033 * safezoneH";
    };
    class username_edit: RscEdit {
      idc = 14102;
      text = "";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.461 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.044 * safezoneH";
      colorBackground[] = {0,0,0,0.5};
    };
    class password_text: RscText {
      idc = 1013;
      text = "Enrollment Password";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.521 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.033 * safezoneH";
    };
    class password_edit: RscEdit {
      idc = 14103;
      text = "";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.554 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.044 * safezoneH";
      colorBackground[] = {0,0,0,0.5};
    };
    class button_cancel: RscButton {
      idc = 1611;
      text = "Cancel";
      action = "closeDialog 2;";
      x = "0.551563 * safezoneW + safezoneX";
      y = "0.645 * safezoneH + safezoneY";
      w = "0.0464063 * safezoneW";
      h = "0.055 * safezoneH";
    };
    class button_ok: RscButton {
      idc = 1610;
      text = "Ok";
      action = QUOTE(call FUNC(ZeusEnrollModuleConfig));
      x = "0.5 * safezoneW + safezoneX";
      y = "0.645 * safezoneH + safezoneY";
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
