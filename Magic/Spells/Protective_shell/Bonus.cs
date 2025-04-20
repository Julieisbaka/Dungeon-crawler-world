// level is spell level not player level
//IMPORTANT: Var values for modifiers are temporary until their code is added.
var level = 1;
var mult_duration = 1;
var mult_range = 1;
var add_duration = 0;
var add_range = 0;
int duration = mult_duration * (5 + level) + add_duration;
int radius = mult_range * (300 + (30 * level)) + add_range;