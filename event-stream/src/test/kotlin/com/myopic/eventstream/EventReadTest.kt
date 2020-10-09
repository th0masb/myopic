package com.myopic.eventstream

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.fail

class EventReadTest {
    @Test
    fun `Given a challenge string then the event is correctly deserialised`() {
        val challengeString = """
            {
                "type":"challenge",
                "challenge":{
                    "id":"k49OUpZB",
                    "url":"https://lichess.org/k49OUpZB",
                    "status":"created",
                    "challenger":{
                        "id":"maumay",
                        "name":"maumay",
                        "title":null,
                        "rating":1500,
                        "provisional":true,
                        "online":true
                    },
                    "destUser":{
                        "id":"myopic-bot",
                        "name":"myopic-bot",
                        "title":"BOT",
                        "rating":1500,
                        "provisional":true,
                        "online":true
                    },
                    "variant":{
                        "key":"standard",
                        "name":"Standard",
                        "short":"Std"
                    },
                    "rated":false,
                    "speed":"classical",
                    "timeControl":{
                        "type":"clock",
                        "limit":2400,
                        "increment":35,
                        "show":"40+35"
                    },
                    "color":"random",
                    "perf":{"icon":"+","name":"Classical"}
                }
            }
        """.trimIndent()

        LichessEvent.fromJson(challengeString)
            .onFailure { err -> fail { "Unexpected failure: ${err.message}" } }
            .onSuccess {
                when (it) {
                    !is LichessEvent.Challenge -> fail { "Expected challenge, received: $it" }
                    else -> {
                        assertEquals("k49OUpZB", it.challenge.id)
                        assertEquals("created", it.challenge.status)
                        assertEquals("standard", it.challenge.variant.key)
                        assertEquals("clock", it.challenge.timeControl.type)
                        assertEquals(2400, it.challenge.timeControl.limit)
                        assertEquals(35, it.challenge.timeControl.increment)
                    }
                }
            }
    }
}