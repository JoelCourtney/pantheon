package net.joelcourtney.simpleRPG

import net.joelcourtney.pantheon.server.PantheonServer

fun main() {
    val system = SimpleRPG()

    PantheonServer(system).start()
}
