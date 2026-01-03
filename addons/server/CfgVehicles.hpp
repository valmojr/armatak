class CfgVehicles {
  	class Logic;
	class Module_F : Logic
	{
		class AttributesBase
		{
			class Edit;
			class ModuleDescription;
		};

		class ModuleDescription;
	};

	class GVAR(moduleBase): Module_F {
    author = PROJECT_AUTHOR;
    category = QEGVAR(main,moduleCategory);
    function = QUOTE({});
    functionPriority = 1;
    isGlobal = 1;
    isTriggerActivated = 0;
    scope = 1;
    scopeCurator = 2;
  };

	class GVAR(coreModule): GVAR(moduleBase) {
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
		};
	};

	class GVAR(coreModuleCurator): GVAR(coreModule) {
		scope = 1;
		scopeCurator = 2;
		function = "";
		displayName = "CoT Router (Zeus)";
		curatorInfoType = "armatak_zeus_core_module_dialog";
	};

	class GVAR(markEntity): GVAR(moduleBase) {
    curatorCanAttach = 1;
    category = QEGVAR(main,moduleCategory);
		displayname = "Mark Entity";
    function = QFUNC(routerEntityAdd);
    icon = "\a3\Modules_F_Curator\Data\iconRadio_ca.paa";
  };
};
