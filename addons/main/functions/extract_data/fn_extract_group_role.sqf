params["_unit"];

_group_roles = ["Team Member", "Team Lead", "HQ", "Sniper", "Medic", "Forward Observer", "RTO", "K9"];
_group_role = "Team Member";

if (["SpecialOperative", (configFile >> "CfgVehicles" >> typeOf _unit >> "role") call BIS_fnc_getCfgData, false] call BIS_fnc_inString) then {
	_group_role = _group_roles select 5;
};

if (_unit getUnitTrait "medic") then {
	_group_role = _group_roles select 4;
};

if ((["jtac", typeOf _unit, false] call BIS_fnc_inString)) then {
	_group_role = _group_roles select 5;
};

if (((backpack _unit) isKindOf "TFAR_Bag_Base") or (["radio", typeOf _unit, false] call BIS_fnc_inString)) then {
	_group_role = _group_roles select 6;
};

if ((["sniper", typeOf _unit, false] call BIS_fnc_inString) or (["marksman", (configOf _unit >> "role") call BIS_fnc_getCfgData, false] call BIS_fnc_inString) or (["sharpshooter", typeOf _unit, false] call BIS_fnc_inString)) then {
	_group_role = _group_roles select 3;
};

if (leader _group == _unit) then {
	_group_role = _group_roles select 1;
};

if (["officer", typeOf _unit, false] call BIS_fnc_inString) then {
	_group_role = _group_roles select 2;
};

_pre_defined_role = _unit getVariable "_atak_group_role";

if (!isNil "_pre_defined_role") then {
	_callsign = _pre_defined_callsign;
};

_group_role
