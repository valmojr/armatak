params["_unit"];

private _affiliation = "f";
private _type = "G";

switch (side _unit) do {
	case "WEST": {
		_affiliation = "f";
	};
	case "EAST": {
		_affiliation = "h";
	};
	case "INDEPENDENT": {
		_affiliation = "n";
	};
	case "CIVILIAN": {
		_affiliation = "u";
	};
	default {
		_affiliation = "f";
	};
};

_unit_type = _unit call BIS_fnc_objectType select 1;

switch (_unit_type) do {
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

if (!isNull vehicle _unit) then {
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
		default {};
	};
};

_role = "a-" + _affiliation + "-" + _type;

_role