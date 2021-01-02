package model.gameObjects

import model.access.Expression

sealed class Proficiency {
    class ArmorProficiency(name: Expression<String>): Proficiency()
    class WeaponProficiency(name: Expression<String>): Proficiency()
    class ToolProficiency(name: Expression<String>, multiplier: ProficiencyMultiplier): Proficiency()
    class LanguageProficiency(name: Expression<String>): Proficiency()
    class SkillProficiency(skill: Expression<Skill>, multiplier: ProficiencyMultiplier): Proficiency()
    class SavingThrowProficiency(savingThrow: Expression<SavingThrow>): Proficiency()
}
