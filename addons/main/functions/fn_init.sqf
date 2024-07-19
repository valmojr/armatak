// 1. Create FreeTAKServer instance
// ....
if (!isServer) exitWith {};

private _armatak_fts_server_url = "armatak" callExtension ["init",[]];

if (!_armatak_fts_server_url) exitWith {};

missionNamespace setVariable ["armatak_fts_server_url", _armatak_fts_server_url];
