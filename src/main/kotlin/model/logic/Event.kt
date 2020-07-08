package model.logic

import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.EventDeserializer
import model.results.TimedResult
import model.results.effects.SpawnEventEffect

/**
 * In-game event used to process [TimedResult]s.
 * 
 * Spawned by either hard-coded logic or by a [SpawnEventEffect]. Triggers listening for this event will be notified when it is spawned. Note that there are a list of common events used by the base rules, but homebrew content can both spawn and listen for arbitrary custom events.
 * 
 * ## List of Common Events
 * 
 * For conciseness, events of the form
 * 
 * - ( thing 1 | thing 2 ) other thing
 * 
 * mean that both "thing 1 other thing" and "thing 2 other thing" are commonly generated events.
 * 
 * Events of the form:
 * 
 * - do *** thing
 *     - some
 *     - no
 * 
 * mean that both "do some thing" and "do no thing" are commonly generated events.
 * 
 * 
 * ### Time-Based:
 * 
 * - ( start | end ) of turn
 * - ( start | end ) of action
 * - ( start | end ) of bonus action
 * - ( start | end ) of reaction
 * - ( start | end ) of ( short | long ) rest
 * - dawn
 *
 * ### Health-Based
 * 
 * Note that when taking (or dealing) damage, two or three events will be triggered: "take damage", "take <type> damage", and "take ( melee | ranged ) damage". *You only need to listen for the most specific event.*
 * 
 * - ( take | deal ) damage
 * - ( take | deal ) ( melee | ranged ) damage
 * - ( take | deal ) *** damage 
 *     - acid
 *     - bludgeoning
 *     - cold
 *     - fire
 *     - force
 *     - lightning
 *     - necrotic
 *     - piercing
 *     - poison
 *     - psychic
 *     - radiant
 *     - slashing
 *     - thunder
 * - be healed
 * - heal ( self | other )
 * 
 * ### Condition-Based
 * 
 * - ( become | no longer ) ***
 *     - blinded
 *     - charmed
 *     - deafened
 *     - frightened
 *     - grappled
 *     - incapacitated
 *     - invisible
 *     - paralyzed
 *     - petrified
 *     - poisoned
 *     - prone
 *     - restrained
 *     - stunned
 *     - unconscious
 * 
 * ### There will be more
 * 
 * but I don't want to do that right now.
 * 
 * @property [name] String that represents the event.
 */
@JsonDeserialize(using = EventDeserializer::class)
data class Event(val name: String)
