params["_unit"];
private _role = "Gnd Combat Infantry Rifleman";

if (vehicle _unit != _unit) then {
	if (vehicle _unit isKindOf "StaticWeapon") then {
		_role = "Gnd Combat Infantry Mortar";
	};
};

if ([_unit] call ace_common_fnc_isMedic) then {
	_role = "Gnd Combat Infantry Medic";
};

if ([_unit] call ace_common_fnc_isEngineer || [_unit] call ace_common_fnc_isEOD) then {
	_role = "Gnd Combat Infantry Engineer";
};

_role