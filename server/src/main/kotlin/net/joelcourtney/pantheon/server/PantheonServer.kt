package net.joelcourtney.pantheon.server

import io.ktor.http.ContentType
import io.ktor.server.engine.embeddedServer
import io.ktor.server.netty.Netty
import io.ktor.server.response.respondText
import io.ktor.server.routing.get
import io.ktor.server.routing.routing
import kotlinx.serialization.json.Json

class PantheonServer<SHEET: Any, S: System<SHEET>>(private val system: S) {
    fun start() {
        embeddedServer(Netty, 8080) {
            routing {
                get("/") {
                    call.respondText("Hello, world!", ContentType.Text.Html)
                }
            }
        }.start(wait = true)
    }

    fun doThing(s: SHEET) {
        println(Json.encodeToString(system.sheetSerializer, s))
    }
}

