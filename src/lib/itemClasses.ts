// Static WotLK 3.3.5a item class/subclass ID -> name lookup, used only to
// build filter dropdowns and label picker rows. These are fixed game
// constants unchanged since the original WotLK release, not fetched from
// the database or any external source -- the backend never needs to
// translate an ID back to a name, only forward the numeric IDs a <select>
// already carries.
//
// Spot-check during use against live data if anything looks mislabeled:
//   SELECT DISTINCT class, subclass, name FROM item_template WHERE class=2 LIMIT 20;

export interface ItemClassInfo {
	name: string;
	subclasses: Record<number, string>;
}

export const ITEM_CLASSES: Record<number, ItemClassInfo> = {
	0: {
		name: 'Consumable',
		subclasses: { 0: 'Consumable', 1: 'Potion', 2: 'Elixir', 3: 'Flask', 4: 'Scroll', 5: 'Food & Drink', 6: 'Item Enhancement', 7: 'Bandage', 8: 'Other' }
	},
	1: {
		name: 'Container',
		subclasses: { 0: 'Bag', 1: 'Soul Bag', 2: 'Herb Bag', 3: 'Enchanting Bag', 4: 'Engineering Bag', 5: 'Gem Bag', 6: 'Mining Bag', 7: 'Leatherworking Bag', 8: 'Inscription Bag' }
	},
	2: {
		name: 'Weapon',
		subclasses: {
			0: 'One-Handed Axe',
			1: 'Two-Handed Axe',
			2: 'Bow',
			3: 'Gun',
			4: 'One-Handed Mace',
			5: 'Two-Handed Mace',
			6: 'Polearm',
			7: 'One-Handed Sword',
			8: 'Two-Handed Sword',
			9: 'Obsolete',
			10: 'Staff',
			11: 'Exotic (One-Handed)',
			12: 'Exotic (Two-Handed)',
			13: 'Fist Weapon',
			14: 'Miscellaneous',
			15: 'Dagger',
			16: 'Thrown',
			17: 'Spear',
			18: 'Crossbow',
			19: 'Wand',
			20: 'Fishing Pole'
		}
	},
	3: { name: 'Gem', subclasses: { 0: 'Red', 1: 'Blue', 2: 'Yellow', 3: 'Purple', 4: 'Green', 5: 'Orange', 6: 'Meta', 7: 'Simple', 8: 'Prismatic' } },
	4: {
		name: 'Armor',
		subclasses: { 0: 'Miscellaneous', 1: 'Cloth', 2: 'Leather', 3: 'Mail', 4: 'Plate', 5: 'Buckler', 6: 'Shield', 7: 'Libram', 8: 'Idol', 9: 'Totem', 10: 'Sigil' }
	},
	5: { name: 'Reagent', subclasses: { 0: 'Reagent' } },
	6: { name: 'Projectile', subclasses: { 2: 'Arrow', 3: 'Bullet' } },
	7: {
		name: 'Trade Goods',
		subclasses: {
			0: 'Trade Goods',
			1: 'Parts',
			2: 'Explosives',
			3: 'Devices',
			4: 'Jewelcrafting',
			5: 'Cloth',
			6: 'Leather',
			7: 'Metal & Stone',
			8: 'Meat',
			9: 'Herb',
			10: 'Elemental',
			11: 'Other',
			12: 'Enchanting',
			13: 'Materials (Inscription)'
		}
	},
	9: { name: 'Recipe', subclasses: { 0: 'Book', 1: 'Leatherworking', 2: 'Tailoring', 3: 'Engineering', 4: 'Blacksmithing', 5: 'Cooking', 6: 'Alchemy', 7: 'First Aid', 8: 'Enchanting', 9: 'Fishing', 10: 'Jewelcrafting', 11: 'Inscription' } },
	11: { name: 'Quiver', subclasses: { 0: 'Quiver', 1: 'Quiver', 2: 'Ammo Pouch', 3: 'Ammo Pouch' } },
	12: { name: 'Quest', subclasses: { 0: 'Quest' } },
	13: { name: 'Key', subclasses: { 0: 'Key', 1: 'Lockpick' } },
	15: { name: 'Miscellaneous', subclasses: { 0: 'Junk', 1: 'Reagent', 2: 'Pet', 3: 'Holiday', 4: 'Other', 5: 'Mount' } },
	16: { name: 'Glyph', subclasses: { 1: 'Warrior', 2: 'Paladin', 3: 'Hunter', 4: 'Rogue', 5: 'Priest', 6: 'Death Knight', 7: 'Shaman', 8: 'Mage', 9: 'Warlock', 11: 'Druid' } }
};

export const ITEM_QUALITIES = [
	'Poor',
	'Common',
	'Uncommon',
	'Rare',
	'Epic',
	'Legendary',
	'Artifact',
	'Heirloom'
];

// WoW's standard item-quality color palette.
export const ITEM_QUALITY_COLORS = [
	'#9d9d9d', // Poor
	'#ffffff', // Common
	'#1eff00', // Uncommon
	'#0070dd', // Rare
	'#a335ee', // Epic
	'#ff8000', // Legendary
	'#e6cc80', // Artifact
	'#00ccff' // Heirloom
];

export function itemClassLabel(cls: number, subclass: number): string {
	const info = ITEM_CLASSES[cls];
	if (!info) return `Class ${cls}/${subclass}`;
	const sub = info.subclasses[subclass];
	return sub ? `${info.name} — ${sub}` : info.name;
}

export function qualityColor(quality: number): string {
	return ITEM_QUALITY_COLORS[quality] ?? ITEM_QUALITY_COLORS[1];
}
