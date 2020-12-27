package model.gameObjects

sealed class Proficiency {
    class ArmorProficiency(name: String): Proficiency()
    class WeaponProficiency(name: String): Proficiency()
    class ToolProficiency(name: String): Proficiency()
    class LanguageProficiency(name: String): Proficiency()
    class SkillProficiency(skill: Skill, type: ProficiencyType): Proficiency()
    class SavingThrowProficiency(savingThrow: SavingThrow)
}
