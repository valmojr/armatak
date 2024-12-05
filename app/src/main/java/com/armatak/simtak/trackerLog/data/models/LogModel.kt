package com.armatak.simtak.trackerLog.data.models

data class LogModel(
    val idLine: Int,
    val time: String,
    val body: String,
    val type: LogTypes
)

sealed interface LogTypes {
    data object Error : LogTypes
    data object Warning : LogTypes
    data object Normal : LogTypes
    data object NetworkOperation : LogTypes
}