if (!hasInterface) exitWith {};

_initializedServer = "armatak" callExtension ["start",[]] select 0;
_local_address = "armatak" callExtension ["local_ip", []] select 0;

player setVariable ["initializedSocket", _initializedServer];
player setVariable ["localAddress", _local_address];

player addEventHandler ["Killed", {
	"armatak" callExtension ["stop", []];
}];

player addEventHandler ["Deleted", {
	"armatak" callExtension ["stop", []];
}];

player addEventHandler ["Respawn", {
	params["_unit", "_corpse"];

	_unit spawn {
		"armatak" callExtension ["start", []];

		[{ 
			if (alive _this) then {
				"armatak" callExtension ["location",[player call armatak_fnc_extract_info]];	
			};
		}, 1, []] call CBA_fnc_addPerFrameHandler;
	};
}];

onPlayerDisconnected "'armatak' callExtension ['stop',[]];";

[{ 
	"armatak" callExtension ["location",[player call armatak_fnc_extract_info]];	
}, 1, []] call CBA_fnc_addPerFrameHandler;