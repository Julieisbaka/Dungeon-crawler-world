// level is spell level not player level
// Var values for modifiers are temporary until their code is added.
int level = 1;
int mult_duration = 1;
int mult_range = 1;
int add_duration = 0;
int add_range = 0;
int duration = mult_duration * (5 + level) + add_duration;
int radius = mult_range * (300 + (30 * level)) + add_range;
