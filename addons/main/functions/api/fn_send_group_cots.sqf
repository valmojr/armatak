// function name: armatak_fnc_send_group_cots
// function author: Valmo
// function description: handle the cot routing for drones

params["_group"];

{
	_group_colors = missionNamespace getVariable "_group_colors";

	_group_roles = ["Team Member", "Team Lead", "HQ", "Sniper", "Medic", "Forward Observer", "RTO", "K9"];

	_group_role = "Team Member";

	_group_name = "";

	_group_name = _group getVariable "_atak_group_name";

	if (isNil "_group_name") then {
		_group_colors = missionNamespace getVariable "_group_colors";
		_group_name = selectRandom _group_colors;

		if (count _group_colors > 0) then {
			_randomIndex = floor (random (count _group_colors));

			_selectedColor = _group_colors select _randomIndex;

			_group_colors deleteAt _randomIndex;

			_group_name = _selectedColor;
			_group setVariable ["_atak_group_name", _group_name];
			missionNamespace setVariable ["_group_colors", _group_colors]
		} else {
			_group_name = "Red";
			_group setVariable ["_atak_group_name", _group_name];
		};
	};

	if (["SpecialOperative", (configFile >> "CfgVehicles" >> typeOf _x >> "role") call BIS_fnc_getCfgData, false] call BIS_fnc_inString) then {
		_group_role = _group_roles select 5;
	};

	if (_x getUnitTrait "medic") then {
		_group_role = _group_roles select 4;
	};

	if ((["jtac", typeOf _x, false] call BIS_fnc_inString)) then {
		_group_role = _group_roles select 5;
	};

	if (((backpack _x) isKindOf "TFAR_Bag_Base") or (["radio", typeOf _x, false] call BIS_fnc_inString)) then {
		_group_role = _group_roles select 6;
	};

	if ((["sniper", typeOf _x, false] call BIS_fnc_inString) or (["marksman", (configFile >> "CfgVehicles" >> typeOf _x >> "role") call BIS_fnc_getCfgData, false] call BIS_fnc_inString) or (["sharpshooter", typeOf _x, false] call BIS_fnc_inString)) then {
		_group_role = _group_roles select 3;
	};

	if (leader _group == _x) then {
		_group_role = _group_roles select 1;
	};

	if (["officer", typeOf _x, false] call BIS_fnc_inString) then {
		_group_role = _group_roles select 2;
	};

	[_x, name _x, _group_name, _group_role] call armatak_fnc_send_human_cot;
} forEach (units _group);