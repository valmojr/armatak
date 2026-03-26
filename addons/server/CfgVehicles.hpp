class CfgVehicles {
	class Logic;
	class Module_F : Logic {
		class AttributesBase {
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

	class GVAR(connectionModuleBase): GVAR(moduleBase) {
		scopeCurator = 0;
		icon = "\a3\Modules_F_Curator\Data\iconRadio_ca.paa";
		category = QEGVAR(main,moduleCategory);
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
	};

	class GVAR(tcpModule): GVAR(connectionModuleBase) {
		scope = 2;
		displayName = "CoT Router (TCP)";
		function = QFUNC(3denTcpModuleConfig);

		class Attributes: AttributesBase {
			class GVAR(moduleInstanceAddress): Edit {
				property = QGVAR(moduleInstanceAddress);
				displayName = "TAK Server Address";
				tooltip = "Hostname or IP address for the TAK or IronTAK server.";
				typeName = "STRING";
				defaultValue = "'localhost'";
			};
			class GVAR(moduleInstancePort): Edit {
				property = QGVAR(moduleInstancePort);
				displayName = "TAK Server TCP Port";
				tooltip = "Port for the unauthenticated TCP socket.";
				typeName = "NUMBER";
				defaultValue = "8088";
			};
			class ModuleDescription: ModuleDescription {};
		};

		class ModuleDescription: ModuleDescription {
			description = "Connect ArmaTAK to a TAK server over plain TCP.";
			sync[] = {"LocationArea_F"};
		};
	};

	class GVAR(enrollModule): GVAR(connectionModuleBase) {
		scope = 2;
		displayName = "CoT Router (Authenticated)";
		function = QFUNC(3denEnrollModuleConfig);

		class Attributes: AttributesBase {
			class GVAR(moduleInstanceAddress): Edit {
				property = QGVAR(moduleInstanceAddress);
				displayname = "TAK Server Address";
				tooltip = "Hostname or IP address used for enrollment and the final TLS connection.";
				typeName = "STRING";
				defaultValue = "'localhost'";
			};
			class GVAR(moduleEnrollmentPort): Edit {
				property = QGVAR(moduleEnrollmentPort);
				displayName = "Enrollment HTTPS Port";
				tooltip = "Port used for GET /Marti/api/tls/config and POST /Marti/api/tls/signClient/v2.";
				typeName = "NUMBER";
				defaultValue = "8446";
			};
			class GVAR(moduleEnrollmentUsername): Edit {
				property = QGVAR(moduleEnrollmentUsername);
				displayName = "Enrollment Username";
				tooltip = "Username used in Basic Auth for client certificate enrollment.";
				typeName = "STRING";
				defaultValue = "''";
			};
			class GVAR(moduleEnrollmentPassword): Edit {
				property = QGVAR(moduleEnrollmentPassword);
				displayName = "Enrollment Password";
				tooltip = "Password used in Basic Auth for client certificate enrollment.";
				typeName = "STRING";
				defaultValue = "''";
			};
			class ModuleDescription: ModuleDescription {};
		};

		class ModuleDescription: ModuleDescription {
			description = "Enroll a client certificate and connect ArmaTAK over mTLS.";
			sync[] = {"LocationArea_F"};
		};
	};

	class GVAR(tcpModuleCurator): GVAR(tcpModule) {
		scope = 1;
		scopeCurator = 2;
		function = "";
		displayName = "CoT Router (TCP, Zeus)";
		curatorInfoType = "armatak_zeus_tcp_module_dialog";
	};

	class GVAR(enrollModuleCurator): GVAR(enrollModule) {
		scope = 1;
		scopeCurator = 2;
		function = "";
		displayName = "CoT Router (Authenticated, Zeus)";
		curatorInfoType = "armatak_zeus_enroll_module_dialog";
	};

	class GVAR(markEntity): GVAR(moduleBase) {
		curatorCanAttach = 1;
		category = QEGVAR(main,moduleCategory);
		displayname = "Mark Entity";
		function = QFUNC(routerEntityAdd);
		icon = "\a3\Modules_F_Curator\Data\iconRadio_ca.paa";
	};
};
