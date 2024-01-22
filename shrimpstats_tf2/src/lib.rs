#![deny(
    unsafe_code,
    clippy::correctness,
    clippy::nursery,
    clippy::pedantic,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::suspicious
)]


use sm_ext::{forwards, native, ExecType, ICallableApi, IExtension, IExtensionInterface, IShareSys, SMExtension};

#[derive(Default, SMExtension)]
#[extension(name = "shrimpstats_tf2", description = "ShrimpStats TF2 Extension")]
pub struct ShrimpStatsTF2Extension;


// Name:	player_death
// Structure:
// short	userid	user ID who died
// long	victim_entindex
// long	inflictor_entindex	ent index of inflictor (a sentry, for example)
// short	attacker	user ID who killed
// string	weapon	weapon name killer used
// short	weaponid	ID of weapon killed used
// long	damagebits	bits of type of damage
// short	customkill	type of custom kill
// short	assister	user ID of assister
// string	weapon_logclassname	weapon name that should be printed on the log
// short	stun_flags	victim's stun flags at the moment of death
// short	death_flags	death flags.
// bool	silent_kill
// short	playerpenetratecount
// string	assister_fallback	contains a string to use if "assister" is -1
// short	kill_streak_total	Kill streak count (level)
// short	kill_streak_wep	Kill streak for killing weapon
// short	kill_streak_assist	Kill streak for assister count
// short	kill_streak_victim	Victims kill streak
// short	ducks_streaked	Duck streak increment from this kill
// short	duck_streak_total	Duck streak count for attacker
// short	duck_streak_assist	Duck streak count for assister
// short	duck_streak_victim	(former) duck streak count for victim
// bool	rocket_jump	was the victim rocket jumping
// short	weapon_def_index	item def index of weapon killer used
// short	crit_type	Crit type of kill. (0: None, 1: Mini, 2: Full)

// #[forwards]
// pub struct ShrimpStatsTF2ExtensionForward{
//     #[global_forward("player_death", ExecType::Hook)]
//     pub player_death: fn(userid: i32, victim_entindex: i32, inflictor_entindex: i32, attacker: i32, weapon: , weaponid: i32, damagebits: i32, customkill: i32, assister: i32, weapon_logclassname: String, stun_flags: i32, death_flags: u8, silent_kill: bool, playerpenetratecount: u8, assister_fallback: String, kill_streak_total: u8, kill_streak_wep: u8, kill_streak_assist: u8, kill_streak_victim: u8, ducks_streaked: u8, duck_streak_total: u8, duck_streak_assist: u8, duck_streak_victim: u8, rocket_jump: bool, weapon_def_index: i32, crit_type: u8)
// }

impl IExtensionInterface for ShrimpStatsTF2Extension {
    fn on_extension_load(&mut self, _me: IExtension, _sys: IShareSys, _late: bool) -> Result<(), Box<dyn std::error::Error>> {

        Ok(())
    }

    fn on_extension_unload(&mut self) {}

    fn on_extensions_all_loaded(&mut self) {
    }
}