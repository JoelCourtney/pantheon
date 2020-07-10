package io

import model.logic.Trigger
import model.logic.Event
import model.logic.LogicType
import model.logic.Timer
import model.quantities.Quantity
import model.quantities.QuantityComponent
import model.quantities.amounts.Dice
import model.quantities.amounts.NumberLiteral
import model.quantities.TimeKeyword
import model.quantities.TimeUnit
import org.junit.jupiter.api.Assertions.assertArrayEquals
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.EnumSource
import model.quantities.QuantityType.Time

class TestLogicIO {

    @Test
    fun parseEvent() {
        assertEquals(
            Event("ran a test"),
            JacksonWrapper.readString<Event>("ran a test")
        )
    }

    @Test
    fun parseEventList() {
        assertArrayEquals(
            arrayOf(
                Event("event 1"),
                Event("event 2"),
                Event("event 3")
            ),
            JacksonWrapper.readString<Array<Event>>(
                """
                    - event 1
                    - event 2
                    - event 3
                """.trimMargin()
            )
        )
    }

    @ParameterizedTest(name = "parse logic type {0}")
    @EnumSource(LogicType::class)
    fun parseLogicType(lt: LogicType) {
        assertEquals(
            lt,
            JacksonWrapper.readString<LogicType>(lt.identity)
        )
    }

    @Test
    fun parseTimer() {
        assertEquals(
            Timer(
                Quantity<Time>(
                    arrayListOf(
                        QuantityComponent(
                            NumberLiteral(2),
                            TimeUnit.DAY
                        )
                    )
                )
            ),
            JacksonWrapper.readString<Timer>("2 days")
        )
        assertEquals(
            Timer(
                Quantity<Time>(
                    arrayListOf(
                        QuantityComponent(
                            Dice(3, 6),
                            TimeUnit.ACTION
                        )
                    )
                )
            ),
            JacksonWrapper.readString<Timer>("3d6 actions")
        )
    }

    @Test
    fun parseTimerList() {
        assertArrayEquals(
            arrayOf(
                Timer(Quantity<Time>(arrayListOf(TimeKeyword.NOW))),
                Timer(Quantity<Time>(arrayListOf(QuantityComponent(NumberLiteral(2), TimeUnit.MINUTE)))),
                Timer(Quantity<Time>(arrayListOf(QuantityComponent(Dice(4,6), TimeUnit.DAY))))
            ),
            JacksonWrapper.readString<Array<Timer>>(
                """
                    - now
                    - 2 minutes
                    - 4d6 days
                """.trimIndent()
            )
        )
    }

    @Test
    fun parseTrigger() {
        assertEquals(
            Trigger(
                arrayOf(
                    Event("take damage")
                ),
                arrayOf(
                    Timer(Quantity<Time>(arrayListOf(TimeKeyword.INDEFINITE)))
                ),
                LogicType.CONSECUTIVE
            ),
            JacksonWrapper.readString<Trigger>(
                """
                    events:
                      - take damage
                    timers:
                      - infinite
                    type: consecutive
                """.trimIndent()
            )
        )
    }
}
