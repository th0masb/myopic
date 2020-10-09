package com.myopic.eventstream

import com.fasterxml.jackson.annotation.JsonTypeInfo
import com.fasterxml.jackson.annotation.JsonTypeName
import com.fasterxml.jackson.databind.DeserializationFeature
import com.fasterxml.jackson.module.kotlin.jacksonObjectMapper
import com.fasterxml.jackson.module.kotlin.jacksonTypeRef

@JsonTypeInfo(use = JsonTypeInfo.Id.NAME, property = "type")
sealed class LichessEvent {
    companion object {
        private val reader = jacksonObjectMapper()
            .disable(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES)
            .readerFor(jacksonTypeRef<LichessEvent>())

        fun fromJson(json: String) = runCatching { reader.readValue<LichessEvent>(json) }
    }

    @JsonTypeName("gameStart")
    data class GameStart(
        val game: Game,
    ) : LichessEvent() {
        data class Game(
            val id: String,
        )
    }

    @JsonTypeName("challenge")
    data class Challenge(
        val challenge: Challenge,
    ) : LichessEvent() {
        data class Challenge(
            val id: String,
            val status: String,
            val variant: Variant,
            val timeControl: TimeControl,
        )

        data class Variant(
            val key: String,
        )

        data class TimeControl(
            val type: String,
            val limit: Int,
            val increment: Int,
        )
    }
}


