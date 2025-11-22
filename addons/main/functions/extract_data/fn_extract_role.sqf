// function name: armatak_fnc_extract_role
// function author: Valmo
// function description: Gets the unit function to be used as TAK role or NATO 2525 marker

params["_unit"];

private _affiliation = "f";
private _type = "G";
private _role = "a-f-G-U-C-I";
private _side = side _unit;

if (isNil {_unit getVariable "armatak_current_side"}) then {
	_side = _unit getVariable "armatak_current_side";	
};

switch (str _side) do {
	case "WEST": {
		_affiliation = "f";
	};
	case "EAST": {
		_affiliation = "h";
	};
	case "GUER": {
		_affiliation = "n";
	};
	case "CIVILIAN": {
		_affiliation = "u";
	};
	default {
		_affiliation = "f";
	};
};

_unit_type = _unit call BIS_fnc_objectType;

if ((_unit_type select 0) == "Soldier") then {
	switch (_unit_type select 1) do {
		case "AT": {
			_type = "G-U-C-F-R";
		};
		case "Civilian": {
			_type = "G-E-V-C";
		};
		case "Diver": {
			_type = "U-S";
		};
		case "Infantry": {
			_type = "G-U-C-I";
		};
		case "Medic": {
			_type = "a-f-G-U-C";
		};
		case "MG": {
			_type = "G-U-C-I";
		};
		case "Officer": {
			_type = "G-U-C-I";
		};
		case "Pilot": {
			_type = "G-U-C-V";
		};
		case "Sniper": {
			_type = "G-U-C-R-X";
		};
		case "SpecialForces": {
			_type = "G-U-C-R-X";
		};
		case "UAVPilot": {
			_type = "G-U-C-V-U";
		};
		default {
			_type = "G-U-C-I";
		};
	};
};

if ((typeOf (vehicle _unit) != typeOf _unit) or ((_unit_type select 0) == "Vehicle")) then {
	_vehicle_type = (vehicle _unit) call BIS_fnc_objectType select 1;
	switch (_vehicle_type) do {
		case "Car": {
			_type = "G-U-C-I-M";
		};
		case "Helicopter": {
			_type = "A-M-H";
		};
		case "Motorcycle": {
			_type = "G-U-C-I-M";
		};
		case "Plane": {
			_type = "A-M-F";
		};
		case "Ship": {
			_type = "S";
		};
		case "StaticWeapon": {
			_type = "G-U-C-F-M";
		};
		case "Submarine": {
			_type = "U-S";
		};
		case "TrackedAPC": {
			_type = "G-U-C-I-I";
		};
		case "Tank": {
			_type = "G-U-C-A-T";
		};
		case "WheeledAPC": {
			_type = "G-U-C-I-Z";
		};
		default {
			_type = "G-U-C-I";
		};
	};
};

_role = "a-" + _affiliation + "-" + _type;


armatak_attribute_marker_type = _unit getVariable "armatak_attribute_marker_type";

if (!isNil "armatak_attribute_marker_type" or armatak_attribute_marker_type != '') then {
	_role = armatak_attribute_marker_type;
};

_role
