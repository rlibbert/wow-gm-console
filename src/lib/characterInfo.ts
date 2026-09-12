// Static WotLK 3.3.5a playable race/class ID -> name lookup, used only to
// label picker rows. Fixed game constants, not fetched from the database.
// Spot-checked against live character data (e.g. race=8/class=4 correctly
// matches the known Troll Rogue "Bazzul").

export const RACE_NAMES: Record<number, string> = {
	1: 'Human',
	2: 'Orc',
	3: 'Dwarf',
	4: 'Night Elf',
	5: 'Undead',
	6: 'Tauren',
	7: 'Gnome',
	8: 'Troll',
	10: 'Blood Elf',
	11: 'Draenei'
};

export const CLASS_NAMES: Record<number, string> = {
	1: 'Warrior',
	2: 'Paladin',
	3: 'Hunter',
	4: 'Rogue',
	5: 'Priest',
	6: 'Death Knight',
	7: 'Shaman',
	8: 'Mage',
	9: 'Warlock',
	11: 'Druid'
};

// WoW's standard class color palette.
export const CLASS_COLORS: Record<number, string> = {
	1: '#c79c6e',
	2: '#f58cba',
	3: '#abd473',
	4: '#fff569',
	5: '#ffffff',
	6: '#c41f3b',
	7: '#0070de',
	8: '#69ccf0',
	9: '#9482c9',
	11: '#ff7d0a'
};

export function raceName(race: number): string {
	return RACE_NAMES[race] ?? `Race ${race}`;
}

export function className(cls: number): string {
	return CLASS_NAMES[cls] ?? `Class ${cls}`;
}

export function classColor(cls: number): string {
	return CLASS_COLORS[cls] ?? '#ffffff';
}
