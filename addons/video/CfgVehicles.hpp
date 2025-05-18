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
  class armatak_module_core;
	class armatak_module_video_stream_core: armatak_module_core {
		scope = 2;
		displayname = "ARMATAK MediaMTX Video Feed Parser";
		icon = "\a3\Modules_F_Curator\Data\iconcuratorsetcamera_ca.paa";
		category = "armatak_module_category";
		function = "armatak_fnc_video_init";
		functionPriority = 1;
		isGlobal = 0;
		isTriggerActivated = 0;
		isDisposable = 1;
		is3den = 0;
		curatorCanAttach = 0;
		curatorInfoType = "RscDisplayAttributeModuleNuke";

		canSetArea = 0;
		canSetAreaShape = 0;
		canSetAreaHeight = 0;

		class AttributesValues {
			size3[] = { 1, 1, -1 };
			isRectangle = 0;
		};

		class Attributes: AttributesBase {
			class armatak_module_mediamtx_video_stream_instance_address: Edit {
				property = "armatak_module_mediamtx_video_stream_instance_address";
				displayname = "MediaMTX Provider Address";
				tooltip = "MediaMTX Provider Instance Address";
				typeName = "STRING";
				defaultValue = "localhost";
			};
			class armatak_module_mediamtx_video_stream_instance_port: Edit {
				property = "armatak_module_mediamtx_video_stream_instance_port";
				displayname = "MediaMTX Provider Port";
				tooltip = "MediaMTX Provider Port for handling video streams";
				typeName = "STRING";
				defaultValue = "8554";
			};
			class armatak_module_mediamtx_video_stream_instance_auth_user: Edit {
				property = "armatak_module_mediamtx_video_stream_instance_auth_user";
				displayname = "MediaMTX Provider Username";
				tooltip = "MediaMTX Provider Instance Username";
				typeName = "STRING";
				defaultValue = "administrator";
			};
			class armatak_module_mediamtx_video_stream_instance_auth_pass: Edit {
				property = "armatak_module_mediamtx_video_stream_instance_auth_pass";
				displayname = "MediaMTX Provider Password";
				tooltip = "MediaMTX Provider Instance Password";
				typeName = "STRING";
				defaultValue = "password";
			};
			class ModuleDescription: ModuleDescription {};
		};

		class ModuleDescription: ModuleDescription {
			description = "Generate the initial ARMATAK configuration, syncronizing all players to the TAK server instance";
			sync[] = {"LocationArea_F"};

			class LocationArea_F {
				description[] = {
					"First line",
					"Second line"
				};
				position = 1;
				direction = 1;
				optional = 1;
				duplicate = 1;
				synced[] = { "BluforUnit", "AnyBrain" };
			};
			class BluforUnit
			{
				description = "Short description";
				displayName = "Any BLUFOR unit";
				icon = "iconMan";
				side = 1;
			};
		};
	};
};