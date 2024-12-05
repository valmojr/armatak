package com.armatak.simtak.trackerLog.data.models

sealed interface ConnectionStatus {
    data object Connected : ConnectionStatus
    data object Disconnected : ConnectionStatus
    data object OnReconnect : ConnectionStatus
    data object Awaiting : ConnectionStatus
    data object InitialValue : ConnectionStatus
}