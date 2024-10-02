package net.joelcourtney.simpleRPG

import kotlinx.serialization.Serializable
import net.joelcourtney.pantheon.server.System

@Serializable
data class StringCell(var s: String)

class SimpleRPG: System<StringCell> {
    override val name = "Simple RPG!"
    override val sheetSerializer = StringCell.serializer()

    override fun newCharacterSheet() = StringCell("")

    override fun newCharacterSource() = listOf(SayHello)
}