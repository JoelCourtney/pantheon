import IOWrapper.Companion.read
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.dataformat.yaml.YAMLFactory
import com.fasterxml.jackson.module.kotlin.KotlinModule
import model.*
import java.nio.file.FileSystems
import java.nio.file.Files
import java.nio.file.Path

fun main() {
    val spell = read<Spell>(".", "spell.yaml")
    println(spell)
}

