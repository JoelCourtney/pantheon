package io

import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.dataformat.yaml.YAMLFactory
import com.fasterxml.jackson.module.kotlin.KotlinModule
import io.deserializers.*
import model.logic.Event
import model.logic.Timer
import model.quantities.Identifier
import model.quantities.amounts.Amount
import model.quantities.damage.Damage
import model.quantities.damage.DamageUnit
import model.quantities.distance.Distance
import model.quantities.distance.DistanceUnit
import model.quantities.time.Time
import model.quantities.time.TimeUnit
import model.results.Result
import java.nio.file.FileSystems
import java.nio.file.Files

class JacksonWrapper {
    companion object {
        val mapper = ObjectMapper(YAMLFactory())
        private val module = KotlinModule()

        init {
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
                Amount::class.java,
                AmountDeserializer()
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
            module.addDeserializer(
                Identifier::class.java,
                IdentifierDeserializer()
            )
            module.addDeserializer(
                Timer::class.java,
                TimerDeserializer()
            )
            mapper.registerModule(module)
        }

        inline fun<reified T: Any> readFile(dir: String, file: String): T {
            val path = FileSystems.getDefault().getPath(dir, file)
            return Files.newBufferedReader(path).use {
                mapper.readValue(it, T::class.java)
            }
        }

        inline fun<reified T: Any> readString(s: String): T {
            val reader = java.io.StringReader(s)
            return reader.use {
                mapper.readValue(it, T::class.java)
            }
        }

    }
}