// function name: armatak_fnc_extract_side
// function author: Valmo
// function description: Gets the unit side

params["_unit"];

_side = "friendly";
switch (side _unit) do {
	case "WEST": {
		_side = "friendly";
	};
	case "EAST": {
		_side = "hostile";
	};
	case "INDEPENDENT": {
		_side = "neutral";
	};
	case "CIVILIAN": {
		_side = "unknown";
	};
	default {
		_side = "friendly";
	};
};
_side