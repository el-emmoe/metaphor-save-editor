#[derive(Debug)]
#[allow(unused)]
pub(crate) struct StatsGeneral {
    pub(crate) name: String,
    pub(crate) mag: u32,
    pub(crate) money: u32,
    pub(crate) courage: u16,
    pub(crate) wisdom: u16,
    pub(crate) tolerance: u16,
    pub(crate) eloquence: u16,
    pub(crate) imagination: u16,
}

#[derive(Debug)]
#[allow(unused)]
pub(crate) struct StatsParty {
    pub(crate) currhp: u32,
    pub(crate) currmp: u32,
    pub(crate) totalhp: u32,
    pub(crate) totalmp: u32,
    pub(crate) lvl: u32,
    pub(crate) exp: u32,
    pub(crate) strength: u8,
    pub(crate) magic: u8,
    pub(crate) endurance: u8,
    pub(crate) agility: u8,
    pub(crate) luck: u8,
}
