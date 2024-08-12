// function name: armatak_fnc_send_message
// function author: Valmo
// function description: Receives a message string and send it to the FTS chat 

params["_server_message"];

private _atak_server_instance = missionNamespace getVariable "_atak_server_instance";
private _atak_server_instance_token = missionNamespace getVariable "_atak_server_instance_token";

private _message_payload = [_server_message, _atak_server_instance, _atak_server_instance_token];

private _request = "armatak" callExtension ["ManageChat/postChatToAll",_message_payload];