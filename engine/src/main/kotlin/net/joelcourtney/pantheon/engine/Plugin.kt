package net.joelcourtney.pantheon.engine

import kotlinx.serialization.Serializable

@Serializable
abstract class Plugin<SHEET: Any>(val id: SUUID) {
    constructor(): this(SUUID.randomUUID())

    abstract fun SHEET.apply()
}
