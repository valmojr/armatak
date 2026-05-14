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
private _lrf_port = ctrlText 16971;
private _video_feed_url = ctrlText 16969;

private _udp_socket_fulladdress = _eud_address + ":" + _gnss_port;
private _mavlink_address = _eud_address + ":" + _mavlink_port;
private _lrf_port_trimmed = trim _lrf_port;
private _lrf_enabled = _lrf_port_trimmed isNotEqualTo "";
private _lrf_address = _eud_address + ":" + _lrf_port_trimmed;

player setVariable [QGVAR(udp_socket_address), _udp_socket_fulladdress];
player setVariable [QGVAR(mavlink_address), _mavlink_address];
player setVariable [QGVAR(lrf_address), _lrf_address];
player setVariable [QGVAR(lrfEnabled), _lrf_enabled];
player setVariable [QGVAR(lrfWasActive), false];
player setVariable [QGVAR(lrfLostAt), -1];
player setVariable [QGVAR(lrfClearSent), false];
player setVariable [QGVAR(lrfLastTargetASL), []];
player setVariable [QGVAR(lrfLastSentAt), -1000];
player setVariable [QGVAR(video_feed_url), trim _video_feed_url];
player setVariable [QGVAR(eudConnected), true, true];

private _advertised_video_uri = [objNull] call EFUNC(uav,resolveVideoUri);

"armatak" callExtension ["udp_socket:start", [_udp_socket_fulladdress]];
"armatak" callExtension ["uas:start_endpoint", [parseNumber _mavlink_port]];
if (_lrf_enabled) then {
	"armatak" callExtension ["udp_socket:start_lrf", [_lrf_address]];
};

private _mdnsInstanceName = format ["ArmaTAK-%1", name player];
"armatak" callExtension ["mdns:start_uas_advertisement", [_mdnsInstanceName, parseNumber _mavlink_port, _advertised_video_uri]];
"armatak" callExtension ["log", [["info", format ["Client UDP socket started for %1, MAVLink target set to %2, LRF target set to %3 and advertised video URI set to %4. Digital pointer uses ATAK LRF when enabled.", _udp_socket_fulladdress, _mavlink_address, _lrf_address, _advertised_video_uri]]]];

call EFUNC(uav,startMavlinkBroadcast);

[{
	if !(player getVariable [QGVAR(eudConnected), false]) exitWith {};

	"armatak" callExtension ["udp_socket:send_gps_cot", [player call FUNC(extractClientPosition)]];
}, 0.5, []] call CBA_fnc_addPerFrameHandler;

[{
	if !(player getVariable [QGVAR(eudConnected), false]) exitWith {};

	[player] call FUNC(sendLaserRangeFinder);
}, 0.25, []] call CBA_fnc_addPerFrameHandler;

deleteVehicle _logic;
closeDialog 1;
