package com.myopic.eventstream

import mu.KotlinLogging
import java.io.InputStreamReader
import java.net.URL
import javax.net.ssl.HttpsURLConnection

private val logger = KotlinLogging.logger {  }

fun main() {
    while (true) {
        try {
            InputStreamReader(createConnection().inputStream).useLines { lines ->
                lines.filter { it.isNotBlank() }.forEach { event ->
                    LichessEvent.fromJson(event).fold(
                        onSuccess = {
                            when (it) {
                                is LichessEvent.GameStart -> {
                                    logger.info { "Received game start trigger with id: ${it.game.id}" }
                                    // Trigger game lambda
                                    // If fails then call the abort game endpoint
                                }
                                is LichessEvent.Challenge -> {
                                    logger.info { "Received challenge with id: ${it.challenge.id}" }
                                    // Check challenge paramters
                                    // If ok then call the accept challenge endpoint
                                }
                            }
                        },

                        onFailure = {
                            logger.warn { "Received unexpected event: $event" }
                        }
                    )
                }
            }
        } catch (t: Throwable) {
            logger.error { t.message }
            Thread.sleep(1000)
        }
    }
}

private fun createConnection() = System.getenv("STREAM_ADDRESS").let { address ->
    (URL(address).openConnection() as HttpsURLConnection).apply {
        addRequestProperty("Authorization", "Bearer ${System.getenv("AUTH_KEY")}")
    }.also {
        logger.info { "Opening new connection to $address" }
    }
}
