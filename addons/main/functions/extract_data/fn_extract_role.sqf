params["_unit"];

private _affiliation = "f";
private _type = "G";

switch (side _unit) do {
	case "WEST": { _affiliation = "f"; };
	case "EAST": { _affiliation = "h"; };
	case "INDEPENDENT": { _affiliation = "n"; };
	case "CIVILIAN": { _affiliation = "u"; };
	default { };
};

if (vehicle _unit isKindOf "plane") then {
	_type = "A-M-F";
};

if (vehicle _unit isKindOf "Helicopter") then {
	_type = "A-M-H";
};

if (vehicle _unit isKindOf "tank") then {
	_type = "G-U-C-A-T";
};

_role = "a-" + _affiliation + "-" + _type;

_role