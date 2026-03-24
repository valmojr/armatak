class CfgVehicles {
  	class Logic;
		class Module_F : Logic
		{
			class AttributesBase
			{
				class Edit;
				class Combo;
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
				class GVAR(moduleTransportMode): Combo {
					property = QGVAR(moduleTransportMode);
					displayname = "Transport Mode";
					tooltip = "Choose how ArmaTAK connects to the TAK or IronTAK server.";
					typeName = "STRING";
					defaultValue = "'tcp'";
					class Values {
						class tcp {
							name = "TCP 8088 (Unauthenticated)";
							value = "tcp";
							default = 1;
						};
						class manual_mtls {
							name = "mTLS (Manual Certificate)";
							value = "manual_mtls";
						};
						class enroll_mtls {
							name = "mTLS (GET config + POST signClient)";
							value = "enroll_mtls";
						};
					};
				};
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
				class GVAR(moduleTlsServerName): Edit {
					property = QGVAR(moduleTlsServerName);
					displayname = "TLS Server Name";
					tooltip = "Optional hostname used for TLS certificate validation. Leave blank to reuse the address host.";
					typeName = "STRING";
					defaultValue = "";
				};
				class GVAR(moduleTlsCaCertPath): Edit {
					property = QGVAR(moduleTlsCaCertPath);
					displayname = "TLS CA Cert Path";
					tooltip = "PEM path for the CA that signs the IronTAK or TAK server certificate.";
					typeName = "STRING";
					defaultValue = "";
				};
				class GVAR(moduleTlsClientCertPath): Edit {
					property = QGVAR(moduleTlsClientCertPath);
					displayname = "TLS Client Cert Path";
					tooltip = "PEM path for the client certificate used by this ArmaTAK session.";
					typeName = "STRING";
					defaultValue = "";
				};
				class GVAR(moduleTlsClientKeyPath): Edit {
					property = QGVAR(moduleTlsClientKeyPath);
					displayname = "TLS Client Key Path";
					tooltip = "PEM path for the private key that matches the client certificate.";
					typeName = "STRING";
					defaultValue = "";
				};
				class GVAR(moduleEnrollmentPort): Edit {
					property = QGVAR(moduleEnrollmentPort);
					displayname = "Enrollment HTTPS Port";
					tooltip = "Port used for GET /Marti/api/tls/config and POST /Marti/api/tls/signClient/v2.";
					typeName = "NUMBER";
					defaultValue = "8446";
				};
				class GVAR(moduleEnrollmentUsername): Edit {
					property = QGVAR(moduleEnrollmentUsername);
					displayname = "Enrollment Username";
					tooltip = "Socket enrollment username for Basic Auth.";
					typeName = "STRING";
					defaultValue = "";
				};
				class GVAR(moduleEnrollmentPassword): Edit {
					property = QGVAR(moduleEnrollmentPassword);
					displayname = "Enrollment Password";
					tooltip = "Socket enrollment password for Basic Auth.";
					typeName = "STRING";
					defaultValue = "";
				};
				class GVAR(moduleEnrollmentClientUid): Edit {
					property = QGVAR(moduleEnrollmentClientUid);
					displayname = "Enrollment Client UID";
					tooltip = "Optional device identifier sent as clientUid. Leave blank to auto-generate.";
					typeName = "STRING";
					defaultValue = "";
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
