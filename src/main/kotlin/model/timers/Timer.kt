package model.timers

import model.quantities.Time

sealed class Timer {
    class Countdown(val t: Time) : Timer()
    object StartOfTurn : Timer()
    object EndOfTurn : Timer()
}