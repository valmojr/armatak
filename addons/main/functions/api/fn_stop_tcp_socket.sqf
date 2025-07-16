_armatak_tcp_socket_is_running = missionNamespace getVariable "armatak_tcp_socket_is_running";

if (_armatak_tcp_socket_is_running) then {
	missionNamespace setVariable ["armatak_tcp_socket_is_running", false];

	"armatak" callExtension ["tcp_socket:stop", []];
} else {
	_warning = format ["<t color='#FF0021'>ARMATAK</t><br/> %1", "There is no TCP Socket running to be stopped"];
	[[_warning, 1.5]] call CBA_fnc_notify;
};
