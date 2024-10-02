package net.joelcourtney.pantheon.server

import kotlinx.serialization.SerializationStrategy
import net.joelcourtney.pantheon.engine.Plugin

interface System<SHEET: Any> {
    val name: String

    fun newCharacterSheet(): SHEET
    fun newCharacterSource(): List<Plugin<SHEET>>

    val sheetSerializer: SerializationStrategy<SHEET>
}
