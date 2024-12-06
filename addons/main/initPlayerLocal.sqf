if (!hasInterface) exitWith {};

_initializedServer = "armatak" callExtension ["websocket:start",[]] select 0;
_local_address = "armatak" callExtension ["local_ip", []] select 0;

player setVariable ["initializedSocket", _initializedServer];
player setVariable ["localAddress", _local_address];

player addEventHandler ["Killed", {
	"armatak" callExtension ["websocket:stop", []];
}];

player addEventHandler ["Deleted", {
	"armatak" callExtension ["websocket:stop", []];
}];

player addEventHandler ["Respawn", {
	params["_unit", "_corpse"];

	_unit spawn {
		"armatak" callExtension ["websocket:start", []];

		[{ 
			if (alive _this) then {
				"armatak" callExtension ["websocket:location",[player call armatak_fnc_extract_info]];	
			};
		}, 1, []] call CBA_fnc_addPerFrameHandler;
	};
}];

addMissionEventHandler ["OnUserDisconnected", {
	"armatak" callExtension ["websocket:stop", []];
}];

onPlayerDisconnected "'armatak' callExtension ['websocket:stop',[]];";

[{ 
	"armatak" callExtension ["websocket:location",[player call armatak_fnc_extract_info]];	
}, 1, []] call CBA_fnc_addPerFrameHandler;