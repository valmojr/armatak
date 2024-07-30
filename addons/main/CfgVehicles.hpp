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
		displayname = "ARMATAK Core";
		icon = "\a3\Modules_F_Curator\Data\iconRadio_ca.paa";
		category = "armatak_module_category";
		function = "armatak_fnc_init";
		functionPriority = 1;
		isGlobal = 2;
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
				property = "armatak_module_property_attached_units";
			};
			class armatak_module_fts_api_instance_protocol: Combo {
				property = "armatak_module_property_fts_api_instance_protocol";
				displayname = "FTS Protocol";
				tooltip = "FreeTAKServer instance protocol";
				typeName = "STRING";
				defaultValue = "http";
			
				class Values {
					class http { name = "HTTP"; value = "http"; default = 1; };
					class https { name = "HTTPS"; value = "https"; };
				};
			};
			class armatak_module_fts_api_instance_address: Edit {
				property = "armatak_module_property_fts_api_instance_address";
				displayname = "FTS Address";
				tooltip = "FreeTAKServer Instance Address";
				typeName = "STRING";
				defaultValue = "localhost";
			};
			class armatak_module_fts_api_instance_port: Edit {
				property = "armatak_module_property_fts_api_instance_port";
				displayname = "FTS Port";
				tooltip = "FreeTAKServer Instance Port";
				typeName = "NUMBER";
				defaultValue = "19023";
			};
			class armatak_module_fts_api_instance_token: Edit {
				property = "armatak_module_property_fts_api_instance_token";
				displayname = "API Token";
				tooltip = "API Bearer Token for authorization";
				typeName = "STRING";
				defaultValue = "token";
			};
			class ModuleDescription: ModuleDescription {};
		};

		class ModuleDescription: ModuleDescription {
			description = "Generate the initial ARMATAK configuration, syncronizing all players to the FTS instance";
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