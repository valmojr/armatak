if (!hasInterface) exitWith {};

_local_address = "armatak" callExtension ["local_ip", []] select 0;

player setVariable ["localAddress", _local_address];

player addEventHandler ["Respawn", {
	params["_unit", "_corpse"];
	[{
		if (alive _this) then {
			"armatak" callExtension ["websocket:location", [player call armatak_fnc_extract_position]];
		};
	}, 1, []] call CBA_fnc_addPerFrameHandler;
}];

[{
	"armatak" callExtension ["websocket:location", [player call armatak_fnc_extract_position]];
}, 1, []] call CBA_fnc_addPerFrameHandler;
