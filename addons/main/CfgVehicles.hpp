class CfgFactionClasses {
	class NO_CATEGORY;
	class armatak_module_category: NO_CATEGORY {
		displayName = "Team Awareness Kit";
	};
};

class CfgVehicles {
	class Logic;
	class Module_F : Logic
	{
		class AttributesBase
		{
			class Default;
			class Edit;					// Default edit box (i.e. text input field)
			class Combo;				// Default combo box (i.e. drop-down menu)
			class Checkbox;				// Default checkbox (returned value is Boolean)
			class CheckboxNumber;		// Default checkbox (returned value is Number)
			class ModuleDescription;	// Module description
			class Units;				// Selection of units on which the module is applied
		};

		class ModuleDescription
		{
			class AnyBrain;
		};
	};
	class armatak_module_core: Module_F {
		scope = 2;
		displayname = "ARMATAK CoT Router";
		icon = "\a3\Modules_F_Curator\Data\iconRadio_ca.paa";
		category = "armatak_module_category";
		function = "armatak_fnc_init";
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
			class Units: Units {
				property = "armatak_module_attached_units";
			};
			class armatak_module_tak_server_instance_address: Edit {
				property = "armatak_module_tak_server_instance_address";
				displayname = "TAK Server Address";
				tooltip = "TAK Server Instance Address";
				typeName = "STRING";
				defaultValue = "localhost";
			};
			class armatak_module_tak_server_instance_port: Edit {
				property = "armatak_module_tak_server_instance_port";
				displayname = "TAK Server TCP Port";
				tooltip = "TAK Server instance Port for TCP connection";
				typeName = "NUMBER";
				defaultValue = "8080";
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