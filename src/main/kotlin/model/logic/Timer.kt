package model.logic

import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.TimerDeserializer
import model.quantities.time.Time

@JsonDeserialize(using = TimerDeserializer::class)
data class Timer(val time: Time)
