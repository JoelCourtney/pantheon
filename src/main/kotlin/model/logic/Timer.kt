package model.logic

import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.TimerDeserializer
import model.identity.Expression
import model.quantities.Quantity
import model.quantities.QuantityType

@JsonDeserialize(using = TimerDeserializer::class)
inline class Timer(val time: Expression<Quantity<QuantityType.Time>>)
