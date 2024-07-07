mod ltr {
	#![allow(non_snake_case)]
	const BANISH_FROM_EDORAS: &'static str = "LTR C 0001;Banish from Edoras;4 w;Sorcery;C;\"This spells costs 2 less to cast if it targets a tapped creature.\\r\\nExile target creature.\"" ;
	const THE_BATTLE_OF_BYWATER: &'static str = "LTR R 0002;The Battle of Bywater;1 w w;Sorcery;R;\"Destroy all creatures with power 3 or greater. Then create a Food token for each creature you control.\"" ;
	const BILL_THE_PONEY: &'static str = "LTR U 0003;Bill the Poney;3 w;Legendary Creature - Horse;U;\"When Bill the Pony enters the battlefield, create two Food tokens.\\r\\nSacrifice a Food: Until end of turn, target creature you control assigns combat damage equal to its toughness rather than its power.\"" ;
	const BOROMIR__WARDEN_OF_THE_TOWER: &'static str = "LTR R 0004;\"Boromir, Warden of the Tower\";2 w;Legendary Creature - Human Soldier;R;\"Vigilence\\r\\nWhenever an opponent casts a spell, if no mana was spent to cast it, counter that spell.\\r\\nSacrifice Boromir, Warden of the Tower: Creatures you control gain indestructible until end of turn. The Ring tempts you.\"" ;
	const DAWN_OF_A_NEW_AGE: &'static str = "LTR M 0005;Dawn of a New Age;1 w;Enchantment;M;\"Dawn of a New Age enters the battlefield with a hope counter on it for each of the creature you control.\\r\\nAt the beginning of your end step, remove a hope counter from Dawn of a New Age. If you do, draw a card. Then if Dawn of a New Age has no hope counters on it, sacrifice it and you gain 4 life.\"" ;
	pub fn collection() -> crate::collections::Collection<'static>
	{
		crate::collections::Collection::make(vec![
			BANISH_FROM_EDORAS,
			THE_BATTLE_OF_BYWATER,
			BILL_THE_PONEY,
			BOROMIR__WARDEN_OF_THE_TOWER,
			DAWN_OF_A_NEW_AGE,

		])
	}
}
