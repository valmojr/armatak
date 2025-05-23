class RscObject;
class RscText;
class RscFrame;
class RscLine;
class RscProgress;
class RscPicture;
class RscBackground;
class RscPictureKeepAspect;
class RscVideo;
class RscHTML;
class RscButton;
class RscShortcutButton;
class RscEdit;
class RscCombo;
class RscListBox;
class RscListNBox;
class RscXListBox;
class RscTree;
class RscSlider;
class RscXSliderH;
class RscActiveText;
class RscActivePicture;
class RscActivePictureKeepAspect;
class RscStructuredText;
class RscToolbox;
class RscControlsGroup;
class RscControlsGroupNoScrollbars;
class RscControlsGroupNoHScrollbars;
class RscControlsGroupNoVScrollbars;
class RscButtonTextOnly;
class RscButtonMenu;
class RscButtonMenuOK;
class RscButtonMenuCancel;
class RscButtonMenuSteam;
class RscMapControl;
class RscMapControlEmpty;
class RscCheckBox;

class armatak_zeus_core_module_dialog {
  idd = 999991;
  movingEnable = 0;
  class ControlsBackground {
    class armatak_gui_module_zeus_core_dialog_main_frame: RscBackground {
      idc = 1800;
      x = "0.386562 * safezoneW + safezoneX";
      y = "0.401 * safezoneH + safezoneY";
      w = "0.216563 * safezoneW";
      h = "0.242 * safezoneH";
      colorBackground[]={0,0,0,0.45};
    };
  };
  class Controls {
    class armatak_gui_module_zeus_core_dialog_address_edit: RscEdit {
      idc = 14000;
      text = "localhost";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.445 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.044 * safezoneH";
      colorBackground[]={0,0,0,0.5};
    };
    class armatak_gui_module_zeus_core_dialog_address_port_edit: RscEdit {
      idc = 14001;
      text = "8088";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.522 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.044 * safezoneH";
      colorBackground[]={0,0,0,0.5};
    };
    class armatak_gui_module_zeus_core_dialog_address_text: RscText {
      idc = 1000;
      text = "TAK Server Address";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.412 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.033 * safezoneH";
    };
    class armatak_gui_module_zeus_core_dialog_address_port_text: RscText {
      idc = 1001;
      text = "TAK Server Port";
      x = "0.391719 * safezoneW + safezoneX";
      y = "0.489 * safezoneH + safezoneY";
      w = "0.20625 * safezoneW";
      h = "0.033 * safezoneH";
    };
    class armatak_gui_module_zeus_core_dialog_address_button_cancel: RscButton {
      idc = 1601;
      text = "Cancel";
      action = "closeDialog 2;";
      x = "0.551563 * safezoneW + safezoneX";
      y = "0.577 * safezoneH + safezoneY";
      w = "0.0464063 * safezoneW";
      h = "0.055 * safezoneH";
    };
    class armatak_gui_module_zeus_core_dialog_address_button_ok: RscButton {
      idc = 1600;
      text = "Ok";
      action = QUOTE(call FUNC(zeusCoreModuleConfig));
      x = "0.5 * safezoneW + safezoneX";
      y = "0.577 * safezoneH + safezoneY";
      w = "0.0464063 * safezoneW";
      h = "0.055 * safezoneH";
    };
  };
};

class armatak_zeus_custom_marker_dialog {
  idd = 990991;
  movingEnable = 0;

  class Controls {
    class RscFrame_1800: RscFrame
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