package net.joelcourtney.pantheon.server

import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import net.joelcourtney.pantheon.engine.Plugin
import net.joelcourtney.pantheon.engine.Staged

class PantheonServer<SHEET: Any, S: System<SHEET>>(private val system: S) {
    fun doThing(s: SHEET) {
        println(Json.encodeToString(system.sheetSerializer, s))
    }
}

@Serializable
class MySheet {
    val name = Staged<String>()
}

class MySystem: System<MySheet> {
    override fun newCharacterSheet() = MySheet()

    override fun newCharacterSource() = listOf<Plugin<MySheet>>()

    override val sheetSerializer = MySheet.serializer()
}
