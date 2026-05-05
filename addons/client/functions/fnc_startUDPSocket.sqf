#include "..\script_component.hpp"

params ["_logic"];

private _socket_is_running = player getVariable [QGVAR(eudConnected), false];

if (_socket_is_running) exitWith {
	["Socket is already running", "error", "UDP Socket"] call EFUNC(main,notify);
	closeDialog 1;
};

disableSerialization;

private _eud_address = ctrlText 16961;
private _gnss_port = ctrlText 16962;
private _mavlink_port = ctrlText 16967;

private _udp_socket_fulladdress = _eud_address + ":" + _gnss_port;
private _mavlink_address = _eud_address + ":" + _mavlink_port;

player setVariable [QGVAR(udp_socket_address), _udp_socket_fulladdress];
player setVariable [QGVAR(mavlink_address), _mavlink_address];
player setVariable [QGVAR(eudConnected), true];

"armatak" callExtension ["udp_socket:start", [_udp_socket_fulladdress]];

private _mdnsInstanceName = format ["ArmaTAK-%1", name player];
"armatak" callExtension ["mdns:start_uas_advertisement", [_mdnsInstanceName, parseNumber _mavlink_port, "rtp://0.0.0.0:5600"]];
"armatak" callExtension ["log", [["info", format ["Client UDP socket started for %1 and MAVLink target set to %2", _udp_socket_fulladdress, _mavlink_address]]]];

call EFUNC(uav,startMavlinkBroadcast);

[{
	if !(player getVariable [QGVAR(eudConnected), false]) exitWith {};

	"armatak" callExtension ["udp_socket:send_gps_cot", [player call FUNC(extractClientPosition)]];
}, 0.5, []] call CBA_fnc_addPerFrameHandler;

deleteVehicle _logic;
closeDialog 1;
