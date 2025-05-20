class RscObject;
class RscText;
class RscFrame;
class RscLine;
class RscProgress;
class RscPicture;
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
  class controls {
    class armatak_gui_module_zeus_core_dialog_main_frame: RscPicture
    {
      idc = 1200;
      text = "#(argb,8,8,3)color(1,1,1,1)";
      x = "0.386562 * safezoneW + safezoneX";
      y = "0.346 * safezoneH + safezoneY";
      w = "0.237187 * safezoneW";
      h = "0.275 * safezoneH";
      colorBackground[] = {0,0,0,1};
    };
    class armatak_gui_module_zeus_core_dialog_address_text: RscText
    {
      idc = 1000;
      text = "TAK Server Address";
      x = "0.402031 * safezoneW + safezoneX";
      y = "0.368 * safezoneH + safezoneY";
      w = "0.216563 * safezoneW";
      h = "0.044 * safezoneH";
    };
    class armatak_gui_module_zeus_core_dialog_address_port_text: RscText
    {
      idc = 1001;
      text = "TAK Server Port";
      x = "0.402031 * safezoneW + safezoneX";
      y = "0.445 * safezoneH + safezoneY";
      w = "0.216563 * safezoneW";
      h = "0.044 * safezoneH";
    };
    class armatak_gui_module_zeus_core_dialog_address_edit: RscEdit
    {
      idc = 14000;
      text = "localhost";
      x = "0.396875 * safezoneW + safezoneX";
      y = "0.401 * safezoneH + safezoneY";
      w = "0.216563 * safezoneW";
      h = "0.044 * safezoneH";
      tooltip = "Address without protocol prefix of the TAK Server (localhost, 192.168.1.1, etcetera...)";
    };
    class armatak_gui_module_zeus_core_dialog_address_port_edit: RscEdit
    {
      idc = 14001;
      text = "8088";
      x = "0.396875 * safezoneW + safezoneX";
      y = "0.489 * safezoneH + safezoneY";
      w = "0.216563 * safezoneW";
      h = "0.044 * safezoneH";
    };
    class armatak_gui_module_zeus_core_dialog_address_button_ok: RscButton
    {
      idc = 1600;
      text = "OK";
      action = QUOTE(call FUNC(zeusCoreModuleConfig));
      x = "0.5 * safezoneW + safezoneX";
      y = "0.555 * safezoneH + safezoneY";
      w = "0.0515625 * safezoneW";
      h = "0.044 * safezoneH";
    };
    class armatak_gui_module_zeus_core_dialog_address_button_cancel: RscButton
    {
      idc = 1601;
      text = "Cancel";
      action = "closeDialog 2;";
      x = "0.561875 * safezoneW + safezoneX";
      y = "0.555 * safezoneH + safezoneY";
      w = "0.0515625 * safezoneW";
      h = "0.044 * safezoneH";
    };
  };
};
