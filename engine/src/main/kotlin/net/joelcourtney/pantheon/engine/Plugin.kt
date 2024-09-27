package net.joelcourtney.pantheon.engine

import kotlinx.serialization.Serializable

@Serializable
abstract class Plugin<C: Any>(val id: SUUID) {
    constructor(): this(SUUID.randomUUID())

    abstract fun C.apply()
}
