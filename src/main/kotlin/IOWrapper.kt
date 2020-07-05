import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.dataformat.yaml.YAMLFactory
import com.fasterxml.jackson.module.kotlin.KotlinModule
import model.spells.SpellSchool
import model.spells.SpellSchoolDeserializer
import quantities.*
import java.nio.file.FileSystems
import java.nio.file.Files

class IOWrapper {
    companion object {
        val mapper = ObjectMapper(YAMLFactory())

        init {
            val module = KotlinModule()
            module.addDeserializer(
                SpellSchool::class.java,
                SpellSchoolDeserializer()
            )
            module.addDeserializer(
                Time::class.java,
                TimeDeserializer()
            )
            module.addDeserializer(
                Damage::class.java,
                DamageDeserializer()
            )
            module.addDeserializer(
                Distance::class.java,
                DistanceDeserializer()
            )
            mapper.registerModule(module)
        }

        inline fun<reified T: Any> read(dir: String, file: String): T {
            val path = FileSystems.getDefault().getPath(dir, file)
            return Files.newBufferedReader(path).use {
                mapper.readValue(it, T::class.java)
            }
        }
    }
}