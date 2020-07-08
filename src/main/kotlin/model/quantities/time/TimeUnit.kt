package model.quantities.time

import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.TimeUnitDeserializer

@JsonDeserialize(using = TimeUnitDeserializer::class)
interface TimeUnit {
    fun getTimeUnitLiteral() : TimeUnitLiteral
}