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
			class armatak_module_api_instance: Combo {
				property = "armatak_module_property_api_instance";
				displayname = "TAK API Instance";
				tooltip = "Used TAK Server Instance";
				typeName = "STRING";
				defaultValue = "ots";

				class Values {
					class ots { name = "Open TAK Server"; value = "ots"; default = 1; };
					class fts { name = "Free TAK Server"; value = "fts"; };
				};
			};
			class armatak_module_api_instance_protocol: Combo {
				property = "armatak_module_property_api_instance_protocol";
				displayname = "OTS Protocol";
				tooltip = "OpenTAKServer instance protocol";
				typeName = "STRING";
				defaultValue = "http";
			
				class Values {
					class http { name = "HTTP"; value = "http"; default = 1; };
					class https { name = "HTTPS"; value = "https"; };
				};
			};
			class armatak_module_api_instance_address: Edit {
				property = "armatak_module_property_api_instance_address";
				displayname = "OTS Address";
				tooltip = "OpenTAKServer Instance Address";
				typeName = "STRING";
				defaultValue = "localhost";
			};
			class armatak_module_api_instance_port: Edit {
				property = "armatak_module_property_api_instance_port";
				displayname = "OTS Port";
				tooltip = "OpenTAKServer Instance Port";
				typeName = "NUMBER";
				defaultValue = "8080";
			};
			class armatak_module_api_instance_username: Edit {
				property = "armatak_module_property_api_instance_username";
				displayname = "API Username";
				tooltip = "API Username for authorization";
				typeName = "STRING";
				defaultValue = "administrator";
			};
			class armatak_module_api_instance_password: Edit {
				property = "armatak_module_property_api_instance_password";
				displayname = "API Password";
				tooltip = "API Password for authorization";
				typeName = "STRING";
				defaultValue = "password";
			};
			class ModuleDescription: ModuleDescription {};
		};

		class ModuleDescription: ModuleDescription {
			description = "Generate the initial ARMATAK configuration, syncronizing all players to the OTS instance";
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