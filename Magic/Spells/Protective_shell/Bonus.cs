// level is spell level not player level
// Var values for modifiers are temporary until their code is added.
// The values assigned currently just make the spell do normal damage.
var level = 1;
var mult_duration = 1;
var mult_range = 1;
var add_duration = 0;
var add_range = 0;
var duration = mult_duration * (5 + level) + add_duration;
var radius = mult_range * (300 + (30 * level)) + add_range;
