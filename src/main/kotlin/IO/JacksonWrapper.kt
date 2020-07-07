package IO

import IO.deserializers.*
import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.dataformat.yaml.YAMLFactory
import com.fasterxml.jackson.module.kotlin.KotlinModule
import model.SavingThrowType
import model.SavingThrowTypeDeserializer
import model.logic.Event
import model.results.Result
//import model.spells.SpellSchoolDeserializer
import model.quantities.*
import model.quantities.damage.Damage
import model.quantities.damage.DamageUnit
import model.quantities.distance.Distance
import model.quantities.distance.DistanceUnit
import model.quantities.time.Time
import model.quantities.time.TimeUnit
import java.nio.file.FileSystems
import java.nio.file.Files

class JacksonWrapper {
    companion object {
        val mapper = ObjectMapper(YAMLFactory())
        val module = KotlinModule()

        init {
//            module.addDeserializer(
//                SpellSchool::class.java,
//                SpellSchoolDeserializer()
//            )
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
            module.addDeserializer(
                Result::class.java,
                ResultDeserializer()
            )
            module.addDeserializer(
                SavingThrowType::class.java,
                SavingThrowTypeDeserializer()
            )
            module.addDeserializer(
                Expression::class.java,
                ExpressionDeserializer()
            )
            module.addDeserializer(
                DamageUnit::class.java,
                DamageUnitDeserializer()
            )
            module.addDeserializer(
                TimeUnit::class.java,
                TimeUnitDeserializer()
            )
            module.addDeserializer(
                DistanceUnit::class.java,
                DistanceUnitDeserializer()
            )
            module.addDeserializer(
                Event::class.java,
                EventDeserializer()
            )
            mapper.registerModule(module)
        }

        inline fun<reified T: Any> read(dir: String, file: String): T {
            val path = FileSystems.getDefault().getPath(dir, file)
            return Files.newBufferedReader(path).use {
                mapper.readValue(it, T::class.java)
            }
        }

        fun enumString(p: JsonParser?): String {
            return p!!.readValueAs(String::class.java).toUpperCase()
        }
    }
}