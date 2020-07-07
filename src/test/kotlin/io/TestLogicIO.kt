package io

import model.logic.Event
import model.logic.LogicType
import model.logic.Question
import model.logic.Timer
import model.quantities.Identifier
import model.quantities.amounts.NumberLiteral
import model.quantities.time.TimeComponent
import model.quantities.time.TimeUnitLiteral
import org.junit.jupiter.api.Assertions.assertArrayEquals
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.EnumSource

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
            JacksonWrapper.readString<LogicType>(lt.symbol)
        )
    }

    @Test
    fun parseQuestion() {
        assertEquals(
            Question(Identifier("hello"), "can you hello"),
            JacksonWrapper.readString<Question>(
                """
                    key: hello
                    ask: can you hello
                """.trimIndent()
            )
        )
    }

    @Test
    fun parseQuestionList() {
        assertArrayEquals(
            arrayOf(
                Question(Identifier("hello", "world"), "compile me papa"),
                Question(Identifier("i", "dont"), "believe i just wrote that")
            ),
            JacksonWrapper.readString<Array<Question>>(
                """
                    - key: hello${'$'}world
                      ask: compile me papa
                    - key: i${'$'}dont
                      ask: believe i just wrote that
                """.trimIndent()
            )
        )
    }

    @Test
    fun parseTimer() {
        assertEquals(
            Timer(TimeComponent(NumberLiteral(2), TimeUnitLiteral.DAY)),
            JacksonWrapper.readString<Timer>("2 days")
        )
    }
}