package model.quantities.damage

import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.DamageDeserializer
import model.quantities.Quantity

@JsonDeserialize(using = DamageDeserializer::class)
interface Damage : Quantity
