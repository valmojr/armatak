class CfgVehicles {
  class Logic;

	class Module_F : Logic
	{
		class AttributesBase
		{
			class Default;
			class Edit;
			class Combo;
			class Checkbox;
			class CheckboxNumber;
			class ModuleDescription;
			class Units;
		};

		class ModuleDescription
		{
			class AnyBrain;
		};
	};
	class EGVAR(server,moduleBase);
	class GVAR(videoModule): EGVAR(server,moduleBase) {
		scope = 2;
		scopeCurator = 0;
		displayname = "Video Streaming Handler";
		icon = "\a3\Modules_F_Curator\Data\iconRadio_ca.paa";
		category = QEGVAR(main,moduleCategory);
		function = QFUNC(videoParser);
		functionPriority = 1;
		isGlobal = 0;
		isTriggerActivated = 1;
		isDisposable = 1;
		is3den = 0;
		curatorCanAttach = 0;
		curatorInfoType = "RscDisplayAttributeModuleNuke";
		canSetArea = 0;
		canSetAreaShape = 0;
		canSetAreaHeight = 0;

		class Attributes: AttributesBase {
			class GVAR(instanceAddress): Edit {
				property = QGVAR(instanceAddress);
				displayname = "MediaMTX Provider Address";
				tooltip = "MediaMTX Provider Instance Address";
				typeName = "STRING";
				defaultValue = "localhost";
			};
			class GVAR(instancePort): Edit {
				property = "armatak_module_mediamtx_video_stream_instance_port";
				displayname = "MediaMTX Provider Port";
				tooltip = "MediaMTX Provider Port for handling video streams";
				typeName = "STRING";
				defaultValue = "8554";
			};
			class GVAR(instanceAuthUser): Edit {
				property = QGVAR(instanceAuthUser);
				displayname = "MediaMTX Provider Username";
				tooltip = "MediaMTX Provider Instance Username";
				typeName = "STRING";
				defaultValue = "administrator";
			};
			class GVAR(instanceAuthPassword): Edit {
				property = QGVAR(instanceAuthPassword);
				displayname = "MediaMTX Provider Password";
				tooltip = "MediaMTX Provider Instance Password";
				typeName = "STRING";
				defaultValue = "password";
			};
		};
	};
};