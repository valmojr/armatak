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

	class GVAR(coreModule): Module_F {
		scope = 2;
		scopeCurator = 0;
		displayname = "CoT Router";
		icon = "\a3\Modules_F_Curator\Data\iconRadio_ca.paa";
		category = QEGVAR(main,moduleCategory);
		function = QFUNC(3denCoreModuleConfig);
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

		class AttributesValues {
			size3[] = { 1, 1, -1 };
			isRectangle = 0;
		};

		class Attributes: AttributesBase {
			class GVAR(moduleInstanceAddress): Edit {
				property = QGVAR(moduleInstanceAddress);
				displayname = "TAK Server Address";
				tooltip = "TAK Server Instance Address";
				typeName = "STRING";
				defaultValue = "localhost";
			};
			class GVAR(moduleInstancePort): Edit {
				property = QGVAR(moduleInstancePort);
				displayname = "TAK Server TCP Port";
				tooltip = "TAK Server instance Port for TCP connection";
				typeName = "NUMBER";
				defaultValue = "8088";
			};
			class ModuleDescription: ModuleDescription {};
		};

		class ModuleDescription: ModuleDescription {
			description = "Generate the initial ARMATAK configuration, syncronizing all players to the TAK server instance";
			sync[] = {"LocationArea_F"};

			class LocationArea_F {
				description[] = {
					"This module will synchronize all",
					"players to the TAK server instance"
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

	class GVAR(coreModuleCurator): GVAR(coreModule) {
		scope = 1;
		scopeCurator = 2;
		function = "";
		curatorInfoType = "armatak_zeus_core_module_dialog";

		class AttributesValues {
			size3[] = { 1, 1, -1 };
			isRectangle = 0;
		};

		class Attributes: AttributesBase {
			class ModuleDescription: ModuleDescription {};
		};

		class ModuleDescription: ModuleDescription {
			description = "Generate the initial ARMATAK configuration, syncronizing all players to the TAK server instance";
			sync[] = {"LocationArea_F"};

			class LocationArea_F {
				description[] = {
					"This module will synchronize all",
					"players to the TAK server instance"
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