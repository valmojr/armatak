addMissionEventHandler ["ExtensionCallback", {
	params ["_name", "_function", "_data"];

	if (_name == "armatak_tcp_socket") then {
		_warning = format ["<t color='#00FF21'>ARMATAK</t><br/> %1", _function];
		[[_warning, 1.5]] call CBA_fnc_notify;
	};

	if (_name == "armatak_tcp_socket_error") then {
		_warning = format ["<t color='#FF0021'>ARMATAK</t><br/> %1", _function];
		[[_warning, 1.5]] call CBA_fnc_notify;
	};

	if (_name == "armatak_video") then {
		_warning = format ["<t color='#00FF21'>ARMATAK Video</t><br/> %1", _function];
		[[_warning, 1.5]] call CBA_fnc_notify;
	};

	if (_name == "armatak_video_error") then {
		_warning = format ["<t color='#FF0021'>ARMATAK Video</t><br/> %1", _function];
		[[_warning, 1.5]] call CBA_fnc_notify;
	};
}];