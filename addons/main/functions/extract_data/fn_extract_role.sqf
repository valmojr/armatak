params["_unit"];

private _affiliation = "f";
private _type = "G";

if (vehicle _unit isKindOf "plane") then {
	_type = "A";
};

_role = "a-" + _affiliation + "-" + _type;

_role