addMissionEventHandler ["ExtensionCallback", {
	params ["_name", "_function", "_data"];

	if (_name == "armatak_tcp_socket") then {
		[_function, "success", _name] call armatak_fnc_notify;
	};

	if (_name == "armatak_tcp_socket_error") then {
		[_function, "error", _name] call armatak_fnc_notify;
	};

	if (_name == "armatak_video") then {
		[_function, "success", _name] call armatak_fnc_notify;
	};

	if (_name == "armatak_video_error") then {
		[_function, "error", _name] call armatak_fnc_notify;
	};
}];