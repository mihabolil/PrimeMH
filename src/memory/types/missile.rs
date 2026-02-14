use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::memory::{
    process::D2RInstance,
    structs::{MissileData, Unit},
};

use super::get_position;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MissileUnit {
    pub unit_id: u32,
    pub txt_file_no: Missile,
    pub pos_x: f32,
    pub pos_y: f32,
    pub p_path: u64,
    pub missile_type: MissileType,
    pub missile_color: u32,
    pub missile_data: MissileData,
    pub collided: bool,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub enum MissileType {
    Fire,
    Ice,
    Lightning,
    Poison,
    Magic,
    Physical,
    Sfx,
    FxTrigger,
    #[default]
    Dummy,
}

impl MissileUnit {
    pub fn new(d2rprocess: &D2RInstance, unit: Unit, player_pos_x: f32, player_pos_y: f32, unit_id: u32) -> Self {
        let txt_file_no: Missile = Missile::from_u32(unit.txt_file_no).unwrap_or(Missile::Unknown);
        let (pos_x, pos_y) = get_position(d2rprocess, unit);
        let missile_type = get_missile_type(&txt_file_no);
        let missile_color = get_missile_color(&txt_file_no);
        let missile_data: MissileData = d2rprocess.read_mem::<MissileData>(unit.p_unit_data);
        let mut collided = false;
        if (txt_file_no == Missile::BattleOrders || txt_file_no == Missile::BattleCommand)
            && missile_data.base_skill_level < 100
            && missile_data.base_skill_level > 0
        {
            collided = (unit_id == missile_data.dw_owner_id)
                || ((pos_x - player_pos_x).abs() < 2.5 && (pos_y - player_pos_y).abs() < 2.5);
        }

        MissileUnit {
            unit_id: unit.unit_id,
            txt_file_no,
            pos_x,
            pos_y,
            p_path: unit.p_path,
            missile_type,
            missile_color,
            missile_data,
            collided,
        }
    }
}

#[derive(FromPrimitive, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Missile {
    Arrow,
    Javelin,
    Bighead1,
    Bighead2,
    Bighead3,
    Bighead4,
    Bighead5,
    Spike1,
    Spike2,
    Spike3,
    Spike4,
    Spike5,
    Firearrow,
    CrArrow1,
    CrArrow2,
    CrArrow3,
    CrArrow4,
    CrArrow5,
    Blood1,
    Blood2,
    Bigblood1,
    Bigblood2,
    Shafire1,
    Shafire2,
    Shafire3,
    Shafire4,
    Shafire5,
    MagicArrow,
    IceArrow,
    FireExplode,
    IceExplode,
    Bolt,
    AndarielsSpray,
    BigheadExp,
    ShamanExp,
    ThrowAxe,
    ThrowKnife,
    Glaive,
    PoisonJav,
    PoisonJavCloud,
    ColdArrow,
    ExplodingArrow,
    ExplodingArrowExp,
    PlagueJavelin,
    OilPotion,
    ExplosivePotion,
    FulminatingPotion,
    RancidGasePotion,
    ChokingGasPoition,
    StranglingGasPotion,
    Notused50,
    ExplosivePotionExp,
    ExplosivePotionDebris1,
    ExplosivePotionDebris2,
    ExplosivePotionDebris3,
    HolyBolt,
    ChargedBolt,
    SanctuaryBolt,
    FireBolt,
    IceBolt,
    InfernoFlame1,
    InfernoFlame2,
    FireBall,
    Mummy1,
    Mummy2,
    Mummy3,
    Mummy4,
    Blaze,
    FireWallMaker,
    FireWall,
    Goospit1,
    Goospit2,
    Goospit3,
    Goospit4,
    Goospit5,
    Goosplat,
    Sand,
    SandPile,
    UnholyBolt1,
    UnholyBolt2,
    UnholyBolt3,
    UnholyBolt4,
    SanctuaryCenter,
    FireExplosion,
    StuckArrow,
    Footprint,
    ImmolationArrow,
    GuidedArrow,
    FreezingArrow,
    FreezingArrowExp1,
    FreezingArrowExp2,
    Nova,
    IceBlast,
    BlessedHammer,
    ChainLightning,
    FistOfAres,
    ChillBlood,
    GlacialSpike,
    Teleport,
    LightningBolt,
    LightningHit,
    Meteor,
    MeteorCenter,
    MeteorTail,
    MeteorExplode,
    FireSmall,
    FireMedium,
    MonBlizCenter,
    MonBliz1,
    MonBliz2,
    MonBliz3,
    MonBliz4,
    MonBlizExplode1,
    MonBlizExplode2,
    MonBlizExplode3,
    Teeth,
    CorpseExplosion,
    PoisonCorpseExplosion,
    MonsterCorpseExplode,
    PoisonNova,
    FrostNova,
    Rogue1,
    Rogue2,
    Rogue3,
    BatLightningBolt,
    BatLightningTrail,
    SkMage1,
    SkMage2,
    SkMage3,
    SkMage4,
    VampireFireball,
    VampireFirewallMaker,
    VampireFirewall,
    VampireMeteor,
    VampireMeteorCenter,
    VampireMeteorExp,
    Raven1,
    AmphibianGoo1,
    AmphibianGoo2,
    TentacleGoo,
    AmphibianExplode,
    PoisonPuff,
    CurseEffectRed,
    SpiderGooLay,
    FetishInferno1,
    FetishInferno2,
    SpiderGoo,
    CurseCast,
    Howl,
    Shout,
    Dust,
    RedlightMissile,
    GreenlightMissile,
    BluelightMissile,
    WhitelightMissile,
    CorpsePoisonCloud,
    ChillBloodCloud,
    ChillBloodPuff,
    BlizzardCenter,
    Blizzard1,
    Blizzard2,
    Blizzard3,
    Blizzard4,
    BlizzardExplode1,
    BlizzardExplode2,
    BlizzardExplode3,
    ThunderStorm1,
    ThunderStorm2,
    ThunderStorm3,
    ThunderStorm4,
    MonsterLight,
    PoisonBall,
    DiabLight,
    Redemption,
    RedemptionFail,
    HandOfGod,
    DiabFire,
    FingerMageSpider,
    Electric,
    DiabThrowAxe,
    DiabWallMaker,
    DiabWall,
    CurseAmplifyDamage,
    CurseDimVision,
    CurseWeaken,
    CurseIronMaiden,
    CurseTerror,
    CurseAttract,
    CurseReverseVampire,
    CurseConfuse,
    CurseDecrepify,
    CurseLowerResist,
    CurseCenter,
    BoneSpear,
    BoneSpirit,
    ColdUnique,
    LightUnique,
    SkBowArrow1,
    SkBowArrow2,
    SkBowArrow3,
    SkBowArrow4,
    SkBowArrow5,
    Nova1,
    Nova2,
    AndyPoisonBolt,
    TeethExplode,
    LightningJavelin,
    LightningFury,
    BoneWallMaker,
    Necromage1,
    Necromage2,
    Necromage3,
    Necromage4,
    Sparkle,
    MultipleShotArrow,
    MultipleShotBolt,
    ChargedStrikeBolt,
    BoneSpearExplode,
    PoisonExplosionCloud,
    BoneCast,
    BattleCry,
    PrimePoisonCloud,
    PlagueJavCloud,
    RancidGasCloud,
    ChokingGasCloud,
    StranglingGasCloud,
    BugLightning,
    PantherJav1,
    PantherJav2,
    PantherJav3,
    PantherJav4,
    ImmolationFire,
    FuryLightning,
    LightningStrike,
    FistOfTheHeavensDelay,
    FistOfTheHeavensBolt,
    WarCry,
    BattleCommand,
    BattleOrders,
    PantherPotOrange,
    PantherPotGreen,
    MeteorFire,
    TrapSpikeRight,
    TrapSpikeLeft,
    TrapCursedSkullRight,
    TrapCursedSkullLeft,
    TrapPoisonBallRight,
    TrapPoisonBallLeft,
    Hydra,
    BoneSpearTrail,
    GrimWardSmallStart,
    GrimWardSmall,
    GrimWardSmallStop,
    GrimWardMediumStart,
    GrimWardMedium,
    GrimWardMediumStop,
    GrimWardLargeStart,
    GrimWardLarge,
    GrimWardLargeStop,
    ZakarumLight,
    GrimWardScare,
    FrozenOrb,
    FrozenOrbBolt,
    FrozenOrbNova,
    FrozenOrbExplode,
    ChillingArmorBolt,
    FireExplosion2,
    Blowgun,
    ChainLightning2,
    ReviveSmall,
    ReviveMedium,
    ReviveLarge,
    MonGlacialSpike,
    IceBreakSmall,
    IceBreakMedium,
    IceBreakLarge,
    IceBreakSmoke,
    MephistoFirehead,
    Whilrwind,
    ArcaneLightningBolt,
    FrogFire,
    FrogCold,
    FrogPois,
    DesertFireball,
    BrDeathControl,
    BrDeathLightningBolt,
    BrDeathLightningHit,
    DenOfEvilLight,
    CairnStones,
    CairnStonesSky,
    CairnStonesGround,
    TowerMist,
    TowerMistTrail,
    BrDeathSmokes1,
    BrDeathSmokeNu,
    BrDeathSmokeDt,
    BrDeathSpirits1,
    BrDeathSpiritNu,
    BrDeathSpiritDt,
    MephistoDeathControl,
    MephistoFirewallMaker,
    MephistoFirewall,
    MephistoFlyingRocksBig,
    MephistoExplosionBig,
    MephistoFlyingRocksSmall,
    MephistoExplosionSmall,
    MephistoDoNotDraw,
    AndyControl0,
    AndyFirewallMaker,
    AndyFirewall,
    AndyColumnFireBase,
    AndyColumnFire,
    AndyFallingDebris1,
    AndyFallingDebris2,
    AndyFallingDebris3,
    AndyDebrisExplosion1,
    AndyDebrisExplosion2,
    AndyDebrisExplosion3,
    AndyDebrisExplosion4,
    AndyDebrisExplosion5,
    WillowisplightningBolt,
    QueenPoisonCloud,
    Dirt,
    DirtPile,
    UndeadMissile1,
    UndeadMissile2,
    UndeadMissile3,
    UndeadMissile4,
    BoneSpiritExplode,
    DopplezonExplode,
    MonBoneSpirit,
    TowerMistFade,
    CountessFirewall,
    TowerChestSpawner,
    HellMeteorLaunch1,
    HellMeteorLaunch2,
    HellMeteorUp,
    HellMeteorDown,
    HellMeteorBall,
    HoradricStaff,
    HoradricLightning,
    HoradricLight,
    RegurgitatorCorpse,
    RegurgitatorCorpseExpl,
    HighPriestLightning,
    IceBreakSmallMelt,
    IceBreakLargeMelt,
    LeapKnockback,
    RadamentDeath,
    RadamentHandOfGod,
    RadamentHolyBolt,
    TaintedSunControl,
    TaintedSunFlash,
    TaintedSunBall,
    QueenDeathCenter,
    QueenDeathGlob,
    QueenDeathSplat1,
    QueenDeathSplat2,
    HealingBolt,
    MephistoHoleDelay,
    MephistoHoleBirth,
    MephistoHoleNeutral,
    MephistoHoleDeath,
    MephistoHoleDead,
    DurielDeathControl,
    DurielDeathRock,
    DurielDeathDebris,
    DurielDeathSmoke,
    MephistoExplosion,
    OrbMist,
    OrbMistTrail,
    OrbMistFade,
    Pilum,
    DiabloAppears,
    HfControl,
    HfFragment1,
    HfFragment2,
    HfFragment3,
    HfSpirit1,
    HfReserved3,
    IzualControl,
    IzualMistLoop,
    IzualMistFade,
    IzualLightning,
    IzualLightningTrail,
    CairnStonesBolt,
    BombInAir,
    BombOnGround,
    BombExplosion,
    ShockFieldInAir,
    ShockFieldOnGround,
    ThrowingStar,
    AcidSpray,
    BladeCreeper,
    Distraction,
    DistractionFog,
    DistractionPuff,
    DistractionStart,
    DistractionEnd,
    ImpInfernoFlame1,
    ImpInfernoFlame2,
    BaalLightningBolt,
    BaalLightningTrail,
    BaalLightningBolt2,
    BaalLightningTrail2,
    ImpFireball,
    ImpFireballExplode,
    CatapultChargedBallOn,
    CatapultChargedBall,
    CatapultChargedBallBolt,
    ImpSpawnMonsters,
    CatapultSpikeBallOn,
    CatapultSpikeBall,
    CatapultSpikeInAir,
    CatapultSpikeOnGround,
    CatapultSpikeExplosion,
    CatapultColdBallOn,
    CatapultColdBall,
    CatapultColdExplosion,
    CatapultPlagueBallOn,
    CatapultPlagueBall,
    CatapultPlagueCloud,
    CatapultMeteorBallOn,
    CatapultMeteorBall,
    CatapultMeteorFire,
    TowerDeath,
    HealingVortex,
    SuicideCorpseExplode,
    SuicideFireExplode,
    SuicideIceExplode,
    ExplodingJavalin,
    ExplodingJavalinExp,
    LightingTrailingJavalin,
    LightJavalinTrail,
    LightJavalinExplosion,
    IceJavalin,
    IceJavalinExplode,
    PlagueJavelin2,
    PlagueJavlinExplode,
    AdvLightTrailingJav,
    AdvLightTrailingJav2,
    AdvLightJavExplode,
    SucFireball,
    SucFireballExplode,
    SucFireballTrail,
    SucShockFieldMissile,
    SucShockFieldMissileExp,
    SucShockField,
    HellfireMissile,
    HellfireExa,
    HellfireExb,
    ImpChargedBolt,
    ImpTeleport,
    MoltenBoulder,
    MoltenBoulderEmerge,
    MoltenBoulderExplode,
    MoltenBoulderFirePath,
    MoltenBoulderFlyingRocks,
    Firestorm,
    FirestormMaker,
    ArcticBlast1,
    ArcticBlast2,
    ErruptionCenter,
    ErruptionCrack1,
    ErruptionCrack2,
    ErruptionSmoke1,
    ErruptionSmoke2,
    VineBeastWalk1,
    VineBeastWalk2,
    VineBeastNeutral,
    VineBeastAttack,
    VineBeastDeath,
    Vines,
    VinesTrail,
    VinesWither,
    PlagueVines,
    PlagueVinesTrail,
    PlagueVinesWither,
    Twister,
    Tornado,
    Volcano,
    VolcanoOverlay,
    Fire,
    VolcanoDebris2,
    VolcanoExplosion,
    VolcanoSmallFire,
    DragonBreathMissile,
    LureProjectile,
    LureCenter,
    LureCloud,
    ImpMiss1,
    ImpMiss2,
    ImpMiss3,
    ImpMiss4,
    ImpMiss5,
    FrozenHorrorArcticBlast1,
    FrozenHorrorArcticBlast2,
    SentryChargedBolt,
    SentrySpikeInAir,
    SentrySpikeOnGround,
    RecyclerDelay,
    RecyclerVine,
    RecyclerFade,
    RecyclerExplosion,
    DeathMauler,
    DeathMaulerTrail,
    DeathMaulerTrailFade,
    BladeFury1,
    BladeFragment1,
    BladeFury2,
    BladeFragment2,
    BladeFury3,
    BladeFragment3,
    ShockWave,
    LightningTalons,
    LightningTalonsTrail,
    PhoenixTrail,
    RabiesPlague,
    RabiesContagion,
    WakeOfDestructionMaker,
    WakeOfDestructionDeathSentryExplode,
    TigerFury,
    TigerFuryTrail,
    TigerFuryTrail2,
    InfernoSentry1,
    InfernoSentry2,
    AncientThrowingAxe,
    SentryLightningBolt,
    SentryLightningHit,
    AnyaCenter,
    AnyaIcicle,
    AnyaIceImpact,
    AnyaIceSteam,
    AnyaIceMagic,
    DragonTailMissile,
    DragonFlight,
    DragonFlightMaker,
    ProgressiveRadiusDamage,
    VineBeastWalk1Fade,
    VineBeastWalk2Fade,
    VineBeastNeutralFade,
    VineRecyclerDelay,
    AncientDeathCenter,
    AncientDeathCloud,
    LightningChargeUp,
    ChargeUpNova,
    ChainLightningChargeUp,
    PainWormAppear,
    BaalTauntControl,
    BaalTauntLightning,
    BaalTauntLightningTrail,
    BaalTauntPoison,
    BaalSpawnMonsters,
    MindBlastHit,
    BladeShieldMissile,
    BladeShieldAttachment,
    BaalInferno,
    BaalNova,
    FistsOfFireExplode,
    FistsOfFireFirewall,
    ClawsOfThunderBolt,
    ClawsOfThunderNova,
    BladesOfIceExplode,
    BladesOfIceCubes,
    BladesOfIceCubesMelt,
    RoyalStrikeMeteor,
    RoyalStrikeMeteorCenter,
    RoyalStrikeMeteorTail,
    RoyalStrikeMeteorExplode,
    RoyalStrikeMeteorFire,
    RoyalStrikeChainLightning,
    RoyalStrikeChaosIce,
    WorldStoneChip1,
    WorldStoneChip2,
    WorldStoneChip3,
    WorldStoneChip4,
    HighPriestLightning2,
    InfernoFlame3,
    MindBlastCenter,
    ArmageddonControl,
    ArmageddonRock,
    ArmageddonTail,
    ArmageddonExplosion,
    HurricaneSwoosh,
    HurricaneCart,
    HurricaneRock,
    HurricaneSack,
    HurricaneTree,
    HurricaneVase,
    BaalCorpseExplodeDelay,
    BaalCorpseExplodeExpl,
    BaalColdMaker,
    BaalColdTrail,
    BaalSpawnMonstersExp,
    ImpMiss21,
    ImpMiss22,
    ImpMiss23,
    ImpMiss24,
    ImpMiss25,
    AnyaSteam1,
    AnyaSteam2,
    AncientsGuide,
    AncientsMarker,
    AncientsControl,
    OverseerControl,
    Nihlithak1,
    Nihlithak2,
    Nihlithak3,
    NihlathakControl,
    NihlathakSwoosh,
    NihlathakDebris1,
    NihlathakDebris2,
    NihlathakDebris3,
    NihlathakDebris4,
    NihlathakGlow,
    BaalTeleport,
    BaalCloneDeath,
    AnyaSteamVent,
    AnyaSteam,
    NihlathakHole,
    NihlathakHoleLight,
    VolcanoFireTrail,
    NihlathakGlow2,
    NihlathakBoneChips,
    BaalCorpseExplodeFade,
    ArmageddonFire,
    IceSparkle,
    BaalFxControl,
    BaalFxSpirit1,
    BaalFxSpirit2,
    BaalFxSpirit3,
    BaalFxSpirit4,
    BaalFxSpirit5,
    BaalFxBaalHeadAppear,
    BaalFxBaalHead1,
    BaalFxBaalHead2,
    BaalFxBaalHead3,
    BaalFxTyrealDebris1,
    BaalFxTyrealDebris2,
    BaalFxTyrealDebris3,
    BaalFxTyrealDebrisBreak,
    WorldstoneShake,
    BlessedHammerEx,
    SentryLightningBolt2,
    SentryLightningHit2,
    LightningTowerNova,
    SkBowArrow6,
    SkBowArrow7,
    SkBowArrow8,
    Bighead6,
    ViperPoisJav,
    ViperPoisJavCloud,
    ViperFire,
    ViperFireCloud,
    ViperBoneSpear,
    CountessFirewallMaker,
    BaalTauntLightningControl,
    BaalTauntPoisonControl,
    ExplodingArrowExp2,
    FreezingArrowExp3,
    PantherJav5,
    Spike6,
    CrArrow6,
    SkMagePois,
    SkMageCold,
    SkMageFire,
    SkMageLtng,
    SuccubusMiss,
    WillowisplightningBolt2,
    MummyEx,
    GoospitEx,
    ImpMissEx,
    DiablogeddonControl,
    DiablogeddonRock,
    DiablogeddonTail,
    DiablogeddonExplosion,
    DiablogeddonFire,
    MegademonInferno,
    TrapFireBolt,
    TrapPoisonJavCloud,
    TrapNova,
    MephFrostNova,
    MephLight,
    VampireMeteorFire,
    StrafeArrow,
    StrafeBolt,
    RecklessAttacksmissile,
    LeapAttack,
    SigilLethargyMedium,
    SigilRancorMedium,
    SigilDeathMedium,
    RingOfFire,
    FlameWave,
    MindWallMaker,
    BladeWarpExplode,
    MiasmaBoltDot,
    AbyssalShatterShards,
    FlameWaveFire,
    MiasmaChainsCloud,
    MiasmaBolt,
    MiasmaPuff,
    SigilLethargySmall,
    SigilRancorSmall,
    SigilDeathSmall,
    SigilLethargyLarge,
    SigilRancorLarge,
    SigilDeathLarge,
    MiasmaBoltCloud,
    EchoingStrike,
    BladeWarp,
    FlameWaveUnveiling,
    AbyssCenter,
    Engorge,
    MiasmaChainsMaker,
    MiasmaChains,
    MiasmaChainsCloudMaker,
    Abyss,
    AbyssExplode,
    FlameWaveLingerFire,
    HexPurgeExplode,
    EngorgeCorpseEffect,
    BloodBoilExplode,
    EchoingStrikeDetonate,
    EldritchBlastNova,
    Apocalypse,
    BindDemonChannelMaker,
    BindDemonChannel,
    ColdFissureCenter,
    IceCrack1,
    IceCrack2,
    IceVapor1,
    IceVapor2,
    RingOfFireExplode,
    ColossalThrowingAxe,
    FireTwister,
    TaintedFireBolt,
    TaintedFireBall,
    ColossalChargedBolt,
    Unknown,
}

fn get_missile_color(txt_file_no: &Missile) -> u32 {
    let arrow_color = 0xFFFFFF54;
    let javelin_color = 0xFFFFFF54;
    let bighead1_color = 0x8F8FFF54;
    let bighead2_color = 0x8F8FFF54;
    let bighead3_color = 0x8F8FFF54;
    let bighead4_color = 0x8F8FFF54;
    let bighead5_color = 0x8F8FFF54;
    let spike1_color = 0xFFFFFF54;
    let spike2_color = 0xFFFFFF54;
    let spike3_color = 0xFFFFFF54;
    let spike4_color = 0xFFFFFF54;
    let spike5_color = 0xFFFFFF54;
    let firearrow_color = 0xFFB24054;
    let crarrow1_color = 0xFFFFFF54;
    let crarrow2_color = 0xFFFFFF54;
    let crarrow3_color = 0xFFFFFF54;
    let crarrow4_color = 0xFFFFFF54;
    let crarrow5_color = 0xFFFFFF54;
    let blood1_color = 0xFFFFFF54;
    let blood2_color = 0xFFFFFF54;
    let bigblood1_color = 0xFFFFFF54;
    let bigblood2_color = 0xFFFFFF54;
    let shafire1_color = 0xFFB24054;
    let shafire2_color = 0xFFB24054;
    let shafire3_color = 0xFFB24054;
    let shafire4_color = 0xFFB24054;
    let shafire5_color = 0xFFB24054;
    let magicarrow_color = 0xFFFFFF54;
    let icearrow_color = 0x5151FF54;
    let fireexplode_color = 0xFFB24054;
    let iceexplode_color = 0x5151FF54;
    let bolt_color = 0xFFFFFF54;
    let andarielspray_color = 0xFFFFFF54;
    let bigheadexp_color = 0x8F8FFF54;
    let shamanexp_color = 0xFFB24054;
    let throwaxe_color = 0xFFFFFF54;
    let throwknife_color = 0xFFFFFF54;
    let glaive_color = 0xFFFFFF54;
    let poisonjav_color = 0x80FF8054;
    let poisonjavcloud_color = 0x80FF8054;
    let coldarrow_color = 0x5151FF54;
    let explodingarrow_color = 0xFFB24054;
    let explodingarrowexp_color = 0xFFB24054;
    let plaguejavelin_color = 0x80FF8054;
    let oilpotion_color = 0xFF808054;
    let explosivepotion_color = 0xFF808054;
    let fulminatingpotion_color = 0xFF808054;
    let rancidgasepotion_color = 0x80FF8054;
    let chokinggaspoition_color = 0x80FF8054;
    let stranglinggaspotion_color = 0x80FF8054;
    let notused50_color = 0xFFFFFF54;
    let explosivepotionexp_color = 0xFFB24054;
    let explosivepotiondebris1_color = 0xFFFFFF54;
    let explosivepotiondebris2_color = 0xFFFFFF54;
    let explosivepotiondebris3_color = 0xFFFFFF54;
    let holybolt_color = 0xDEDEFF54;
    let chargedbolt_color = 0xFFFFFF54;
    let sanctuarybolt_color = 0xDEDEFF54;
    let firebolt_color = 0xFFB24054;
    let icebolt_color = 0x5151FF54;
    let infernoflame1_color = 0xFFB24054;
    let infernoflame2_color = 0xFFB24054;
    let fireball_color = 0xFFB24054;
    let mummy1_color = 0xFFFFFF54;
    let mummy2_color = 0xFFFFFF54;
    let mummy3_color = 0xFFFFFF54;
    let mummy4_color = 0xFFFFFF54;
    let blaze_color = 0xFFB24054;
    let firewallmaker_color = 0xFFB24054;
    let firewall_color = 0xFFB24054;
    let goospit1_color = 0xFFFFFF54;
    let goospit2_color = 0xFFFFFF54;
    let goospit3_color = 0xFFFFFF54;
    let goospit4_color = 0xFFFFFF54;
    let goospit5_color = 0xFFFFFF54;
    let goosplat_color = 0xFFFFFF54;
    let sand_color = 0xFFFFFF54;
    let sandpile_color = 0xFFFFFF54;
    let unholybolt1_color = 0x00000054;
    let unholybolt2_color = 0x00000054;
    let unholybolt3_color = 0x00000054;
    let unholybolt4_color = 0x00000054;
    let sanctuarycenter_color = 0x00000054;
    let fireexplosion_color = 0xFFB24054;
    let stuckarrow_color = 0x00000054;
    let footprint_color = 0x00000054;
    let immolationarrow_color = 0xFFB24054;
    let guidedarrow_color = 0xFFFFFF54;
    let freezingarrow_color = 0x5151FF54;
    let freezingarrowexp1_color = 0x5151FF54;
    let freezingarrowexp2_color = 0x5151FF54;
    let nova_color = 0xFFFFFF54;
    let iceblast_color = 0x5151FF54;
    let blessedhammer_color = 0xDEDEFF54;
    let chainlightning_color = 0xFFFFFF54;
    let fistofares_color = 0xFFFFFF54;
    let chillblood_color = 0x5151FF54;
    let glacialspike_color = 0x5151FF54;
    let teleport_color = 0xFFFFFF54;
    let lightningbolt_color = 0xFFFFFF54;
    let lightninghit_color = 0xFFFFFF54;
    let meteor_color = 0x00000054;
    let meteorcenter_color = 0xFF404054;
    let meteortail_color = 0x00000054;
    let meteorexplode_color = 0xFFB24054;
    let firesmall_color = 0xFFB24054;
    let firemedium_color = 0xFFB24054;
    let monblizcenter_color = 0x5151FF54;
    let monbliz1_color = 0x00000054;
    let monbliz2_color = 0x00000054;
    let monbliz3_color = 0x00000054;
    let monbliz4_color = 0x00000054;
    let monblizexplode1_color = 0x00000054;
    let monblizexplode2_color = 0x00000054;
    let monblizexplode3_color = 0x00000054;
    let teeth_color = 0xFFFFFF54;
    let corpseexplosion_color = 0xFFB24054;
    let poisoncorpseexplosion_color = 0xFFB24054;
    let monstercorpseexplode_color = 0xFFB24054;
    let poisonnova_color = 0x00000054;
    let frostnova_color = 0x5151FF54;
    let rogue1_color = 0xFFFFFF54;
    let rogue2_color = 0xFFB24054;
    let rogue3_color = 0x5151FF54;
    let batlightningbolt_color = 0xFFFFFF54;
    let batlightningtrail_color = 0xFFFFFF54;
    let skmage1_color = 0xFFFFFF54;
    let skmage2_color = 0xFFFFFF54;
    let skmage3_color = 0xFFFFFF54;
    let skmage4_color = 0xFFFFFF54;
    let vampirefireball_color = 0xFFB24054;
    let vampirefirewallmaker_color = 0xFFB24054;
    let vampirefirewall_color = 0xFFB24054;
    let vampiremeteor_color = 0x00000054;
    let vampiremeteorcenter_color = 0xFF404054;
    let vampiremeteorexp_color = 0xFFB24054;
    let raven1_color = 0xFFFFFF54;
    let amphibiangoo1_color = 0x40FF4054;
    let amphibiangoo2_color = 0x40FF4054;
    let tentaclegoo_color = 0x40FF4054;
    let amphibianexplode_color = 0xFFFFFF54;
    let poisonpuff_color = 0xFFFFFF54;
    let curseeffectred_color = 0xFF000054;
    let spidergoolay_color = 0x00000054;
    let fetishinferno1_color = 0xFFB24054;
    let fetishinferno2_color = 0xFFB24054;
    let spidergoo_color = 0x00000054;
    let cursecast_color = 0xFF000054;
    let howl_color = 0xC0C0C054;
    let shout_color = 0xC0C0C054;
    let dust_color = 0xFFFFFF54;
    let redlightmissile_color = 0xFFB24054;
    let greenlightmissile_color = 0x80FF8054;
    let bluelightmissile_color = 0x5151FF54;
    let whitelightmissile_color = 0xFFFFFF54;
    let corpsepoisoncloud_color = 0x80FF8054;
    let chillbloodcloud_color = 0x5151FF54;
    let chillbloodpuff_color = 0x5151FF54;
    let blizzardcenter_color = 0x5151FF54;
    let blizzard1_color = 0x00000054;
    let blizzard2_color = 0x00000054;
    let blizzard3_color = 0x00000054;
    let blizzard4_color = 0x00000054;
    let blizzardexplode1_color = 0x00000054;
    let blizzardexplode2_color = 0x00000054;
    let blizzardexplode3_color = 0x00000054;
    let thunderstorm1_color = 0xFFFFFF54;
    let thunderstorm2_color = 0xFFFFFF54;
    let thunderstorm3_color = 0xFFFFFF54;
    let thunderstorm4_color = 0xFFFFFF54;
    let monsterlight_color = 0xFFFFFF54;
    let poisonball_color = 0xFFFFFF54;
    let diablight_color = 0xFFFFFF54;
    let redemption_color = 0xFFFFFF54;
    let redemptionfail_color = 0xFFB24054;
    let handofgod_color = 0xFFFFFF54;
    let diabfire_color = 0xFFFFFF54;
    let fingermagespider_color = 0xFFB2B254;
    let electric_color = 0xFFFFFF54;
    let diabthrowaxe_color = 0xFFFFFF54;
    let diabwallmaker_color = 0xFFB24054;
    let diabwall_color = 0xFFB24054;
    let curseamplifydamage_color = 0xFF000054;
    let cursedimvision_color = 0xFF000054;
    let curseweaken_color = 0xFF000054;
    let curseironmaiden_color = 0xFF000054;
    let curseterror_color = 0xFF000054;
    let curseattract_color = 0xFF000054;
    let cursereversevampire_color = 0xFF000054;
    let curseconfuse_color = 0xFF000054;
    let cursedecrepify_color = 0xFF000054;
    let curselowerresist_color = 0xFF000054;
    let cursecenter_color = 0xFF000054;
    let bonespear_color = 0xFFFFFF54;
    let bonespirit_color = 0xFFFFFF54;
    let coldunique_color = 0x5151FF54;
    let lightunique_color = 0xFFFFFF54;
    let skbowarrow1_color = 0xFFFFFF54;
    let skbowarrow2_color = 0xFFFFFF54;
    let skbowarrow3_color = 0xFFFFFF54;
    let skbowarrow4_color = 0xFFB24054;
    let skbowarrow5_color = 0xFFFFFF54;
    let nova1_color = 0xFFFFFF54;
    let nova2_color = 0xFFFFFF54;
    let andypoisonbolt_color = 0xFFFFFF54;
    let teethexplode_color = 0x00000054;
    let lightningjavelin_color = 0xFFFFFF54;
    let lightningfury_color = 0xFFFFFF54;
    let bonewallmaker_color = 0xFFFFFF54;
    let necromage1_color = 0xFFFFFF54;
    let necromage2_color = 0xFFFFFF54;
    let necromage3_color = 0xFFFFFF54;
    let necromage4_color = 0xFFFFFF54;
    let sparkle_color = 0xFFFFFF54;
    let multipleshotarrow_color = 0xFFFFFF54;
    let multipleshotbolt_color = 0xFFFFFF54;
    let chargedstrikebolt_color = 0xFFFFFF54;
    let bonespearexplode_color = 0x00000054;
    let poisonexplosioncloud_color = 0x80FF8054;
    let bonecast_color = 0xFFFFFF54;
    let battlecry_color = 0xC0C0C054;
    let primepoisoncloud_color = 0xFFFFFF54;
    let plaguejavcloud_color = 0xFFFFFF54;
    let rancidgascloud_color = 0xFFFFFF54;
    let chokinggascloud_color = 0xFFFFFF54;
    let stranglinggascloud_color = 0xFFFFFF54;
    let buglightning_color = 0xFFFFFF54;
    let pantherjav1_color = 0xFFFFFF54;
    let pantherjav2_color = 0xFFFFFF54;
    let pantherjav3_color = 0xFFFFFF54;
    let pantherjav4_color = 0xFFFFFF54;
    let immolationfire_color = 0xFFB24054;
    let furylightning_color = 0xFFFFFF54;
    let lightningstrike_color = 0xFFFFFF54;
    let fistoftheheavensdelay_color = 0x00000054;
    let fistoftheheavensbolt_color = 0xDEDEFF54;
    let warcry_color = 0xC0C0C054;
    let battlecommand_color = 0xC0C0C054;
    let battleorders_color = 0xC0C0C054;
    let pantherpotorange_color = 0xFF808054;
    let pantherpotgreen_color = 0x80FF8054;
    let meteorfire_color = 0xFFB24054;
    let trapspikeright_color = 0xFFFFFF54;
    let trapspikeleft_color = 0xFFFFFF54;
    let trapcursedskullright_color = 0xFFFFFF54;
    let trapcursedskullleft_color = 0xFFFFFF54;
    let trappoisonballright_color = 0xFFFFFF54;
    let trappoisonballleft_color = 0xFFFFFF54;
    let hydra_color = 0xFFFFFF54;
    let bonespeartrail_color = 0xFFFFFF54;
    let grimwardsmallstart_color = 0x00000054;
    let grimwardsmall_color = 0x00000054;
    let grimwardsmallstop_color = 0x00000054;
    let grimwardmediumstart_color = 0x00000054;
    let grimwardmedium_color = 0x00000054;
    let grimwardmediumstop_color = 0x00000054;
    let grimwardlargestart_color = 0x00000054;
    let grimwardlarge_color = 0x00000054;
    let grimwardlargestop_color = 0x00000054;
    let zakarumlight_color = 0xFFFFFF54;
    let grimwardscare_color = 0x00000054;
    let frozenorb_color = 0x5151FF54;
    let frozenorbbolt_color = 0x5151FF54;
    let frozenorbnova_color = 0x5151FF54;
    let frozenorbexplode_color = 0x5151FF54;
    let chillingarmorbolt_color = 0x5151FF54;
    let fireexplosion2_color = 0x00000054;
    let blowgun_color = 0xFFFFFF54;
    let chainlightning2_color = 0xFFFFFF54;
    let revivesmall_color = 0xFFB24054;
    let revivemedium_color = 0xFFB24054;
    let revivelarge_color = 0xFFB24054;
    let monglacialspike_color = 0x5151FF54;
    let icebreaksmall_color = 0xFFFFFF54;
    let icebreakmedium_color = 0xFFFFFF54;
    let icebreaklarge_color = 0xFFFFFF54;
    let icebreaksmoke_color = 0xFFFFFF54;
    let mephisto_color = 0xFFFFFF54;
    let firehead_color = 0x00000054;
    let whilrwind_color = 0x00000054;
    let arcanelightningbolt_color = 0xB2B2FF54;
    let frogfire_color = 0xFF404054;
    let frogcold_color = 0x4040FF54;
    let frogpois_color = 0x40FF4054;
    let desertfireball_color = 0xFF404054;
    let brdeathcontrol_color = 0xFFFFFF54;
    let brdeathlightningbolt_color = 0xFFFFFF54;
    let brdeathlightninghit_color = 0xFFFFFF54;
    let denofevillight_color = 0xFFFFFF54;
    let cairnstones_color = 0xFFFFFF54;
    let cairnstonessky_color = 0xFFFFFF54;
    let cairnstonesground_color = 0xFFFFFF54;
    let towermist_color = 0xFFFFFF54;
    let towermisttrail_color = 0xFFFFFF54;
    let brdeathsmokes1_color = 0x00000054;
    let brdeathsmokenu_color = 0x00000054;
    let brdeathsmokedt_color = 0x00000054;
    let brdeathspirits1_color = 0xFFFFFF54;
    let brdeathspiritnu_color = 0xFFFFFF54;
    let brdeathspiritdt_color = 0xFFFFFF54;
    let mephistodeathcontrol_color = 0xFFFFFF54;
    let mephistofirewallmaker_color = 0xFFB24054;
    let mephistofirewall_color = 0xFFB24054;
    let mephistoflyingrocksbig_color = 0x00000054;
    let mephistoexplosionbig_color = 0xFFB24054;
    let mephistoflyingrockssmall_color = 0x00000054;
    let mephistoexplosionsmall_color = 0xFFB24054;
    let mephistodonotdraw_color = 0x00000054;
    let andycontrol0_color = 0x00000054;
    let andyfirewallmaker_color = 0xFFB24054;
    let andyfirewall_color = 0xFFB24054;
    let andycolumnfirebase_color = 0xFFB24054;
    let andycolumnfire_color = 0xC0808054;
    let andyfallingdebris1_color = 0x00000054;
    let andyfallingdebris2_color = 0x00000054;
    let andyfallingdebris3_color = 0x00000054;
    let andydebrisexplosion1_color = 0x00000054;
    let andydebrisexplosion2_color = 0x00000054;
    let andydebrisexplosion3_color = 0x00000054;
    let andydebrisexplosion4_color = 0x00000054;
    let andydebrisexplosion5_color = 0x00000054;
    let willowisplightningbolt_color = 0xB2B2FF54;
    let queenpoisoncloud_color = 0x40FF4054;
    let dirt_color = 0xFFFFFF54;
    let dirtpile_color = 0xFFFFFF54;
    let undeadmissile1_color = 0xFFFFFF54;
    let undeadmissile2_color = 0xFFFFFF54;
    let undeadmissile3_color = 0xFFFFFF54;
    let undeadmissile4_color = 0xFFFFFF54;
    let bonespiritexplode_color = 0xFFFFFF54;
    let dopplezonexplode_color = 0xFFFFFF54;
    let monbonespirit_color = 0xFFFFFF54;
    let towermistfade_color = 0xFFFFFF54;
    let countessfirewall_color = 0xFFB24054;
    let towerchestspawner_color = 0xC0C0FF54;
    let hellmeteorlaunch1_color = 0xFFFFFF54;
    let hellmeteorlaunch2_color = 0xFFFFFF54;
    let hellmeteorup_color = 0xFFFFFF54;
    let hellmeteordown_color = 0xFFFFFF54;
    let hellmeteorball_color = 0xFFFFFF54;
    let horadricstaff_color = 0xFFFFFF54;
    let horadriclightning_color = 0xFFFFFF54;
    let horadriclight_color = 0xFFFFFF54;
    let regurgitatorcorpse_color = 0xFF404054;
    let regurgitatorcorpseexpl_color = 0xFF404054;
    let highpriestlightning_color = 0xFF404054;
    let icebreaksmallmelt_color = 0xFFFFFF54;
    let icebreaklargemelt_color = 0xFFFFFF54;
    let leapknockback_color = 0x00000054;
    let radamentdeath_color = 0xFFFFFF54;
    let radamenthandofgod_color = 0x00000054;
    let radamentholybolt_color = 0xFFFFFF54;
    let taintedsuncontrol_color = 0xFFFFFF54;
    let taintedsunflash_color = 0xFFFFFF54;
    let taintedsunball_color = 0xFFFFFF54;
    let queendeathcenter_color = 0xFFFFFF54;
    let queendeathglob_color = 0xFFFFFF54;
    let queendeathsplat1_color = 0xFFFFFF54;
    let queendeathsplat2_color = 0xFFFFFF54;
    let healingbolt_color = 0xDEDEFF54;
    let mephistoholedelay_color = 0x00000054;
    let mephistoholebirth_color = 0x00000054;
    let mephistoholeneutral_color = 0x00000054;
    let mephistoholedeath_color = 0x00000054;
    let mephistoholedead_color = 0x00000054;
    let durieldeathcontrol_color = 0x00000054;
    let durieldeathrock_color = 0x00000054;
    let durieldeathdebris_color = 0x00000054;
    let durieldeathsmoke_color = 0x00000054;
    let mephistoexplosion_color = 0xFFFFFF54;
    let orbmist_color = 0xFF7D7D54;
    let orbmisttrail_color = 0xFF7D7D54;
    let orbmistfade_color = 0xFF7D7D54;
    let pilum_color = 0xFFFFFF54;
    let diabloappears_color = 0xFFFFFF54;
    let hfcontrol_color = 0xFFFFFF54;
    let hffragment1_color = 0xFFFFFF54;
    let hffragment2_color = 0xFFFFFF54;
    let hffragment3_color = 0xFFFFFF54;
    let hfspirit1_color = 0xFFFFFF54;
    let hfreserved3_color = 0xFFFFFF54;
    let izualcontrol_color = 0xFFFFFF54;
    let izualmistloop_color = 0xFFFFFF54;
    let izualmistfade_color = 0xFFFFFF54;
    let izuallightning_color = 0xFFFFFF54;
    let izuallightningtrail_color = 0xFFFFFF54;
    let cairnstonesbolt_color = 0xFFFFFF54;
    let bombinair_color = 0xFFFFFF54;
    let bombonground_color = 0xFFFFFF54;
    let bombexplosion_color = 0xFFB24054;
    let shockfieldinair_color = 0xFFFFFF54;
    let shockfieldonground_color = 0xFFFFFF54;
    let throwingstar_color = 0xFFFFFF54;
    let acidspray_color = 0xFFFFFF54;
    let bladecreeper_color = 0xFFFFFF54;
    let distraction_color = 0xFFFFFF54;
    let distractionfog_color = 0xFFFFFF54;
    let distractionpuff_color = 0xFFFFFF54;
    let distractionstart_color = 0xFFFFFF54;
    let distractionend_color = 0xFFFFFF54;
    let impinfernoflame1_color = 0xFFB24054;
    let impinfernoflame2_color = 0xFFB24054;
    let baallightningbolt_color = 0xFFFFFF54;
    let baallightningtrail_color = 0xFFFFFF54;
    let baallightningbolt2_color = 0xFFFFFF54;
    let baallightningtrail2_color = 0xFFFFFF54;
    let impfireball_color = 0xFFB24054;
    let impfireballexplode_color = 0xFFB24054;
    let catapultchargedballon_color = 0xFFFFFF54;
    let catapultchargedball_color = 0x8F8FFF54;
    let catapultchargedballbolt_color = 0xFFFFFF54;
    let impspawnmonsters_color = 0xFFB24054;
    let catapultspikeballon_color = 0xFFFFFF54;
    let catapultspikeball_color = 0xFFFFFF54;
    let catapultspikeinair_color = 0xFFFFFF54;
    let catapultspikeonground_color = 0xFFFFFF54;
    let catapultspikeexplosion_color = 0xFFFFFF54;
    let catapultcoldballon_color = 0xFFFFFF54;
    let catapultcoldball_color = 0x8F8FFF54;
    let catapultcoldexplosion_color = 0xFFFFFF54;
    let catapultplagueballon_color = 0xFFFFFF54;
    let catapultplagueball_color = 0x8F8FFF54;
    let catapultplaguecloud_color = 0x80FF8054;
    let catapultmeteorballon_color = 0xFFFFFF54;
    let catapultmeteorball_color = 0x8F8FFF54;
    let catapultmeteorfire_color = 0xFFB24054;
    let towerdeath_color = 0xFFFFFF54;
    let healingvortex_color = 0xFFB24054;
    let suicidecorpseexplode_color = 0xFFB24054;
    let suicidefireexplode_color = 0xFFB24054;
    let suicideiceexplode_color = 0xFFB24054;
    let explodingjavalin_color = 0xFFFFFF54;
    let explodingjavalinexp_color = 0xFFB24054;
    let lightingtrailingjavalin_color = 0xFFFFFF54;
    let lightjavalintrail_color = 0xFFFFFF54;
    let lightjavalinexplosion_color = 0xFFFFFF54;
    let icejavalin_color = 0xFFFFFF54;
    let icejavalinexplode_color = 0x5151FF54;
    let plaguejavelin2_color = 0xFFFFFF54;
    let plaguejavlinexplode_color = 0xFFFFFF54;
    let advlighttrailingjav_color = 0xFFFFFF54;
    let advlighttrailingjav2_color = 0xFFFFFF54;
    let advlightjavexplode_color = 0xFFFFFF54;
    let sucfireball_color = 0xFFB24054;
    let sucfireballexplode_color = 0xFFB24054;
    let sucfireballtrail_color = 0xFFB24054;
    let sucshockfieldmissile_color = 0xFFFFFF54;
    let sucshockfieldmissileexp_color = 0x00000054;
    let sucshockfield_color = 0xFFB24054;
    let hellfiremissile_color = 0x00000054;
    let hellfireexa_color = 0xFFFFFF54;
    let hellfireexb_color = 0xFFFFFF54;
    let impchargedbolt_color = 0xFFFFFF54;
    let impteleport_color = 0xFFFFFF54;
    let moltenboulder_color = 0xFFB24054;
    let moltenboulderemerge_color = 0xFFB24054;
    let moltenboulderexplode_color = 0xFFB24054;
    let moltenboulderfirepath_color = 0xFFB24054;
    let moltenboulderflyingrocks_color = 0xFFB24054;
    let firestorm_color = 0xFFB24054;
    let firestormmaker_color = 0xFFFFFF54;
    let arcticblast1_color = 0xFFFFFF54;
    let arcticblast2_color = 0xFFFFFF54;
    let erruptioncenter_color = 0xFFFFFF54;
    let erruptioncrack1_color = 0xFFB24054;
    let erruptioncrack2_color = 0xFFB24054;
    let erruptionsmoke1_color = 0xFFFFFF54;
    let erruptionsmoke2_color = 0xFFFFFF54;
    let vinebeastwalk1_color = 0xFFFFFF54;
    let vinebeastwalk2_color = 0xFFFFFF54;
    let vinebeastneutral_color = 0xFFFFFF54;
    let vinebeastattack_color = 0xFFFFFF54;
    let vinebeastdeath_color = 0xFFFFFF54;
    let vines_color = 0xFFFFFF54;
    let vinestrail_color = 0xFFFFFF54;
    let vineswither_color = 0xFFFFFF54;
    let plaguevines_color = 0xFFFFFF54;
    let plaguevinestrail_color = 0xFFFFFF54;
    let plaguevineswither_color = 0xFFFFFF54;
    let twister_color = 0xFFFFFF54;
    let tornado_color = 0xFFFFFF54;
    let volcano_color = 0x00000054;
    let volcanooverlayfire_color = 0xFF404054;
    let volcanodebris2_color = 0x00000054;
    let volcanoexplosion_color = 0x00000054;
    let volcanosmallfire_color = 0x00000054;
    let dragonbreathmissile_color = 0xFFFFFF54;
    let lureprojectile_color = 0xFFFFFF54;
    let lurecenter_color = 0x5151FF54;
    let lurecloud_color = 0x00000054;
    let impmiss1_color = 0xFF404054;
    let impmiss2_color = 0xFF404054;
    let impmiss3_color = 0xFF404054;
    let impmiss4_color = 0xFF404054;
    let impmiss5_color = 0xFF404054;
    let frozenhorrorarcticblast1_color = 0xFFFFFF54;
    let frozenhorrorarcticblast2_color = 0xFFFFFF54;
    let sentrychargedbolt_color = 0xFFFFFF54;
    let sentryspikeinair_color = 0xFFFFFF54;
    let sentryspikeonground_color = 0xFFFFFF54;
    let recyclerdelay_color = 0x00000054;
    let recyclervine_color = 0xFFFFFF54;
    let recyclerfade_color = 0xFFFFFF54;
    let recyclerexplosion_color = 0xFFB24054;
    let deathmauler_color = 0xFFFFFF54;
    let deathmaulertrail_color = 0xFFFFFF54;
    let deathmaulertrailfade_color = 0xFFFFFF54;
    let bladefury1_color = 0x00000054;
    let bladefragment1_color = 0x00000054;
    let bladefury2_color = 0x00000054;
    let bladefragment2_color = 0x00000054;
    let bladefury3_color = 0x00000054;
    let bladefragment3_color = 0x00000054;
    let shockwave_color = 0xFFFFFF54;
    let lightningtalons_color = 0xFFFFFF54;
    let lightningtalonstrail_color = 0xFFFFFF54;
    let phoenixtrail_color = 0xFFB24054;
    let rabiesplague_color = 0x00000054;
    let rabiescontagion_color = 0x00000054;
    let wakeofdestructionmaker_color = 0xFFB24054;
    let wakeofdestruction_color = 0x00000054;
    let deathsentryexplode_color = 0x00000054;
    let tigerfury_color = 0xFFFFFF54;
    let tigerfurytrail_color = 0xFFB24054;
    let tigerfurytrail2_color = 0xFFFFFF54;
    let infernosentry1_color = 0xFFB24054;
    let infernosentry2_color = 0xFFB24054;
    let ancientthrowingaxe_color = 0xFFFFFF54;
    let sentrylightningbolt_color = 0xFFFFFF54;
    let sentrylightninghit_color = 0xFFFFFF54;
    let anyacenter_color = 0x00000054;
    let anyaicicle_color = 0x00000054;
    let anyaiceimpact_color = 0x00000054;
    let anyaicesteam_color = 0x00000054;
    let anyaicemagic_color = 0x00000054;
    let dragontailmissile_color = 0xFFB24054;
    let dragonflight_color = 0xFFFFFF54;
    let dragonflightmaker_color = 0xFFFFFF54;
    let progressiveradiusdamage_color = 0xFFFFFF54;
    let vinebeastwalk1fade_color = 0xFFFFFF54;
    let vinebeastwalk2fade_color = 0xFFFFFF54;
    let vinebeastneutralfade_color = 0xFFFFFF54;
    let vinerecyclerdelay_color = 0x00000054;
    let ancientdeathcenter_color = 0xFF7D7D54;
    let ancientdeathcloud_color = 0xFF7D7D54;
    let lightningchargeup_color = 0xFFFFFF54;
    let chargeupnova_color = 0xFFFFFF54;
    let chainlightningchargeup_color = 0xFFFFFF54;
    let painwormappear_color = 0xFFB24054;
    let baaltauntcontrol_color = 0xFFFFFF54;
    let baaltauntlightning_color = 0xFFFFFF54;
    let baaltauntlightningtrail_color = 0xFFFFFF54;
    let baaltauntpoison_color = 0xFFFFFF54;
    let baalspawnmonsters_color = 0x5151FF54;
    let mindblasthit_color = 0xFFFFFF54;
    let bladeshieldmissile_color = 0xFFFFFF54;
    let bladeshieldattachment_color = 0x00000054;
    let baalinferno_color = 0xFFFFFF54;
    let baalnova_color = 0xFFFFFF54;
    let fistsoffireexplode_color = 0xFFB24054;
    let fistsoffirefirewall_color = 0xFFB24054;
    let clawsofthunderbolt_color = 0xFFFFFF54;
    let clawsofthundernova_color = 0xFFFFFF54;
    let bladesoficeexplode_color = 0x5151FF54;
    let bladesoficecubes_color = 0xFFFFFF54;
    let bladesoficecubesmelt_color = 0xFFFFFF54;
    let royalstrikemeteor_color = 0x00000054;
    let royalstrikemeteorcenter_color = 0xFF404054;
    let royalstrikemeteortail_color = 0x00000054;
    let royalstrikemeteorexplode_color = 0xFFB24054;
    let royalstrikemeteorfire_color = 0xFFB24054;
    let royalstrikechainlightning_color = 0xFFFFFF54;
    let royalstrikechaosice_color = 0x5151FF54;
    let worldstonechip1_color = 0xFFFFFF54;
    let worldstonechip2_color = 0xFFFFFF54;
    let worldstonechip3_color = 0xFFFFFF54;
    let worldstonechip4_color = 0xFFFFFF54;
    let highpriestlightning2_color = 0xFF404054;
    let infernoflame3_color = 0xFFB24054;
    let mindblastcenter_color = 0xFFFFFF54;
    let armageddoncontrol_color = 0x00000054;
    let armageddonrock_color = 0x00000054;
    let armageddontail_color = 0x00000054;
    let armageddonexplosion_color = 0xFFFFFF54;
    let hurricaneswoosh_color = 0x00000054;
    let hurricanecart_color = 0x00000054;
    let hurricanerock_color = 0x00000054;
    let hurricanesack_color = 0x00000054;
    let hurricanetree_color = 0x00000054;
    let hurricanevase_color = 0x00000054;
    let baalcorpseexplodedelay_color = 0xFFB24054;
    let baalcorpseexplodeexpl_color = 0xFFB24054;
    let baalcoldmaker_color = 0x8F8FFF54;
    let baalcoldtrail_color = 0x8F8FFF54;
    let baalspawnmonstersexp_color = 0x5151FF54;
    let impmiss21_color = 0xFF404054;
    let impmiss22_color = 0xFF404054;
    let impmiss23_color = 0xFF404054;
    let impmiss24_color = 0xFF404054;
    let impmiss25_color = 0xFF404054;
    let anyasteam1_color = 0x00000054;
    let anyasteam2_color = 0x00000054;
    let ancientsguide_color = 0x00000054;
    let ancientsmarker_color = 0x00000054;
    let ancientscontrol_color = 0x00000054;
    let overseercontrol_color = 0x00000054;
    let nihlithak1_color = 0x00000054;
    let nihlithak2_color = 0x00000054;
    let nihlithak3_color = 0x00000054;
    let nihlathakcontrol_color = 0x00000054;
    let nihlathakswoosh_color = 0x00000054;
    let nihlathakdebris1_color = 0x00000054;
    let nihlathakdebris2_color = 0x00000054;
    let nihlathakdebris3_color = 0x00000054;
    let nihlathakdebris4_color = 0x00000054;
    let nihlathakglow_color = 0x00000054;
    let baalteleport_color = 0x80404054;
    let baalclonedeath_color = 0x80404054;
    let anyasteamvent_color = 0x00000054;
    let anyasteam_color = 0x00000054;
    let nihlathakhole_color = 0x00000054;
    let nihlathakholelight_color = 0x00000054;
    let volcanofiretrail_color = 0x00000054;
    let nihlathakglow2_color = 0x00000054;
    let nihlathakbonechips_color = 0x00000054;
    let baalcorpseexplodefade_color = 0x00000054;
    let armageddonfire_color = 0xFFB24054;
    let icesparkle_color = 0xFFB24054;
    let baalfxcontrol_color = 0xFFB24054;
    let baalfxspirit1_color = 0x00000054;
    let baalfxspirit2_color = 0x00000054;
    let baalfxspirit3_color = 0x00000054;
    let baalfxspirit4_color = 0x00000054;
    let baalfxspirit5_color = 0x00000054;
    let baalfxbaalheadappear_color = 0x00000054;
    let baalfxbaalhead1_color = 0xFFB24054;
    let baalfxbaalhead2_color = 0xFFB24054;
    let baalfxbaalhead3_color = 0xFFB24054;
    let baalfxtyrealdebris1_color = 0x00000054;
    let baalfxtyrealdebris2_color = 0x00000054;
    let baalfxtyrealdebris3_color = 0x00000054;
    let baalfxtyrealdebrisbreak_color = 0x00000054;
    let worldstoneshake_color = 0x00000054;
    let blessedhammerex_color = 0xDEDEFF54;
    let sentrylightningbolt2_color = 0xFFFFFF54;
    let sentrylightninghit2_color = 0xFFFFFF54;
    let lightningtowernova_color = 0xFFFFFF54;
    let skbowarrow6_color = 0xFFB24054;
    let skbowarrow7_color = 0x5151FF54;
    let skbowarrow8_color = 0xFFB24054;
    let bighead6_color = 0x8F8FFF54;
    let viperpoisjav_color = 0xFFFFFF54;
    let viperpoisjavcloud_color = 0x80FF8054;
    let viperfire_color = 0xFFFFFF54;
    let viperfirecloud_color = 0xFFB24054;
    let viperbonespear_color = 0xFFFFFF54;
    let countessfirewallmaker_color = 0xFFB24054;
    let baaltauntlightningcontrol_color = 0x00000054;
    let baaltauntpoisoncontrol_color = 0x00000054;
    let explodingarrowexp2_color = 0x00000054;
    let freezingarrowexp3_color = 0x00000054;
    let pantherjav5_color = 0xFFFFFF54;
    let spike6_color = 0xFFFFFF54;
    let crarrow6_color = 0xFFFFFF54;
    let skmagepois_color = 0xFFFFFF54;
    let skmagecold_color = 0xFFFFFF54;
    let skmagefire_color = 0xFFFFFF54;
    let skmageltng_color = 0xFFFFFF54;
    let succubusmiss_color = 0xFF404054;
    let willowisplightningbolt2_color = 0xB2B2FF54;
    let mummyex_color = 0xFFFFFF54;
    let goospitex_color = 0xFFFFFF54;
    let impmissex_color = 0xFF404054;
    let diablogeddoncontrol_color = 0x00000054;
    let diablogeddonrock_color = 0x00000054;
    let diablogeddontail_color = 0x00000054;
    let diablogeddonexplosion_color = 0xFFFFFF54;
    let diablogeddonfire_color = 0xFFB24054;
    let megademoninferno_color = 0xFFB24054;
    let trapfirebolt_color = 0xFFB24054;
    let trappoisonjavcloud_color = 0x80FF8054;
    let trapnova_color = 0xFFFFFF54;
    let mephfrostnova_color = 0x5151FF54;
    let mephlight_color = 0xFFFFFF54;
    let vampiremeteorfire_color = 0xFFB24054;
    let strafearrow_color = 0xFFFFFF54;
    let strafebolt_color = 0xFFFFFF54;
    let recklessattacksmissile_color = 0xFFFFFF54;
    let leapattack_color = 0xFFFFFF54;
    let sigillethargymedium_color = 0xB040FF54;
    let sigilrancormedium_color = 0xB040FF54;
    let sigildeathmedium_color = 0xB040FF54;
    let ringoffire_color = 0xFFB24054;
    let flamewave_color = 0xFFB24054;
    let mindwallmaker_color = 0xB040FF54;
    let bladewarpexplode_color = 0xB040FF54;
    let miasmaboltdot_color = 0xB040FF54;
    let abyssalshattershards_color = 0xB040FF54;
    let flamewavefire_color = 0xFFB24054;
    let miasmachainscloud_color = 0xB040FF54;
    let miasmabolt_color = 0xB040FF54;
    let miasmapuff_color = 0xB040FF54;
    let sigillethargysmall_color = 0xB040FF54;
    let sigilrancorsmall_color = 0xB040FF54;
    let sigildeathsmall_color = 0xB040FF54;
    let sigillethargylarge_color = 0xB040FF54;
    let sigilrancorlarge_color = 0xB040FF54;
    let sigildeathlarge_color = 0xB040FF54;
    let miasmaboltcloud_color = 0xB040FF54;
    let echoingstrike_color = 0xB040FF54;
    let bladewarp_color = 0xB040FF54;
    let flamewaveunveiling_color = 0xFFB24054;
    let abysscenter_color = 0xB040FF54;
    let engorge_color = 0xB040FF54;
    let miasmachainsmaker_color = 0xB040FF54;
    let miasmachains_color = 0xB040FF54;
    let miasmachainscloudmaker_color = 0xB040FF54;
    let abyss_color = 0xB040FF54;
    let abyssexplode_color = 0xB040FF54;
    let flamewavelingerfire_color = 0xFFB24054;
    let hexpurgeexplode_color = 0xB040FF54;
    let engorgecorpseeffect_color = 0xB040FF54;
    let bloodboilexplode_color = 0xB040FF54;
    let echoingstrikedetonate_color = 0xB040FF54;
    let eldritchblastnova_color = 0xB040FF54;
    let apocalypse_color = 0xB040FF54;
    let binddemonchannelmaker_color = 0xB040FF54;
    let binddemonchannel_color = 0xB040FF54;
    let coldfissurecenter_color = 0x5151FF54;
    let icecrack1_color = 0x5151FF54;
    let icecrack2_color = 0x5151FF54;
    let icevapor1_color = 0x5151FF54;
    let icevapor2_color = 0x5151FF54;
    let ringoffireexplode_color = 0xFFB24054;
    let colossalthrowingaxe_color = 0xFFFFFF54;
    let firetwister_color = 0xFFB24054;
    let taintedfirebolt_color = 0xFFB24054;
    let taintedfireball_color = 0xFFB24054;
    let colossalchargedbolt_color = 0xFFFFFF54;
    let unknown_color = 0x00000000;

    match txt_file_no {
        Missile::Arrow => arrow_color,
        Missile::Javelin => javelin_color,
        Missile::Bighead1 => bighead1_color,
        Missile::Bighead2 => bighead2_color,
        Missile::Bighead3 => bighead3_color,
        Missile::Bighead4 => bighead4_color,
        Missile::Bighead5 => bighead5_color,
        Missile::Spike1 => spike1_color,
        Missile::Spike2 => spike2_color,
        Missile::Spike3 => spike3_color,
        Missile::Spike4 => spike4_color,
        Missile::Spike5 => spike5_color,
        Missile::Firearrow => firearrow_color,
        Missile::CrArrow1 => crarrow1_color,
        Missile::CrArrow2 => crarrow2_color,
        Missile::CrArrow3 => crarrow3_color,
        Missile::CrArrow4 => crarrow4_color,
        Missile::CrArrow5 => crarrow5_color,
        Missile::Blood1 => blood1_color,
        Missile::Blood2 => blood2_color,
        Missile::Bigblood1 => bigblood1_color,
        Missile::Bigblood2 => bigblood2_color,
        Missile::Shafire1 => shafire1_color,
        Missile::Shafire2 => shafire2_color,
        Missile::Shafire3 => shafire3_color,
        Missile::Shafire4 => shafire4_color,
        Missile::Shafire5 => shafire5_color,
        Missile::MagicArrow => magicarrow_color,
        Missile::IceArrow => icearrow_color,
        Missile::FireExplode => fireexplode_color,
        Missile::IceExplode => iceexplode_color,
        Missile::Bolt => bolt_color,
        Missile::AndarielsSpray => andarielspray_color,
        Missile::BigheadExp => bigheadexp_color,
        Missile::ShamanExp => shamanexp_color,
        Missile::ThrowAxe => throwaxe_color,
        Missile::ThrowKnife => throwknife_color,
        Missile::Glaive => glaive_color,
        Missile::PoisonJav => poisonjav_color,
        Missile::PoisonJavCloud => poisonjavcloud_color,
        Missile::ColdArrow => coldarrow_color,
        Missile::ExplodingArrow => explodingarrow_color,
        Missile::ExplodingArrowExp => explodingarrowexp_color,
        Missile::PlagueJavelin => plaguejavelin_color,
        Missile::OilPotion => oilpotion_color,
        Missile::ExplosivePotion => explosivepotion_color,
        Missile::FulminatingPotion => fulminatingpotion_color,
        Missile::RancidGasePotion => rancidgasepotion_color,
        Missile::ChokingGasPoition => chokinggaspoition_color,
        Missile::StranglingGasPotion => stranglinggaspotion_color,
        Missile::Notused50 => notused50_color,
        Missile::ExplosivePotionExp => explosivepotionexp_color,
        Missile::ExplosivePotionDebris1 => explosivepotiondebris1_color,
        Missile::ExplosivePotionDebris2 => explosivepotiondebris2_color,
        Missile::ExplosivePotionDebris3 => explosivepotiondebris3_color,
        Missile::HolyBolt => holybolt_color,
        Missile::ChargedBolt => chargedbolt_color,
        Missile::SanctuaryBolt => sanctuarybolt_color,
        Missile::FireBolt => firebolt_color,
        Missile::IceBolt => icebolt_color,
        Missile::InfernoFlame1 => infernoflame1_color,
        Missile::InfernoFlame2 => infernoflame2_color,
        Missile::FireBall => fireball_color,
        Missile::Mummy1 => mummy1_color,
        Missile::Mummy2 => mummy2_color,
        Missile::Mummy3 => mummy3_color,
        Missile::Mummy4 => mummy4_color,
        Missile::Blaze => blaze_color,
        Missile::FireWallMaker => firewallmaker_color,
        Missile::FireWall => firewall_color,
        Missile::Goospit1 => goospit1_color,
        Missile::Goospit2 => goospit2_color,
        Missile::Goospit3 => goospit3_color,
        Missile::Goospit4 => goospit4_color,
        Missile::Goospit5 => goospit5_color,
        Missile::Goosplat => goosplat_color,
        Missile::Sand => sand_color,
        Missile::SandPile => sandpile_color,
        Missile::UnholyBolt1 => unholybolt1_color,
        Missile::UnholyBolt2 => unholybolt2_color,
        Missile::UnholyBolt3 => unholybolt3_color,
        Missile::UnholyBolt4 => unholybolt4_color,
        Missile::SanctuaryCenter => sanctuarycenter_color,
        Missile::FireExplosion => fireexplosion_color,
        Missile::StuckArrow => stuckarrow_color,
        Missile::Footprint => footprint_color,
        Missile::ImmolationArrow => immolationarrow_color,
        Missile::GuidedArrow => guidedarrow_color,
        Missile::FreezingArrow => freezingarrow_color,
        Missile::FreezingArrowExp1 => freezingarrowexp1_color,
        Missile::FreezingArrowExp2 => freezingarrowexp2_color,
        Missile::Nova => nova_color,
        Missile::IceBlast => iceblast_color,
        Missile::BlessedHammer => blessedhammer_color,
        Missile::ChainLightning => chainlightning_color,
        Missile::FistOfAres => fistofares_color,
        Missile::ChillBlood => chillblood_color,
        Missile::GlacialSpike => glacialspike_color,
        Missile::Teleport => teleport_color,
        Missile::LightningBolt => lightningbolt_color,
        Missile::LightningHit => lightninghit_color,
        Missile::Meteor => meteor_color,
        Missile::MeteorCenter => meteorcenter_color,
        Missile::MeteorTail => meteortail_color,
        Missile::MeteorExplode => meteorexplode_color,
        Missile::FireSmall => firesmall_color,
        Missile::FireMedium => firemedium_color,
        Missile::MonBlizCenter => monblizcenter_color,
        Missile::MonBliz1 => monbliz1_color,
        Missile::MonBliz2 => monbliz2_color,
        Missile::MonBliz3 => monbliz3_color,
        Missile::MonBliz4 => monbliz4_color,
        Missile::MonBlizExplode1 => monblizexplode1_color,
        Missile::MonBlizExplode2 => monblizexplode2_color,
        Missile::MonBlizExplode3 => monblizexplode3_color,
        Missile::Teeth => teeth_color,
        Missile::CorpseExplosion => corpseexplosion_color,
        Missile::PoisonCorpseExplosion => poisoncorpseexplosion_color,
        Missile::MonsterCorpseExplode => monstercorpseexplode_color,
        Missile::PoisonNova => poisonnova_color,
        Missile::FrostNova => frostnova_color,
        Missile::Rogue1 => rogue1_color,
        Missile::Rogue2 => rogue2_color,
        Missile::Rogue3 => rogue3_color,
        Missile::BatLightningBolt => batlightningbolt_color,
        Missile::BatLightningTrail => batlightningtrail_color,
        Missile::SkMage1 => skmage1_color,
        Missile::SkMage2 => skmage2_color,
        Missile::SkMage3 => skmage3_color,
        Missile::SkMage4 => skmage4_color,
        Missile::VampireFireball => vampirefireball_color,
        Missile::VampireFirewallMaker => vampirefirewallmaker_color,
        Missile::VampireFirewall => vampirefirewall_color,
        Missile::VampireMeteor => vampiremeteor_color,
        Missile::VampireMeteorCenter => vampiremeteorcenter_color,
        Missile::VampireMeteorExp => vampiremeteorexp_color,
        Missile::Raven1 => raven1_color,

        Missile::AmphibianGoo1 => amphibiangoo1_color,
        Missile::AmphibianGoo2 => amphibiangoo2_color,
        Missile::TentacleGoo => tentaclegoo_color,
        Missile::AmphibianExplode => amphibianexplode_color,
        Missile::PoisonPuff => poisonpuff_color,
        Missile::CurseEffectRed => curseeffectred_color,
        Missile::SpiderGooLay => spidergoolay_color,
        Missile::FetishInferno1 => fetishinferno1_color,
        Missile::FetishInferno2 => fetishinferno2_color,
        Missile::SpiderGoo => spidergoo_color,
        Missile::CurseCast => cursecast_color,
        Missile::Howl => howl_color,
        Missile::Shout => shout_color,
        Missile::Dust => dust_color,
        Missile::RedlightMissile => redlightmissile_color,
        Missile::GreenlightMissile => greenlightmissile_color,
        Missile::BluelightMissile => bluelightmissile_color,
        Missile::WhitelightMissile => whitelightmissile_color,
        Missile::CorpsePoisonCloud => corpsepoisoncloud_color,
        Missile::ChillBloodCloud => chillbloodcloud_color,
        Missile::ChillBloodPuff => chillbloodpuff_color,
        Missile::BlizzardCenter => blizzardcenter_color,
        Missile::Blizzard1 => blizzard1_color,
        Missile::Blizzard2 => blizzard2_color,
        Missile::Blizzard3 => blizzard3_color,
        Missile::Blizzard4 => blizzard4_color,
        Missile::BlizzardExplode1 => blizzardexplode1_color,
        Missile::BlizzardExplode2 => blizzardexplode2_color,
        Missile::BlizzardExplode3 => blizzardexplode3_color,
        Missile::ThunderStorm1 => thunderstorm1_color,
        Missile::ThunderStorm2 => thunderstorm2_color,
        Missile::ThunderStorm3 => thunderstorm3_color,
        Missile::ThunderStorm4 => thunderstorm4_color,
        Missile::MonsterLight => monsterlight_color,
        Missile::PoisonBall => poisonball_color,
        Missile::DiabLight => diablight_color,
        Missile::Redemption => redemption_color,
        Missile::RedemptionFail => redemptionfail_color,
        Missile::HandOfGod => handofgod_color,
        Missile::DiabFire => diabfire_color,
        Missile::FingerMageSpider => fingermagespider_color,
        Missile::Electric => electric_color,
        Missile::DiabThrowAxe => diabthrowaxe_color,
        Missile::DiabWallMaker => diabwallmaker_color,
        Missile::DiabWall => diabwall_color,
        Missile::CurseAmplifyDamage => curseamplifydamage_color,
        Missile::CurseDimVision => cursedimvision_color,
        Missile::CurseWeaken => curseweaken_color,
        Missile::CurseIronMaiden => curseironmaiden_color,
        Missile::CurseTerror => curseterror_color,
        Missile::CurseAttract => curseattract_color,
        Missile::CurseReverseVampire => cursereversevampire_color,
        Missile::CurseConfuse => curseconfuse_color,
        Missile::CurseDecrepify => cursedecrepify_color,
        Missile::CurseLowerResist => curselowerresist_color,
        Missile::CurseCenter => cursecenter_color,
        Missile::BoneSpear => bonespear_color,
        Missile::BoneSpirit => bonespirit_color,
        Missile::ColdUnique => coldunique_color,
        Missile::LightUnique => lightunique_color,
        Missile::SkBowArrow1 => skbowarrow1_color,
        Missile::SkBowArrow2 => skbowarrow2_color,
        Missile::SkBowArrow3 => skbowarrow3_color,
        Missile::SkBowArrow4 => skbowarrow4_color,
        Missile::SkBowArrow5 => skbowarrow5_color,
        Missile::Nova1 => nova1_color,
        Missile::Nova2 => nova2_color,
        Missile::AndyPoisonBolt => andypoisonbolt_color,
        Missile::TeethExplode => teethexplode_color,
        Missile::LightningJavelin => lightningjavelin_color,
        Missile::LightningFury => lightningfury_color,
        Missile::BoneWallMaker => bonewallmaker_color,
        Missile::Necromage1 => necromage1_color,
        Missile::Necromage2 => necromage2_color,
        Missile::Necromage3 => necromage3_color,
        Missile::Necromage4 => necromage4_color,
        Missile::Sparkle => sparkle_color,
        Missile::MultipleShotArrow => multipleshotarrow_color,
        Missile::MultipleShotBolt => multipleshotbolt_color,
        Missile::ChargedStrikeBolt => chargedstrikebolt_color,
        Missile::BoneSpearExplode => bonespearexplode_color,
        Missile::PoisonExplosionCloud => poisonexplosioncloud_color,
        Missile::BoneCast => bonecast_color,
        Missile::BattleCry => battlecry_color,
        Missile::PrimePoisonCloud => primepoisoncloud_color,
        Missile::PlagueJavCloud => plaguejavcloud_color,
        Missile::RancidGasCloud => rancidgascloud_color,
        Missile::ChokingGasCloud => chokinggascloud_color,
        Missile::StranglingGasCloud => stranglinggascloud_color,
        Missile::BugLightning => buglightning_color,
        Missile::PantherJav1 => pantherjav1_color,
        Missile::PantherJav2 => pantherjav2_color,
        Missile::PantherJav3 => pantherjav3_color,
        Missile::PantherJav4 => pantherjav4_color,
        Missile::ImmolationFire => immolationfire_color,
        Missile::FuryLightning => furylightning_color,
        Missile::LightningStrike => lightningstrike_color,
        Missile::FistOfTheHeavensDelay => fistoftheheavensdelay_color,
        Missile::FistOfTheHeavensBolt => fistoftheheavensbolt_color,
        Missile::WarCry => warcry_color,
        Missile::BattleCommand => battlecommand_color,
        Missile::BattleOrders => battleorders_color,
        Missile::PantherPotOrange => pantherpotorange_color,
        Missile::PantherPotGreen => pantherpotgreen_color,
        Missile::MeteorFire => meteorfire_color,
        Missile::TrapSpikeRight => trapspikeright_color,
        Missile::TrapSpikeLeft => trapspikeleft_color,
        Missile::TrapCursedSkullRight => trapcursedskullright_color,
        Missile::TrapCursedSkullLeft => trapcursedskullleft_color,
        Missile::TrapPoisonBallRight => trappoisonballright_color,
        Missile::TrapPoisonBallLeft => trappoisonballleft_color,
        Missile::Hydra => hydra_color,
        Missile::BoneSpearTrail => bonespeartrail_color,
        Missile::GrimWardSmallStart => grimwardsmallstart_color,
        Missile::GrimWardSmall => grimwardsmall_color,
        Missile::GrimWardSmallStop => grimwardsmallstop_color,
        Missile::GrimWardMediumStart => grimwardmediumstart_color,
        Missile::GrimWardMedium => grimwardmedium_color,
        Missile::GrimWardMediumStop => grimwardmediumstop_color,
        Missile::GrimWardLargeStart => grimwardlargestart_color,
        Missile::GrimWardLarge => grimwardlarge_color,
        Missile::GrimWardLargeStop => grimwardlargestop_color,
        Missile::ZakarumLight => zakarumlight_color,
        Missile::GrimWardScare => grimwardscare_color,
        Missile::FrozenOrb => frozenorb_color,
        Missile::FrozenOrbBolt => frozenorbbolt_color,
        Missile::FrozenOrbNova => frozenorbnova_color,
        Missile::FrozenOrbExplode => frozenorbexplode_color,
        Missile::ChillingArmorBolt => chillingarmorbolt_color,
        Missile::FireExplosion2 => fireexplosion2_color,
        Missile::Blowgun => blowgun_color,
        Missile::ChainLightning2 => chainlightning2_color,
        Missile::ReviveSmall => revivesmall_color,
        Missile::ReviveMedium => revivemedium_color,
        Missile::ReviveLarge => revivelarge_color,
        Missile::MonGlacialSpike => monglacialspike_color,
        Missile::IceBreakSmall => icebreaksmall_color,
        Missile::IceBreakMedium => icebreakmedium_color,
        Missile::IceBreakLarge => icebreaklarge_color,
        Missile::IceBreakSmoke => icebreaksmoke_color,
        Missile::MephistoFirehead => mephisto_color,
        Missile::Whilrwind => whilrwind_color,
        Missile::ArcaneLightningBolt => arcanelightningbolt_color,
        Missile::FrogFire => frogfire_color,
        Missile::FrogCold => frogcold_color,
        Missile::FrogPois => frogpois_color,
        Missile::DesertFireball => desertfireball_color,
        Missile::BrDeathControl => brdeathcontrol_color,
        Missile::BrDeathLightningBolt => brdeathlightningbolt_color,
        Missile::BrDeathLightningHit => brdeathlightninghit_color,
        Missile::DenOfEvilLight => denofevillight_color,
        Missile::CairnStones => cairnstones_color,
        Missile::CairnStonesSky => cairnstonessky_color,
        Missile::CairnStonesGround => cairnstonesground_color,
        Missile::TowerMist => towermist_color,
        Missile::TowerMistTrail => towermisttrail_color,
        Missile::BrDeathSmokes1 => brdeathsmokes1_color,
        Missile::BrDeathSmokeNu => brdeathsmokenu_color,
        Missile::BrDeathSmokeDt => brdeathsmokedt_color,
        Missile::BrDeathSpirits1 => brdeathspirits1_color,
        Missile::BrDeathSpiritNu => brdeathspiritnu_color,
        Missile::BrDeathSpiritDt => brdeathspiritdt_color,
        Missile::MephistoDeathControl => mephistodeathcontrol_color,
        Missile::MephistoFirewallMaker => mephistofirewallmaker_color,
        Missile::MephistoFirewall => mephistofirewall_color,
        Missile::MephistoFlyingRocksBig => mephistoflyingrocksbig_color,
        Missile::MephistoExplosionBig => mephistoexplosionbig_color,
        Missile::MephistoFlyingRocksSmall => mephistoflyingrockssmall_color,
        Missile::MephistoExplosionSmall => mephistoexplosionsmall_color,
        Missile::MephistoDoNotDraw => mephistodonotdraw_color,
        Missile::AndyControl0 => andycontrol0_color,
        Missile::AndyFirewallMaker => andyfirewallmaker_color,
        Missile::AndyFirewall => andyfirewall_color,
        Missile::AndyColumnFireBase => andycolumnfirebase_color,
        Missile::AndyColumnFire => andycolumnfire_color,
        Missile::AndyFallingDebris1 => andyfallingdebris1_color,
        Missile::AndyFallingDebris2 => andyfallingdebris2_color,
        Missile::AndyFallingDebris3 => andyfallingdebris3_color,
        Missile::AndyDebrisExplosion1 => andydebrisexplosion1_color,
        Missile::AndyDebrisExplosion2 => andydebrisexplosion2_color,
        Missile::AndyDebrisExplosion3 => andydebrisexplosion3_color,
        Missile::AndyDebrisExplosion4 => andydebrisexplosion4_color,
        Missile::AndyDebrisExplosion5 => andydebrisexplosion5_color,
        Missile::WillowisplightningBolt => willowisplightningbolt_color,
        Missile::QueenPoisonCloud => queenpoisoncloud_color,
        Missile::Dirt => dirt_color,
        Missile::DirtPile => dirtpile_color,
        Missile::UndeadMissile1 => undeadmissile1_color,
        Missile::UndeadMissile2 => undeadmissile2_color,
        Missile::UndeadMissile3 => undeadmissile3_color,
        Missile::UndeadMissile4 => undeadmissile4_color,
        Missile::BoneSpiritExplode => bonespiritexplode_color,
        Missile::DopplezonExplode => dopplezonexplode_color,
        Missile::MonBoneSpirit => monbonespirit_color,
        Missile::TowerMistFade => towermistfade_color,
        Missile::CountessFirewall => countessfirewall_color,
        Missile::TowerChestSpawner => towerchestspawner_color,
        Missile::HellMeteorLaunch1 => hellmeteorlaunch1_color,
        Missile::HellMeteorLaunch2 => hellmeteorlaunch2_color,
        Missile::HellMeteorUp => hellmeteorup_color,
        Missile::HellMeteorDown => hellmeteordown_color,
        Missile::HellMeteorBall => hellmeteorball_color,
        Missile::HoradricStaff => horadricstaff_color,
        Missile::HoradricLightning => horadriclightning_color,
        Missile::HoradricLight => horadriclight_color,
        Missile::RegurgitatorCorpse => regurgitatorcorpse_color,
        Missile::RegurgitatorCorpseExpl => regurgitatorcorpseexpl_color,
        Missile::HighPriestLightning => highpriestlightning_color,
        Missile::IceBreakSmallMelt => icebreaksmallmelt_color,
        Missile::IceBreakLargeMelt => icebreaklargemelt_color,
        Missile::LeapKnockback => leapknockback_color,
        Missile::RadamentDeath => radamentdeath_color,
        Missile::RadamentHandOfGod => radamenthandofgod_color,
        Missile::RadamentHolyBolt => radamentholybolt_color,
        Missile::TaintedSunControl => taintedsuncontrol_color,
        Missile::TaintedSunFlash => taintedsunflash_color,
        Missile::TaintedSunBall => taintedsunball_color,
        Missile::QueenDeathCenter => queendeathcenter_color,
        Missile::QueenDeathGlob => queendeathglob_color,
        Missile::QueenDeathSplat1 => queendeathsplat1_color,
        Missile::QueenDeathSplat2 => queendeathsplat2_color,
        Missile::HealingBolt => healingbolt_color,
        Missile::MephistoHoleDelay => mephistoholedelay_color,
        Missile::MephistoHoleBirth => mephistoholebirth_color,
        Missile::MephistoHoleNeutral => mephistoholeneutral_color,
        Missile::MephistoHoleDeath => mephistoholedeath_color,
        Missile::MephistoHoleDead => mephistoholedead_color,
        Missile::DurielDeathControl => durieldeathcontrol_color,
        Missile::DurielDeathRock => durieldeathrock_color,
        Missile::DurielDeathDebris => durieldeathdebris_color,
        Missile::DurielDeathSmoke => durieldeathsmoke_color,
        Missile::MephistoExplosion => mephistoexplosion_color,
        Missile::OrbMist => orbmist_color,
        Missile::OrbMistTrail => orbmisttrail_color,
        Missile::OrbMistFade => orbmistfade_color,
        Missile::Pilum => pilum_color,
        Missile::DiabloAppears => diabloappears_color,
        Missile::HfControl => hfcontrol_color,
        Missile::HfFragment1 => hffragment1_color,
        Missile::HfFragment2 => hffragment2_color,
        Missile::HfFragment3 => hffragment3_color,
        Missile::HfSpirit1 => hfspirit1_color,
        Missile::HfReserved3 => hfreserved3_color,
        Missile::IzualControl => izualcontrol_color,
        Missile::IzualMistLoop => izualmistloop_color,
        Missile::IzualMistFade => izualmistfade_color,
        Missile::IzualLightning => izuallightning_color,
        Missile::IzualLightningTrail => izuallightningtrail_color,
        Missile::CairnStonesBolt => cairnstonesbolt_color,
        Missile::BombInAir => bombinair_color,
        Missile::BombOnGround => bombonground_color,
        Missile::BombExplosion => bombexplosion_color,
        Missile::ShockFieldInAir => shockfieldinair_color,
        Missile::ShockFieldOnGround => shockfieldonground_color,
        Missile::ThrowingStar => throwingstar_color,
        Missile::AcidSpray => acidspray_color,
        Missile::BladeCreeper => bladecreeper_color,
        Missile::Distraction => distraction_color,
        Missile::DistractionFog => distractionfog_color,
        Missile::DistractionPuff => distractionpuff_color,
        Missile::DistractionStart => distractionstart_color,
        Missile::DistractionEnd => distractionend_color,
        Missile::ImpInfernoFlame1 => impinfernoflame1_color,
        Missile::ImpInfernoFlame2 => impinfernoflame2_color,
        Missile::BaalLightningBolt => baallightningbolt_color,
        Missile::BaalLightningTrail => baallightningtrail_color,
        Missile::BaalLightningBolt2 => baallightningbolt2_color,
        Missile::BaalLightningTrail2 => baallightningtrail2_color,
        Missile::ImpFireball => impfireball_color,
        Missile::ImpFireballExplode => impfireballexplode_color,
        Missile::CatapultChargedBallOn => catapultchargedballon_color,
        Missile::CatapultChargedBall => catapultchargedball_color,
        Missile::CatapultChargedBallBolt => catapultchargedballbolt_color,
        Missile::ImpSpawnMonsters => impspawnmonsters_color,
        Missile::CatapultSpikeBallOn => catapultspikeballon_color,
        Missile::CatapultSpikeBall => catapultspikeball_color,
        Missile::CatapultSpikeInAir => catapultspikeinair_color,
        Missile::CatapultSpikeOnGround => catapultspikeonground_color,
        Missile::CatapultSpikeExplosion => catapultspikeexplosion_color,
        Missile::CatapultColdBallOn => catapultcoldballon_color,
        Missile::CatapultColdBall => catapultcoldball_color,
        Missile::CatapultColdExplosion => catapultcoldexplosion_color,
        Missile::CatapultPlagueBallOn => catapultplagueballon_color,
        Missile::CatapultPlagueBall => catapultplagueball_color,
        Missile::CatapultPlagueCloud => catapultplaguecloud_color,
        Missile::CatapultMeteorBallOn => catapultmeteorballon_color,
        Missile::CatapultMeteorBall => catapultmeteorball_color,
        Missile::CatapultMeteorFire => catapultmeteorfire_color,
        Missile::TowerDeath => towerdeath_color,
        Missile::HealingVortex => healingvortex_color,
        Missile::SuicideCorpseExplode => suicidecorpseexplode_color,
        Missile::SuicideFireExplode => suicidefireexplode_color,
        Missile::SuicideIceExplode => suicideiceexplode_color,
        Missile::ExplodingJavalin => explodingjavalin_color,
        Missile::ExplodingJavalinExp => explodingjavalinexp_color,
        Missile::LightingTrailingJavalin => lightingtrailingjavalin_color,
        Missile::LightJavalinTrail => lightjavalintrail_color,
        Missile::LightJavalinExplosion => lightjavalinexplosion_color,
        Missile::IceJavalin => icejavalin_color,
        Missile::IceJavalinExplode => icejavalinexplode_color,
        Missile::PlagueJavelin2 => plaguejavelin2_color,
        Missile::PlagueJavlinExplode => plaguejavlinexplode_color,
        Missile::AdvLightTrailingJav => advlighttrailingjav_color,
        Missile::AdvLightTrailingJav2 => advlighttrailingjav2_color,
        Missile::AdvLightJavExplode => advlightjavexplode_color,
        Missile::SucFireball => sucfireball_color,
        Missile::SucFireballExplode => sucfireballexplode_color,
        Missile::SucFireballTrail => sucfireballtrail_color,
        Missile::SucShockFieldMissile => sucshockfieldmissile_color,
        Missile::SucShockFieldMissileExp => sucshockfieldmissileexp_color,
        Missile::SucShockField => sucshockfield_color,
        Missile::HellfireMissile => hellfiremissile_color,
        Missile::HellfireExa => hellfireexa_color,
        Missile::HellfireExb => hellfireexb_color,
        Missile::ImpChargedBolt => impchargedbolt_color,
        Missile::ImpTeleport => impteleport_color,
        Missile::MoltenBoulder => moltenboulder_color,
        Missile::MoltenBoulderEmerge => moltenboulderemerge_color,
        Missile::MoltenBoulderExplode => moltenboulderexplode_color,
        Missile::MoltenBoulderFirePath => moltenboulderfirepath_color,
        Missile::MoltenBoulderFlyingRocks => moltenboulderflyingrocks_color,
        Missile::Firestorm => firestorm_color,
        Missile::FirestormMaker => firestormmaker_color,
        Missile::ArcticBlast1 => arcticblast1_color,
        Missile::ArcticBlast2 => arcticblast2_color,
        Missile::ErruptionCenter => erruptioncenter_color,
        Missile::ErruptionCrack1 => erruptioncrack1_color,
        Missile::ErruptionCrack2 => erruptioncrack2_color,
        Missile::ErruptionSmoke1 => erruptionsmoke1_color,
        Missile::ErruptionSmoke2 => erruptionsmoke2_color,
        Missile::VineBeastWalk1 => vinebeastwalk1_color,
        Missile::VineBeastWalk2 => vinebeastwalk2_color,
        Missile::VineBeastNeutral => vinebeastneutral_color,
        Missile::VineBeastAttack => vinebeastattack_color,
        Missile::VineBeastDeath => vinebeastdeath_color,
        Missile::Vines => vines_color,
        Missile::VinesTrail => vinestrail_color,
        Missile::VinesWither => vineswither_color,
        Missile::PlagueVines => plaguevines_color,
        Missile::PlagueVinesTrail => plaguevinestrail_color,
        Missile::PlagueVinesWither => plaguevineswither_color,
        Missile::Twister => twister_color,
        Missile::Tornado => tornado_color,
        Missile::Volcano => volcano_color,
        Missile::VolcanoOverlay => volcanooverlayfire_color,
        Missile::Fire => volcanooverlayfire_color,
        Missile::VolcanoDebris2 => volcanodebris2_color,
        Missile::VolcanoExplosion => volcanoexplosion_color,
        Missile::VolcanoSmallFire => volcanosmallfire_color,
        Missile::DragonBreathMissile => dragonbreathmissile_color,
        Missile::LureProjectile => lureprojectile_color,
        Missile::LureCenter => lurecenter_color,
        Missile::LureCloud => lurecloud_color,
        Missile::ImpMiss1 => impmiss1_color,
        Missile::ImpMiss2 => impmiss2_color,
        Missile::ImpMiss3 => impmiss3_color,
        Missile::ImpMiss4 => impmiss4_color,
        Missile::ImpMiss5 => impmiss5_color,
        Missile::FrozenHorrorArcticBlast1 => frozenhorrorarcticblast1_color,
        Missile::FrozenHorrorArcticBlast2 => frozenhorrorarcticblast2_color,
        Missile::SentryChargedBolt => sentrychargedbolt_color,
        Missile::SentrySpikeInAir => sentryspikeinair_color,
        Missile::SentrySpikeOnGround => sentryspikeonground_color,
        Missile::RecyclerDelay => recyclerdelay_color,
        Missile::RecyclerVine => recyclervine_color,
        Missile::RecyclerFade => recyclerfade_color,
        Missile::RecyclerExplosion => recyclerexplosion_color,
        Missile::DeathMauler => deathmauler_color,
        Missile::DeathMaulerTrail => deathmaulertrail_color,
        Missile::DeathMaulerTrailFade => deathmaulertrailfade_color,
        Missile::BladeFury1 => bladefury1_color,
        Missile::BladeFragment1 => bladefragment1_color,
        Missile::BladeFury2 => bladefury2_color,
        Missile::BladeFragment2 => bladefragment2_color,
        Missile::BladeFury3 => bladefury3_color,
        Missile::BladeFragment3 => bladefragment3_color,
        Missile::ShockWave => shockwave_color,
        Missile::LightningTalons => lightningtalons_color,
        Missile::LightningTalonsTrail => lightningtalonstrail_color,
        Missile::PhoenixTrail => phoenixtrail_color,
        Missile::RabiesPlague => rabiesplague_color,
        Missile::RabiesContagion => rabiescontagion_color,
        Missile::WakeOfDestructionMaker => wakeofdestructionmaker_color,
        Missile::WakeOfDestructionDeathSentryExplode => deathsentryexplode_color,
        Missile::TigerFury => tigerfury_color,
        Missile::TigerFuryTrail => tigerfurytrail_color,
        Missile::TigerFuryTrail2 => tigerfurytrail2_color,
        Missile::InfernoSentry1 => infernosentry1_color,
        Missile::InfernoSentry2 => infernosentry2_color,
        Missile::AncientThrowingAxe => ancientthrowingaxe_color,
        Missile::SentryLightningBolt => sentrylightningbolt_color,
        Missile::SentryLightningHit => sentrylightninghit_color,
        Missile::AnyaCenter => anyacenter_color,
        Missile::AnyaIcicle => anyaicicle_color,
        Missile::AnyaIceImpact => anyaiceimpact_color,
        Missile::AnyaIceSteam => anyaicesteam_color,
        Missile::AnyaIceMagic => anyaicemagic_color,
        Missile::DragonTailMissile => dragontailmissile_color,
        Missile::DragonFlight => dragonflight_color,
        Missile::DragonFlightMaker => dragonflightmaker_color,
        Missile::ProgressiveRadiusDamage => progressiveradiusdamage_color,
        Missile::VineBeastWalk1Fade => vinebeastwalk1fade_color,
        Missile::VineBeastWalk2Fade => vinebeastwalk2fade_color,
        Missile::VineBeastNeutralFade => vinebeastneutralfade_color,
        Missile::VineRecyclerDelay => vinerecyclerdelay_color,
        Missile::AncientDeathCenter => ancientdeathcenter_color,
        Missile::AncientDeathCloud => ancientdeathcloud_color,
        Missile::LightningChargeUp => lightningchargeup_color,
        Missile::ChargeUpNova => chargeupnova_color,
        Missile::ChainLightningChargeUp => chainlightningchargeup_color,
        Missile::PainWormAppear => painwormappear_color,
        Missile::BaalTauntControl => baaltauntcontrol_color,
        Missile::BaalTauntLightning => baaltauntlightning_color,
        Missile::BaalTauntLightningTrail => baaltauntlightningtrail_color,
        Missile::BaalTauntPoison => baaltauntpoison_color,
        Missile::BaalSpawnMonsters => baalspawnmonsters_color,
        Missile::MindBlastHit => mindblasthit_color,
        Missile::BladeShieldMissile => bladeshieldmissile_color,
        Missile::BladeShieldAttachment => bladeshieldattachment_color,
        Missile::BaalInferno => baalinferno_color,
        Missile::BaalNova => baalnova_color,
        Missile::FistsOfFireExplode => fistsoffireexplode_color,
        Missile::FistsOfFireFirewall => fistsoffirefirewall_color,
        Missile::ClawsOfThunderBolt => clawsofthunderbolt_color,
        Missile::ClawsOfThunderNova => clawsofthundernova_color,
        Missile::BladesOfIceExplode => bladesoficeexplode_color,
        Missile::BladesOfIceCubes => bladesoficecubes_color,
        Missile::BladesOfIceCubesMelt => bladesoficecubesmelt_color,
        Missile::RoyalStrikeMeteor => royalstrikemeteor_color,
        Missile::RoyalStrikeMeteorCenter => royalstrikemeteorcenter_color,
        Missile::RoyalStrikeMeteorTail => royalstrikemeteortail_color,
        Missile::RoyalStrikeMeteorExplode => royalstrikemeteorexplode_color,
        Missile::RoyalStrikeMeteorFire => royalstrikemeteorfire_color,
        Missile::RoyalStrikeChainLightning => royalstrikechainlightning_color,
        Missile::RoyalStrikeChaosIce => royalstrikechaosice_color,
        Missile::WorldStoneChip1 => worldstonechip1_color,
        Missile::WorldStoneChip2 => worldstonechip2_color,
        Missile::WorldStoneChip3 => worldstonechip3_color,
        Missile::WorldStoneChip4 => worldstonechip4_color,
        Missile::HighPriestLightning2 => highpriestlightning2_color,
        Missile::InfernoFlame3 => infernoflame3_color,
        Missile::MindBlastCenter => mindblastcenter_color,
        Missile::ArmageddonControl => armageddoncontrol_color,
        Missile::ArmageddonRock => armageddonrock_color,
        Missile::ArmageddonTail => armageddontail_color,
        Missile::ArmageddonExplosion => armageddonexplosion_color,
        Missile::HurricaneSwoosh => hurricaneswoosh_color,
        Missile::HurricaneCart => hurricanecart_color,
        Missile::HurricaneRock => hurricanerock_color,
        Missile::HurricaneSack => hurricanesack_color,
        Missile::HurricaneTree => hurricanetree_color,
        Missile::HurricaneVase => hurricanevase_color,
        Missile::BaalCorpseExplodeDelay => baalcorpseexplodedelay_color,
        Missile::BaalCorpseExplodeExpl => baalcorpseexplodeexpl_color,
        Missile::BaalColdMaker => baalcoldmaker_color,
        Missile::BaalColdTrail => baalcoldtrail_color,
        Missile::BaalSpawnMonstersExp => baalspawnmonstersexp_color,
        Missile::ImpMiss21 => impmiss21_color,
        Missile::ImpMiss22 => impmiss22_color,
        Missile::ImpMiss23 => impmiss23_color,
        Missile::ImpMiss24 => impmiss24_color,
        Missile::ImpMiss25 => impmiss25_color,
        Missile::AnyaSteam1 => anyasteam1_color,
        Missile::AnyaSteam2 => anyasteam2_color,
        Missile::AncientsGuide => ancientsguide_color,
        Missile::AncientsMarker => ancientsmarker_color,
        Missile::AncientsControl => ancientscontrol_color,
        Missile::OverseerControl => overseercontrol_color,
        Missile::Nihlithak1 => nihlithak1_color,
        Missile::Nihlithak2 => nihlithak2_color,
        Missile::Nihlithak3 => nihlithak3_color,
        Missile::NihlathakControl => nihlathakcontrol_color,
        Missile::NihlathakSwoosh => nihlathakswoosh_color,
        Missile::NihlathakDebris1 => nihlathakdebris1_color,
        Missile::NihlathakDebris2 => nihlathakdebris2_color,
        Missile::NihlathakDebris3 => nihlathakdebris3_color,
        Missile::NihlathakDebris4 => nihlathakdebris4_color,
        Missile::NihlathakGlow => nihlathakglow_color,
        Missile::BaalTeleport => baalteleport_color,
        Missile::BaalCloneDeath => baalclonedeath_color,
        Missile::AnyaSteamVent => anyasteamvent_color,
        Missile::AnyaSteam => anyasteam_color,
        Missile::NihlathakHole => nihlathakhole_color,
        Missile::NihlathakHoleLight => nihlathakholelight_color,
        Missile::VolcanoFireTrail => volcanofiretrail_color,
        Missile::NihlathakGlow2 => nihlathakglow2_color,
        Missile::NihlathakBoneChips => nihlathakbonechips_color,
        Missile::BaalCorpseExplodeFade => baalcorpseexplodefade_color,
        Missile::ArmageddonFire => armageddonfire_color,
        Missile::IceSparkle => icesparkle_color,
        Missile::BaalFxControl => baalfxcontrol_color,
        Missile::BaalFxSpirit1 => baalfxspirit1_color,
        Missile::BaalFxSpirit2 => baalfxspirit2_color,
        Missile::BaalFxSpirit3 => baalfxspirit3_color,
        Missile::BaalFxSpirit4 => baalfxspirit4_color,
        Missile::BaalFxSpirit5 => baalfxspirit5_color,
        Missile::BaalFxBaalHeadAppear => baalfxbaalheadappear_color,
        Missile::BaalFxBaalHead1 => baalfxbaalhead1_color,
        Missile::BaalFxBaalHead2 => baalfxbaalhead2_color,
        Missile::BaalFxBaalHead3 => baalfxbaalhead3_color,
        Missile::BaalFxTyrealDebris1 => baalfxtyrealdebris1_color,
        Missile::BaalFxTyrealDebris2 => baalfxtyrealdebris2_color,
        Missile::BaalFxTyrealDebris3 => baalfxtyrealdebris3_color,
        Missile::BaalFxTyrealDebrisBreak => baalfxtyrealdebrisbreak_color,
        Missile::WorldstoneShake => worldstoneshake_color,
        Missile::BlessedHammerEx => blessedhammerex_color,
        Missile::SentryLightningBolt2 => sentrylightningbolt2_color,
        Missile::SentryLightningHit2 => sentrylightninghit2_color,
        Missile::LightningTowerNova => lightningtowernova_color,
        Missile::SkBowArrow6 => skbowarrow6_color,
        Missile::SkBowArrow7 => skbowarrow7_color,
        Missile::SkBowArrow8 => skbowarrow8_color,
        Missile::Bighead6 => bighead6_color,
        Missile::ViperPoisJav => viperpoisjav_color,
        Missile::ViperPoisJavCloud => viperpoisjavcloud_color,
        Missile::ViperFire => viperfire_color,
        Missile::ViperFireCloud => viperfirecloud_color,
        Missile::ViperBoneSpear => viperbonespear_color,
        Missile::CountessFirewallMaker => countessfirewallmaker_color,
        Missile::BaalTauntLightningControl => baaltauntlightningcontrol_color,
        Missile::BaalTauntPoisonControl => baaltauntpoisoncontrol_color,
        Missile::ExplodingArrowExp2 => explodingarrowexp2_color,
        Missile::FreezingArrowExp3 => freezingarrowexp3_color,
        Missile::PantherJav5 => pantherjav5_color,
        Missile::Spike6 => spike6_color,
        Missile::CrArrow6 => crarrow6_color,
        Missile::SkMagePois => skmagepois_color,
        Missile::SkMageCold => skmagecold_color,
        Missile::SkMageFire => skmagefire_color,
        Missile::SkMageLtng => skmageltng_color,
        Missile::SuccubusMiss => succubusmiss_color,
        Missile::WillowisplightningBolt2 => willowisplightningbolt2_color,
        Missile::MummyEx => mummyex_color,
        Missile::GoospitEx => goospitex_color,
        Missile::ImpMissEx => impmissex_color,
        Missile::DiablogeddonControl => diablogeddoncontrol_color,
        Missile::DiablogeddonRock => diablogeddonrock_color,
        Missile::DiablogeddonTail => diablogeddontail_color,
        Missile::DiablogeddonExplosion => diablogeddonexplosion_color,
        Missile::DiablogeddonFire => diablogeddonfire_color,
        Missile::MegademonInferno => megademoninferno_color,
        Missile::TrapFireBolt => trapfirebolt_color,
        Missile::TrapPoisonJavCloud => trappoisonjavcloud_color,
        Missile::TrapNova => trapnova_color,
        Missile::MephFrostNova => mephfrostnova_color,
        Missile::MephLight => mephlight_color,
        Missile::VampireMeteorFire => vampiremeteorfire_color,
        Missile::StrafeArrow => strafearrow_color,
        Missile::StrafeBolt => strafebolt_color,
        Missile::RecklessAttacksmissile => recklessattacksmissile_color,
        Missile::LeapAttack => leapattack_color,
        Missile::SigilLethargyMedium => sigillethargymedium_color,
        Missile::SigilRancorMedium => sigilrancormedium_color,
        Missile::SigilDeathMedium => sigildeathmedium_color,
        Missile::RingOfFire => ringoffire_color,
        Missile::FlameWave => flamewave_color,
        Missile::MindWallMaker => mindwallmaker_color,
        Missile::BladeWarpExplode => bladewarpexplode_color,
        Missile::MiasmaBoltDot => miasmaboltdot_color,
        Missile::AbyssalShatterShards => abyssalshattershards_color,
        Missile::FlameWaveFire => flamewavefire_color,
        Missile::MiasmaChainsCloud => miasmachainscloud_color,
        Missile::MiasmaBolt => miasmabolt_color,
        Missile::MiasmaPuff => miasmapuff_color,
        Missile::SigilLethargySmall => sigillethargysmall_color,
        Missile::SigilRancorSmall => sigilrancorsmall_color,
        Missile::SigilDeathSmall => sigildeathsmall_color,
        Missile::SigilLethargyLarge => sigillethargylarge_color,
        Missile::SigilRancorLarge => sigilrancorlarge_color,
        Missile::SigilDeathLarge => sigildeathlarge_color,
        Missile::MiasmaBoltCloud => miasmaboltcloud_color,
        Missile::EchoingStrike => echoingstrike_color,
        Missile::BladeWarp => bladewarp_color,
        Missile::FlameWaveUnveiling => flamewaveunveiling_color,
        Missile::AbyssCenter => abysscenter_color,
        Missile::Engorge => engorge_color,
        Missile::MiasmaChainsMaker => miasmachainsmaker_color,
        Missile::MiasmaChains => miasmachains_color,
        Missile::MiasmaChainsCloudMaker => miasmachainscloudmaker_color,
        Missile::Abyss => abyss_color,
        Missile::AbyssExplode => abyssexplode_color,
        Missile::FlameWaveLingerFire => flamewavelingerfire_color,
        Missile::HexPurgeExplode => hexpurgeexplode_color,
        Missile::EngorgeCorpseEffect => engorgecorpseeffect_color,
        Missile::BloodBoilExplode => bloodboilexplode_color,
        Missile::EchoingStrikeDetonate => echoingstrikedetonate_color,
        Missile::EldritchBlastNova => eldritchblastnova_color,
        Missile::Apocalypse => apocalypse_color,
        Missile::BindDemonChannelMaker => binddemonchannelmaker_color,
        Missile::BindDemonChannel => binddemonchannel_color,
        Missile::ColdFissureCenter => coldfissurecenter_color,
        Missile::IceCrack1 => icecrack1_color,
        Missile::IceCrack2 => icecrack2_color,
        Missile::IceVapor1 => icevapor1_color,
        Missile::IceVapor2 => icevapor2_color,
        Missile::RingOfFireExplode => ringoffireexplode_color,
        Missile::ColossalThrowingAxe => colossalthrowingaxe_color,
        Missile::FireTwister => firetwister_color,
        Missile::TaintedFireBolt => taintedfirebolt_color,
        Missile::TaintedFireBall => taintedfireball_color,
        Missile::ColossalChargedBolt => colossalchargedbolt_color,
        Missile::Unknown => unknown_color,
    }
}

fn get_missile_type(txt_file_no: &Missile) -> MissileType {
    match txt_file_no {
        Missile::Arrow => MissileType::Physical,
        Missile::Javelin => MissileType::Physical,
        Missile::Bighead1 => MissileType::Lightning,
        Missile::Bighead2 => MissileType::Lightning,
        Missile::Bighead3 => MissileType::Lightning,
        Missile::Bighead4 => MissileType::Lightning,
        Missile::Bighead5 => MissileType::Lightning,
        Missile::Spike1 => MissileType::Physical,
        Missile::Spike2 => MissileType::Physical,
        Missile::Spike3 => MissileType::Physical,
        Missile::Spike4 => MissileType::Physical,
        Missile::Spike5 => MissileType::Physical,
        Missile::Firearrow => MissileType::Fire,
        Missile::CrArrow1 => MissileType::Physical,
        Missile::CrArrow2 => MissileType::Physical,
        Missile::CrArrow3 => MissileType::Physical,
        Missile::CrArrow4 => MissileType::Physical,
        Missile::CrArrow5 => MissileType::Physical,
        Missile::Blood1 => MissileType::Sfx,
        Missile::Blood2 => MissileType::Sfx,
        Missile::Bigblood1 => MissileType::Sfx,
        Missile::Bigblood2 => MissileType::Sfx,
        Missile::Shafire1 => MissileType::Fire,
        Missile::Shafire2 => MissileType::Fire,
        Missile::Shafire3 => MissileType::Fire,
        Missile::Shafire4 => MissileType::Fire,
        Missile::Shafire5 => MissileType::Fire,
        Missile::MagicArrow => MissileType::Magic,
        Missile::IceArrow => MissileType::Ice,
        Missile::FireExplode => MissileType::Fire,
        Missile::IceExplode => MissileType::Ice,
        Missile::Bolt => MissileType::Physical,
        Missile::AndarielsSpray => MissileType::Poison,
        Missile::BigheadExp => MissileType::Sfx,
        Missile::ShamanExp => MissileType::Fire,
        Missile::ThrowAxe => MissileType::Physical,
        Missile::ThrowKnife => MissileType::Physical,
        Missile::Glaive => MissileType::Physical,
        Missile::PoisonJav => MissileType::Poison,
        Missile::PoisonJavCloud => MissileType::Poison,
        Missile::ColdArrow => MissileType::Ice,
        Missile::ExplodingArrow => MissileType::Fire,
        Missile::ExplodingArrowExp => MissileType::Fire,
        Missile::PlagueJavelin => MissileType::Poison,
        Missile::OilPotion => MissileType::Fire,
        Missile::ExplosivePotion => MissileType::Fire,
        Missile::FulminatingPotion => MissileType::Fire,
        Missile::RancidGasePotion => MissileType::Poison,
        Missile::ChokingGasPoition => MissileType::Poison,
        Missile::StranglingGasPotion => MissileType::Poison,
        Missile::Notused50 => MissileType::FxTrigger,
        Missile::ExplosivePotionExp => MissileType::Fire,
        Missile::ExplosivePotionDebris1 => MissileType::Fire,
        Missile::ExplosivePotionDebris2 => MissileType::Fire,
        Missile::ExplosivePotionDebris3 => MissileType::Fire,
        Missile::HolyBolt => MissileType::Magic,
        Missile::ChargedBolt => MissileType::Lightning,
        Missile::SanctuaryBolt => MissileType::Lightning,
        Missile::FireBolt => MissileType::Fire,
        Missile::IceBolt => MissileType::Ice,
        Missile::InfernoFlame1 => MissileType::Fire,
        Missile::InfernoFlame2 => MissileType::Fire,
        Missile::FireBall => MissileType::Fire,
        Missile::Mummy1 => MissileType::Poison,
        Missile::Mummy2 => MissileType::Poison,
        Missile::Mummy3 => MissileType::Poison,
        Missile::Mummy4 => MissileType::Poison,
        Missile::Blaze => MissileType::Fire,
        Missile::FireWallMaker => MissileType::Fire,
        Missile::FireWall => MissileType::Fire,
        Missile::Goospit1 => MissileType::Poison,
        Missile::Goospit2 => MissileType::Poison,
        Missile::Goospit3 => MissileType::Poison,
        Missile::Goospit4 => MissileType::Poison,
        Missile::Goospit5 => MissileType::Poison,
        Missile::Goosplat => MissileType::Poison,
        Missile::Sand => MissileType::Sfx,
        Missile::SandPile => MissileType::Sfx,
        Missile::UnholyBolt1 => MissileType::Magic,
        Missile::UnholyBolt2 => MissileType::Magic,
        Missile::UnholyBolt3 => MissileType::Magic,
        Missile::UnholyBolt4 => MissileType::Magic,
        Missile::SanctuaryCenter => MissileType::Magic,
        Missile::FireExplosion => MissileType::Fire,
        Missile::StuckArrow => MissileType::Sfx,
        Missile::Footprint => MissileType::Sfx,
        Missile::ImmolationArrow => MissileType::Fire,
        Missile::GuidedArrow => MissileType::Magic,
        Missile::FreezingArrow => MissileType::Ice,
        Missile::FreezingArrowExp1 => MissileType::Ice,
        Missile::FreezingArrowExp2 => MissileType::Ice,
        Missile::Nova => MissileType::Lightning,
        Missile::IceBlast => MissileType::Ice,
        Missile::BlessedHammer => MissileType::Magic,
        Missile::ChainLightning => MissileType::Lightning,
        Missile::FistOfAres => MissileType::FxTrigger,
        Missile::ChillBlood => MissileType::Sfx,
        Missile::GlacialSpike => MissileType::Ice,
        Missile::Teleport => MissileType::Magic,
        Missile::LightningBolt => MissileType::Lightning,
        Missile::LightningHit => MissileType::Lightning,
        Missile::Meteor => MissileType::Fire,
        Missile::MeteorCenter => MissileType::Fire,
        Missile::MeteorTail => MissileType::Fire,
        Missile::MeteorExplode => MissileType::Fire,
        Missile::FireSmall => MissileType::Fire,
        Missile::FireMedium => MissileType::Fire,
        Missile::MonBlizCenter => MissileType::Ice,
        Missile::MonBliz1 => MissileType::Ice,
        Missile::MonBliz2 => MissileType::Ice,
        Missile::MonBliz3 => MissileType::Ice,
        Missile::MonBliz4 => MissileType::Ice,
        Missile::MonBlizExplode1 => MissileType::Ice,
        Missile::MonBlizExplode2 => MissileType::Ice,
        Missile::MonBlizExplode3 => MissileType::Ice,
        Missile::Teeth => MissileType::Magic,
        Missile::CorpseExplosion => MissileType::Physical,
        Missile::PoisonCorpseExplosion => MissileType::Poison,
        Missile::MonsterCorpseExplode => MissileType::Physical,
        Missile::PoisonNova => MissileType::Poison,
        Missile::FrostNova => MissileType::Ice,
        Missile::Rogue1 => MissileType::Physical,
        Missile::Rogue2 => MissileType::Fire,
        Missile::Rogue3 => MissileType::Ice,
        Missile::BatLightningBolt => MissileType::Lightning,
        Missile::BatLightningTrail => MissileType::Lightning,
        Missile::SkMage1 => MissileType::Physical,
        Missile::SkMage2 => MissileType::Physical,
        Missile::SkMage3 => MissileType::Physical,
        Missile::SkMage4 => MissileType::Physical,
        Missile::VampireFireball => MissileType::Fire,
        Missile::VampireFirewallMaker => MissileType::Fire,
        Missile::VampireFirewall => MissileType::Fire,
        Missile::VampireMeteor => MissileType::Fire,
        Missile::VampireMeteorCenter => MissileType::Fire,
        Missile::VampireMeteorExp => MissileType::Fire,
        Missile::Raven1 => MissileType::Physical,

        Missile::AmphibianGoo1 => MissileType::Fire,
        Missile::AmphibianGoo2 => MissileType::Fire,
        Missile::TentacleGoo => MissileType::Poison,
        Missile::AmphibianExplode => MissileType::Sfx,
        Missile::PoisonPuff => MissileType::Poison,
        Missile::CurseEffectRed => MissileType::Magic,
        Missile::SpiderGooLay => MissileType::FxTrigger,
        Missile::FetishInferno1 => MissileType::Fire,
        Missile::FetishInferno2 => MissileType::Fire,
        Missile::SpiderGoo => MissileType::Poison,
        Missile::CurseCast => MissileType::Magic,
        Missile::Howl => MissileType::Magic,
        Missile::Shout => MissileType::Magic,
        Missile::Dust => MissileType::Sfx,
        Missile::RedlightMissile => MissileType::Fire,
        Missile::GreenlightMissile => MissileType::Poison,
        Missile::BluelightMissile => MissileType::Ice,
        Missile::WhitelightMissile => MissileType::Physical,
        Missile::CorpsePoisonCloud => MissileType::Poison,
        Missile::ChillBloodCloud => MissileType::Sfx,
        Missile::ChillBloodPuff => MissileType::Sfx,
        Missile::BlizzardCenter => MissileType::Ice,
        Missile::Blizzard1 => MissileType::Ice,
        Missile::Blizzard2 => MissileType::Ice,
        Missile::Blizzard3 => MissileType::Ice,
        Missile::Blizzard4 => MissileType::Ice,
        Missile::BlizzardExplode1 => MissileType::Ice,
        Missile::BlizzardExplode2 => MissileType::Ice,
        Missile::BlizzardExplode3 => MissileType::Ice,
        Missile::ThunderStorm1 => MissileType::Lightning,
        Missile::ThunderStorm2 => MissileType::Lightning,
        Missile::ThunderStorm3 => MissileType::Lightning,
        Missile::ThunderStorm4 => MissileType::Lightning,
        Missile::MonsterLight => MissileType::Lightning,
        Missile::PoisonBall => MissileType::Poison,
        Missile::DiabLight => MissileType::Lightning,
        Missile::Redemption => MissileType::Magic,
        Missile::RedemptionFail => MissileType::Sfx,
        Missile::HandOfGod => MissileType::Lightning,
        Missile::DiabFire => MissileType::Fire,
        Missile::FingerMageSpider => MissileType::Fire,
        Missile::Electric => MissileType::Lightning,
        Missile::DiabThrowAxe => MissileType::Physical,
        Missile::DiabWallMaker => MissileType::Fire,
        Missile::DiabWall => MissileType::Fire,
        Missile::CurseAmplifyDamage => MissileType::Magic,
        Missile::CurseDimVision => MissileType::Magic,
        Missile::CurseWeaken => MissileType::Magic,
        Missile::CurseIronMaiden => MissileType::Magic,
        Missile::CurseTerror => MissileType::Magic,
        Missile::CurseAttract => MissileType::Magic,
        Missile::CurseReverseVampire => MissileType::Magic,
        Missile::CurseConfuse => MissileType::Magic,
        Missile::CurseDecrepify => MissileType::Magic,
        Missile::CurseLowerResist => MissileType::Magic,
        Missile::CurseCenter => MissileType::Magic,
        Missile::BoneSpear => MissileType::Magic,
        Missile::BoneSpirit => MissileType::Magic,
        Missile::ColdUnique => MissileType::Ice,
        Missile::LightUnique => MissileType::Lightning,
        Missile::SkBowArrow1 => MissileType::Physical,
        Missile::SkBowArrow2 => MissileType::Physical,
        Missile::SkBowArrow3 => MissileType::Physical,
        Missile::SkBowArrow4 => MissileType::Physical,
        Missile::SkBowArrow5 => MissileType::Physical,
        Missile::Nova1 => MissileType::Lightning,
        Missile::Nova2 => MissileType::Lightning,
        Missile::AndyPoisonBolt => MissileType::Poison,
        Missile::TeethExplode => MissileType::Magic,
        Missile::LightningJavelin => MissileType::Lightning,
        Missile::LightningFury => MissileType::Lightning,
        Missile::BoneWallMaker => MissileType::Magic,
        Missile::Necromage1 => MissileType::Poison,
        Missile::Necromage2 => MissileType::Ice,
        Missile::Necromage3 => MissileType::Fire,
        Missile::Necromage4 => MissileType::Lightning,
        Missile::Sparkle => MissileType::Poison,
        Missile::MultipleShotArrow => MissileType::Physical,
        Missile::MultipleShotBolt => MissileType::Physical,
        Missile::ChargedStrikeBolt => MissileType::Lightning,
        Missile::BoneSpearExplode => MissileType::Magic,
        Missile::PoisonExplosionCloud => MissileType::Poison,
        Missile::BoneCast => MissileType::Magic,
        Missile::BattleCry => MissileType::Magic,
        Missile::PrimePoisonCloud => MissileType::Poison,
        Missile::PlagueJavCloud => MissileType::Poison,
        Missile::RancidGasCloud => MissileType::Poison,
        Missile::ChokingGasCloud => MissileType::Poison,
        Missile::StranglingGasCloud => MissileType::Poison,
        Missile::BugLightning => MissileType::Lightning,
        Missile::PantherJav1 => MissileType::Physical,
        Missile::PantherJav2 => MissileType::Physical,
        Missile::PantherJav3 => MissileType::Physical,
        Missile::PantherJav4 => MissileType::Physical,
        Missile::ImmolationFire => MissileType::Fire,
        Missile::FuryLightning => MissileType::Lightning,
        Missile::LightningStrike => MissileType::Lightning,
        Missile::FistOfTheHeavensDelay => MissileType::FxTrigger,
        Missile::FistOfTheHeavensBolt => MissileType::Magic,
        Missile::WarCry => MissileType::Magic,
        Missile::BattleCommand => MissileType::Magic,
        Missile::BattleOrders => MissileType::Magic,
        Missile::PantherPotOrange => MissileType::Fire,
        Missile::PantherPotGreen => MissileType::Poison,
        Missile::MeteorFire => MissileType::Fire,
        Missile::TrapSpikeRight => MissileType::Physical,
        Missile::TrapSpikeLeft => MissileType::Physical,
        Missile::TrapCursedSkullRight => MissileType::Magic,
        Missile::TrapCursedSkullLeft => MissileType::Magic,
        Missile::TrapPoisonBallRight => MissileType::Poison,
        Missile::TrapPoisonBallLeft => MissileType::Poison,
        Missile::Hydra => MissileType::Fire,
        Missile::BoneSpearTrail => MissileType::Magic,
        Missile::GrimWardSmallStart => MissileType::Sfx,
        Missile::GrimWardSmall => MissileType::Sfx,
        Missile::GrimWardSmallStop => MissileType::Sfx,
        Missile::GrimWardMediumStart => MissileType::Sfx,
        Missile::GrimWardMedium => MissileType::Sfx,
        Missile::GrimWardMediumStop => MissileType::Sfx,
        Missile::GrimWardLargeStart => MissileType::Sfx,
        Missile::GrimWardLarge => MissileType::Sfx,
        Missile::GrimWardLargeStop => MissileType::Sfx,
        Missile::ZakarumLight => MissileType::Lightning,
        Missile::GrimWardScare => MissileType::Magic,
        Missile::FrozenOrb => MissileType::Ice,
        Missile::FrozenOrbBolt => MissileType::Ice,
        Missile::FrozenOrbNova => MissileType::Ice,
        Missile::FrozenOrbExplode => MissileType::Ice,
        Missile::ChillingArmorBolt => MissileType::Ice,
        Missile::FireExplosion2 => MissileType::Fire,
        Missile::Blowgun => MissileType::Physical,
        Missile::ChainLightning2 => MissileType::Lightning,
        Missile::ReviveSmall => MissileType::Magic,
        Missile::ReviveMedium => MissileType::Magic,
        Missile::ReviveLarge => MissileType::Magic,
        Missile::MonGlacialSpike => MissileType::Ice,
        Missile::IceBreakSmall => MissileType::Ice,
        Missile::IceBreakMedium => MissileType::Ice,
        Missile::IceBreakLarge => MissileType::Ice,
        Missile::IceBreakSmoke => MissileType::Ice,
        Missile::MephistoFirehead => MissileType::Fire,
        Missile::Whilrwind => MissileType::Magic,
        Missile::ArcaneLightningBolt => MissileType::Lightning,
        Missile::FrogFire => MissileType::Fire,
        Missile::FrogCold => MissileType::Ice,
        Missile::FrogPois => MissileType::Poison,
        Missile::DesertFireball => MissileType::Fire,
        Missile::BrDeathControl => MissileType::FxTrigger,
        Missile::BrDeathLightningBolt => MissileType::Lightning,
        Missile::BrDeathLightningHit => MissileType::Lightning,
        Missile::DenOfEvilLight => MissileType::Magic,
        Missile::CairnStones => MissileType::Magic,
        Missile::CairnStonesSky => MissileType::Lightning,
        Missile::CairnStonesGround => MissileType::Magic,
        Missile::TowerMist => MissileType::Magic,
        Missile::TowerMistTrail => MissileType::Sfx,
        Missile::BrDeathSmokes1 => MissileType::Sfx,
        Missile::BrDeathSmokeNu => MissileType::Sfx,
        Missile::BrDeathSmokeDt => MissileType::Sfx,
        Missile::BrDeathSpirits1 => MissileType::Sfx,
        Missile::BrDeathSpiritNu => MissileType::Sfx,
        Missile::BrDeathSpiritDt => MissileType::Sfx,
        Missile::MephistoDeathControl => MissileType::FxTrigger,
        Missile::MephistoFirewallMaker => MissileType::Fire,
        Missile::MephistoFirewall => MissileType::Fire,
        Missile::MephistoFlyingRocksBig => MissileType::Sfx,
        Missile::MephistoExplosionBig => MissileType::Sfx,
        Missile::MephistoFlyingRocksSmall => MissileType::Sfx,
        Missile::MephistoExplosionSmall => MissileType::Sfx,
        Missile::MephistoDoNotDraw => MissileType::FxTrigger,
        Missile::AndyControl0 => MissileType::FxTrigger,
        Missile::AndyFirewallMaker => MissileType::Fire,
        Missile::AndyFirewall => MissileType::Fire,
        Missile::AndyColumnFireBase => MissileType::Fire,
        Missile::AndyColumnFire => MissileType::Fire,
        Missile::AndyFallingDebris1 => MissileType::Sfx,
        Missile::AndyFallingDebris2 => MissileType::Sfx,
        Missile::AndyFallingDebris3 => MissileType::Sfx,
        Missile::AndyDebrisExplosion1 => MissileType::Sfx,
        Missile::AndyDebrisExplosion2 => MissileType::Sfx,
        Missile::AndyDebrisExplosion3 => MissileType::Sfx,
        Missile::AndyDebrisExplosion4 => MissileType::Sfx,
        Missile::AndyDebrisExplosion5 => MissileType::Sfx,
        Missile::WillowisplightningBolt => MissileType::Lightning,
        Missile::QueenPoisonCloud => MissileType::Poison,
        Missile::Dirt => MissileType::Sfx,
        Missile::DirtPile => MissileType::Sfx,
        Missile::UndeadMissile1 => MissileType::Poison,
        Missile::UndeadMissile2 => MissileType::Fire,
        Missile::UndeadMissile3 => MissileType::Ice,
        Missile::UndeadMissile4 => MissileType::Dummy,
        Missile::BoneSpiritExplode => MissileType::Magic,
        Missile::DopplezonExplode => MissileType::Sfx,
        Missile::MonBoneSpirit => MissileType::Magic,
        Missile::TowerMistFade => MissileType::Sfx,
        Missile::CountessFirewall => MissileType::Fire,
        Missile::TowerChestSpawner => MissileType::Sfx,
        Missile::HellMeteorLaunch1 => MissileType::Sfx,
        Missile::HellMeteorLaunch2 => MissileType::Sfx,
        Missile::HellMeteorUp => MissileType::Sfx,
        Missile::HellMeteorDown => MissileType::Fire,
        Missile::HellMeteorBall => MissileType::Sfx,
        Missile::HoradricStaff => MissileType::Sfx,
        Missile::HoradricLightning => MissileType::Lightning,
        Missile::HoradricLight => MissileType::Lightning,
        Missile::RegurgitatorCorpse => MissileType::Physical,
        Missile::RegurgitatorCorpseExpl => MissileType::Physical,
        Missile::HighPriestLightning => MissileType::Lightning,
        Missile::IceBreakSmallMelt => MissileType::Ice,
        Missile::IceBreakLargeMelt => MissileType::Ice,
        Missile::LeapKnockback => MissileType::Physical,
        Missile::RadamentDeath => MissileType::Sfx,
        Missile::RadamentHandOfGod => MissileType::Sfx,
        Missile::RadamentHolyBolt => MissileType::Sfx,
        Missile::TaintedSunControl => MissileType::FxTrigger,
        Missile::TaintedSunFlash => MissileType::Lightning,
        Missile::TaintedSunBall => MissileType::Lightning,
        Missile::QueenDeathCenter => MissileType::Sfx,
        Missile::QueenDeathGlob => MissileType::Sfx,
        Missile::QueenDeathSplat1 => MissileType::Sfx,
        Missile::QueenDeathSplat2 => MissileType::Sfx,
        Missile::HealingBolt => MissileType::Magic,
        Missile::MephistoHoleDelay => MissileType::FxTrigger,
        Missile::MephistoHoleBirth => MissileType::FxTrigger,
        Missile::MephistoHoleNeutral => MissileType::FxTrigger,
        Missile::MephistoHoleDeath => MissileType::FxTrigger,
        Missile::MephistoHoleDead => MissileType::FxTrigger,
        Missile::DurielDeathControl => MissileType::FxTrigger,
        Missile::DurielDeathRock => MissileType::Sfx,
        Missile::DurielDeathDebris => MissileType::Sfx,
        Missile::DurielDeathSmoke => MissileType::Sfx,
        Missile::MephistoExplosion => MissileType::Sfx,
        Missile::OrbMist => MissileType::Sfx,
        Missile::OrbMistTrail => MissileType::Sfx,
        Missile::OrbMistFade => MissileType::Sfx,
        Missile::Pilum => MissileType::Physical,
        Missile::DiabloAppears => MissileType::FxTrigger,
        Missile::HfControl => MissileType::FxTrigger,
        Missile::HfFragment1 => MissileType::Sfx,
        Missile::HfFragment2 => MissileType::Sfx,
        Missile::HfFragment3 => MissileType::Sfx,
        Missile::HfSpirit1 => MissileType::Sfx,
        Missile::HfReserved3 => MissileType::FxTrigger,
        Missile::IzualControl => MissileType::FxTrigger,
        Missile::IzualMistLoop => MissileType::Sfx,
        Missile::IzualMistFade => MissileType::Sfx,
        Missile::IzualLightning => MissileType::Sfx,
        Missile::IzualLightningTrail => MissileType::Sfx,
        Missile::CairnStonesBolt => MissileType::Sfx,
        Missile::BombInAir => MissileType::Sfx,
        Missile::BombOnGround => MissileType::Sfx,
        Missile::BombExplosion => MissileType::Sfx,
        Missile::ShockFieldInAir => MissileType::Sfx,
        Missile::ShockFieldOnGround => MissileType::Sfx,
        Missile::ThrowingStar => MissileType::Physical,
        Missile::AcidSpray => MissileType::Poison,
        Missile::BladeCreeper => MissileType::Physical,
        Missile::Distraction => MissileType::Sfx,
        Missile::DistractionFog => MissileType::Sfx,
        Missile::DistractionPuff => MissileType::Sfx,
        Missile::DistractionStart => MissileType::FxTrigger,
        Missile::DistractionEnd => MissileType::FxTrigger,
        Missile::ImpInfernoFlame1 => MissileType::Fire,
        Missile::ImpInfernoFlame2 => MissileType::Fire,
        Missile::BaalLightningBolt => MissileType::Lightning,
        Missile::BaalLightningTrail => MissileType::Lightning,
        Missile::BaalLightningBolt2 => MissileType::Lightning,
        Missile::BaalLightningTrail2 => MissileType::Lightning,
        Missile::ImpFireball => MissileType::Fire,
        Missile::ImpFireballExplode => MissileType::Fire,
        Missile::CatapultChargedBallOn => MissileType::Lightning,
        Missile::CatapultChargedBall => MissileType::Lightning,
        Missile::CatapultChargedBallBolt => MissileType::Lightning,
        Missile::ImpSpawnMonsters => MissileType::Sfx,
        Missile::CatapultSpikeBallOn => MissileType::Physical,
        Missile::CatapultSpikeBall => MissileType::Physical,
        Missile::CatapultSpikeInAir => MissileType::Physical,
        Missile::CatapultSpikeOnGround => MissileType::Fire,
        Missile::CatapultSpikeExplosion => MissileType::Physical,
        Missile::CatapultColdBallOn => MissileType::Ice,
        Missile::CatapultColdBall => MissileType::Ice,
        Missile::CatapultColdExplosion => MissileType::Ice,
        Missile::CatapultPlagueBallOn => MissileType::Poison,
        Missile::CatapultPlagueBall => MissileType::Poison,
        Missile::CatapultPlagueCloud => MissileType::Poison,
        Missile::CatapultMeteorBallOn => MissileType::Fire,
        Missile::CatapultMeteorBall => MissileType::Fire,
        Missile::CatapultMeteorFire => MissileType::Fire,
        Missile::TowerDeath => MissileType::Sfx,
        Missile::HealingVortex => MissileType::Sfx,
        Missile::SuicideCorpseExplode => MissileType::Sfx,
        Missile::SuicideFireExplode => MissileType::Fire,
        Missile::SuicideIceExplode => MissileType::Ice,
        Missile::ExplodingJavalin => MissileType::Fire,
        Missile::ExplodingJavalinExp => MissileType::Sfx,
        Missile::LightingTrailingJavalin => MissileType::Lightning,
        Missile::LightJavalinTrail => MissileType::Lightning,
        Missile::LightJavalinExplosion => MissileType::Lightning,
        Missile::IceJavalin => MissileType::Ice,
        Missile::IceJavalinExplode => MissileType::Ice,
        Missile::PlagueJavelin2 => MissileType::Poison,
        Missile::PlagueJavlinExplode => MissileType::Poison,
        Missile::AdvLightTrailingJav => MissileType::Lightning,
        Missile::AdvLightTrailingJav2 => MissileType::Lightning,
        Missile::AdvLightJavExplode => MissileType::Lightning,
        Missile::SucFireball => MissileType::Fire,
        Missile::SucFireballExplode => MissileType::Fire,
        Missile::SucFireballTrail => MissileType::Fire,
        Missile::SucShockFieldMissile => MissileType::Physical,
        Missile::SucShockFieldMissileExp => MissileType::Physical,
        Missile::SucShockField => MissileType::Physical,
        Missile::HellfireMissile => MissileType::Fire,
        Missile::HellfireExa => MissileType::Fire,
        Missile::HellfireExb => MissileType::Fire,
        Missile::ImpChargedBolt => MissileType::Lightning,
        Missile::ImpTeleport => MissileType::Magic,
        Missile::MoltenBoulder => MissileType::Fire,
        Missile::MoltenBoulderEmerge => MissileType::Fire,
        Missile::MoltenBoulderExplode => MissileType::Fire,
        Missile::MoltenBoulderFirePath => MissileType::Fire,
        Missile::MoltenBoulderFlyingRocks => MissileType::Fire,
        Missile::Firestorm => MissileType::Fire,
        Missile::FirestormMaker => MissileType::Fire,
        Missile::ArcticBlast1 => MissileType::Ice,
        Missile::ArcticBlast2 => MissileType::Ice,
        Missile::ErruptionCenter => MissileType::Fire,
        Missile::ErruptionCrack1 => MissileType::Fire,
        Missile::ErruptionCrack2 => MissileType::Fire,
        Missile::ErruptionSmoke1 => MissileType::Fire,
        Missile::ErruptionSmoke2 => MissileType::Fire,
        Missile::VineBeastWalk1 => MissileType::Physical,
        Missile::VineBeastWalk2 => MissileType::Physical,
        Missile::VineBeastNeutral => MissileType::Physical,
        Missile::VineBeastAttack => MissileType::Physical,
        Missile::VineBeastDeath => MissileType::Sfx,
        Missile::Vines => MissileType::Physical,
        Missile::VinesTrail => MissileType::Physical,
        Missile::VinesWither => MissileType::Sfx,
        Missile::PlagueVines => MissileType::Poison,
        Missile::PlagueVinesTrail => MissileType::Poison,
        Missile::PlagueVinesWither => MissileType::Sfx,
        Missile::Twister => MissileType::Physical,
        Missile::Tornado => MissileType::Physical,
        Missile::Volcano => MissileType::Fire,
        Missile::VolcanoOverlay => MissileType::Fire,
        Missile::Fire => MissileType::Fire,
        Missile::VolcanoDebris2 => MissileType::Fire,
        Missile::VolcanoExplosion => MissileType::Fire,
        Missile::VolcanoSmallFire => MissileType::Fire,
        Missile::DragonBreathMissile => MissileType::Fire,
        Missile::LureProjectile => MissileType::Physical,
        Missile::LureCenter => MissileType::Physical,
        Missile::LureCloud => MissileType::Sfx,
        Missile::ImpMiss1 => MissileType::Fire,
        Missile::ImpMiss2 => MissileType::Fire,
        Missile::ImpMiss3 => MissileType::Fire,
        Missile::ImpMiss4 => MissileType::Fire,
        Missile::ImpMiss5 => MissileType::Fire,
        Missile::FrozenHorrorArcticBlast1 => MissileType::Ice,
        Missile::FrozenHorrorArcticBlast2 => MissileType::Ice,
        Missile::SentryChargedBolt => MissileType::Lightning,
        Missile::SentrySpikeInAir => MissileType::Physical,
        Missile::SentrySpikeOnGround => MissileType::Physical,
        Missile::RecyclerDelay => MissileType::FxTrigger,
        Missile::RecyclerVine => MissileType::Physical,
        Missile::RecyclerFade => MissileType::Physical,
        Missile::RecyclerExplosion => MissileType::Physical,
        Missile::DeathMauler => MissileType::Physical,
        Missile::DeathMaulerTrail => MissileType::Physical,
        Missile::DeathMaulerTrailFade => MissileType::Physical,
        Missile::BladeFury1 => MissileType::Physical,
        Missile::BladeFragment1 => MissileType::Physical,
        Missile::BladeFury2 => MissileType::Physical,
        Missile::BladeFragment2 => MissileType::Physical,
        Missile::BladeFury3 => MissileType::Physical,
        Missile::BladeFragment3 => MissileType::Physical,
        Missile::ShockWave => MissileType::Physical,
        Missile::LightningTalons => MissileType::Lightning,
        Missile::LightningTalonsTrail => MissileType::Lightning,
        Missile::PhoenixTrail => MissileType::Magic,
        Missile::RabiesPlague => MissileType::Poison,
        Missile::RabiesContagion => MissileType::Poison,
        Missile::WakeOfDestructionMaker => MissileType::Magic,
        Missile::WakeOfDestructionDeathSentryExplode => MissileType::Sfx,
        Missile::TigerFury => MissileType::Physical,
        Missile::TigerFuryTrail => MissileType::Fire,
        Missile::TigerFuryTrail2 => MissileType::Physical,
        Missile::InfernoSentry1 => MissileType::Fire,
        Missile::InfernoSentry2 => MissileType::Fire,
        Missile::AncientThrowingAxe => MissileType::Physical,
        Missile::SentryLightningBolt => MissileType::Lightning,
        Missile::SentryLightningHit => MissileType::Lightning,
        Missile::AnyaCenter => MissileType::Sfx,
        Missile::AnyaIcicle => MissileType::Sfx,
        Missile::AnyaIceImpact => MissileType::Sfx,
        Missile::AnyaIceSteam => MissileType::Sfx,
        Missile::AnyaIceMagic => MissileType::Magic,
        Missile::DragonTailMissile => MissileType::Physical,
        Missile::DragonFlight => MissileType::Physical,
        Missile::DragonFlightMaker => MissileType::Fire,
        Missile::ProgressiveRadiusDamage => MissileType::Sfx,
        Missile::VineBeastWalk1Fade => MissileType::Sfx,
        Missile::VineBeastWalk2Fade => MissileType::Sfx,
        Missile::VineBeastNeutralFade => MissileType::Sfx,
        Missile::VineRecyclerDelay => MissileType::FxTrigger,
        Missile::AncientDeathCenter => MissileType::Sfx,
        Missile::AncientDeathCloud => MissileType::Sfx,
        Missile::LightningChargeUp => MissileType::Lightning,
        Missile::ChargeUpNova => MissileType::Lightning,
        Missile::ChainLightningChargeUp => MissileType::Lightning,
        Missile::PainWormAppear => MissileType::Physical,
        Missile::BaalTauntControl => MissileType::FxTrigger,
        Missile::BaalTauntLightning => MissileType::Lightning,
        Missile::BaalTauntLightningTrail => MissileType::Lightning,
        Missile::BaalTauntPoison => MissileType::Poison,
        Missile::BaalSpawnMonsters => MissileType::Magic,
        Missile::MindBlastHit => MissileType::Physical,
        Missile::BladeShieldMissile => MissileType::Physical,
        Missile::BladeShieldAttachment => MissileType::Physical,
        Missile::BaalInferno => MissileType::Fire,
        Missile::BaalNova => MissileType::Lightning,
        Missile::FistsOfFireExplode => MissileType::Fire,
        Missile::FistsOfFireFirewall => MissileType::Fire,
        Missile::ClawsOfThunderBolt => MissileType::Lightning,
        Missile::ClawsOfThunderNova => MissileType::Lightning,
        Missile::BladesOfIceExplode => MissileType::Ice,
        Missile::BladesOfIceCubes => MissileType::Ice,
        Missile::BladesOfIceCubesMelt => MissileType::Ice,
        Missile::RoyalStrikeMeteor => MissileType::Fire,
        Missile::RoyalStrikeMeteorCenter => MissileType::Fire,
        Missile::RoyalStrikeMeteorTail => MissileType::Fire,
        Missile::RoyalStrikeMeteorExplode => MissileType::Fire,
        Missile::RoyalStrikeMeteorFire => MissileType::Fire,
        Missile::RoyalStrikeChainLightning => MissileType::Lightning,
        Missile::RoyalStrikeChaosIce => MissileType::Ice,
        Missile::WorldStoneChip1 => MissileType::Sfx,
        Missile::WorldStoneChip2 => MissileType::Sfx,
        Missile::WorldStoneChip3 => MissileType::Sfx,
        Missile::WorldStoneChip4 => MissileType::Sfx,
        Missile::HighPriestLightning2 => MissileType::Lightning,
        Missile::InfernoFlame3 => MissileType::Fire,
        Missile::MindBlastCenter => MissileType::Physical,
        Missile::ArmageddonControl => MissileType::Fire,
        Missile::ArmageddonRock => MissileType::Sfx,
        Missile::ArmageddonTail => MissileType::Sfx,
        Missile::ArmageddonExplosion => MissileType::Sfx,
        Missile::HurricaneSwoosh => MissileType::Ice,
        Missile::HurricaneCart => MissileType::Ice,
        Missile::HurricaneRock => MissileType::Ice,
        Missile::HurricaneSack => MissileType::Ice,
        Missile::HurricaneTree => MissileType::Ice,
        Missile::HurricaneVase => MissileType::Ice,
        Missile::BaalCorpseExplodeDelay => MissileType::Physical,
        Missile::BaalCorpseExplodeExpl => MissileType::Physical,
        Missile::BaalColdMaker => MissileType::Ice,
        Missile::BaalColdTrail => MissileType::Ice,
        Missile::BaalSpawnMonstersExp => MissileType::Magic,
        Missile::ImpMiss21 => MissileType::Magic,
        Missile::ImpMiss22 => MissileType::Magic,
        Missile::ImpMiss23 => MissileType::Magic,
        Missile::ImpMiss24 => MissileType::Magic,
        Missile::ImpMiss25 => MissileType::Magic,
        Missile::AnyaSteam1 => MissileType::Sfx,
        Missile::AnyaSteam2 => MissileType::Sfx,
        Missile::AncientsGuide => MissileType::Sfx,
        Missile::AncientsMarker => MissileType::Sfx,
        Missile::AncientsControl => MissileType::FxTrigger,
        Missile::OverseerControl => MissileType::FxTrigger,
        Missile::Nihlithak1 => MissileType::Magic,
        Missile::Nihlithak2 => MissileType::Magic,
        Missile::Nihlithak3 => MissileType::Magic,
        Missile::NihlathakControl => MissileType::FxTrigger,
        Missile::NihlathakSwoosh => MissileType::Sfx,
        Missile::NihlathakDebris1 => MissileType::Sfx,
        Missile::NihlathakDebris2 => MissileType::Sfx,
        Missile::NihlathakDebris3 => MissileType::Sfx,
        Missile::NihlathakDebris4 => MissileType::Sfx,
        Missile::NihlathakGlow => MissileType::Sfx,
        Missile::BaalTeleport => MissileType::Magic,
        Missile::BaalCloneDeath => MissileType::Magic,
        Missile::AnyaSteamVent => MissileType::Magic,
        Missile::AnyaSteam => MissileType::Magic,
        Missile::NihlathakHole => MissileType::Magic,
        Missile::NihlathakHoleLight => MissileType::Magic,
        Missile::VolcanoFireTrail => MissileType::Fire,
        Missile::NihlathakGlow2 => MissileType::Magic,
        Missile::NihlathakBoneChips => MissileType::Magic,
        Missile::BaalCorpseExplodeFade => MissileType::Magic,
        Missile::ArmageddonFire => MissileType::Fire,
        Missile::IceSparkle => MissileType::Ice,
        Missile::BaalFxControl => MissileType::Sfx,
        Missile::BaalFxSpirit1 => MissileType::Sfx,
        Missile::BaalFxSpirit2 => MissileType::Sfx,
        Missile::BaalFxSpirit3 => MissileType::Sfx,
        Missile::BaalFxSpirit4 => MissileType::Sfx,
        Missile::BaalFxSpirit5 => MissileType::Sfx,
        Missile::BaalFxBaalHeadAppear => MissileType::Sfx,
        Missile::BaalFxBaalHead1 => MissileType::Sfx,
        Missile::BaalFxBaalHead2 => MissileType::Sfx,
        Missile::BaalFxBaalHead3 => MissileType::Sfx,
        Missile::BaalFxTyrealDebris1 => MissileType::Sfx,
        Missile::BaalFxTyrealDebris2 => MissileType::Sfx,
        Missile::BaalFxTyrealDebris3 => MissileType::Sfx,
        Missile::BaalFxTyrealDebrisBreak => MissileType::Sfx,
        Missile::WorldstoneShake => MissileType::Sfx,
        Missile::BlessedHammerEx => MissileType::Magic,
        Missile::SentryLightningBolt2 => MissileType::Lightning,
        Missile::SentryLightningHit2 => MissileType::Lightning,
        Missile::LightningTowerNova => MissileType::Lightning,
        Missile::SkBowArrow6 => MissileType::Physical,
        Missile::SkBowArrow7 => MissileType::Physical,
        Missile::SkBowArrow8 => MissileType::Physical,
        Missile::Bighead6 => MissileType::Physical,
        Missile::ViperPoisJav => MissileType::Poison,
        Missile::ViperPoisJavCloud => MissileType::Poison,
        Missile::ViperFire => MissileType::Fire,
        Missile::ViperFireCloud => MissileType::Fire,
        Missile::ViperBoneSpear => MissileType::Magic,
        Missile::CountessFirewallMaker => MissileType::Fire,
        Missile::BaalTauntLightningControl => MissileType::Lightning,
        Missile::BaalTauntPoisonControl => MissileType::Poison,
        Missile::ExplodingArrowExp2 => MissileType::Fire,
        Missile::FreezingArrowExp3 => MissileType::Ice,
        Missile::PantherJav5 => MissileType::Physical,
        Missile::Spike6 => MissileType::Physical,
        Missile::CrArrow6 => MissileType::Physical,
        Missile::SkMagePois => MissileType::Poison,
        Missile::SkMageCold => MissileType::Ice,
        Missile::SkMageFire => MissileType::Fire,
        Missile::SkMageLtng => MissileType::Lightning,
        Missile::SuccubusMiss => MissileType::Magic,
        Missile::WillowisplightningBolt2 => MissileType::Lightning,
        Missile::MummyEx => MissileType::Poison,
        Missile::GoospitEx => MissileType::Poison,
        Missile::ImpMissEx => MissileType::Fire,
        Missile::DiablogeddonControl => MissileType::Fire,
        Missile::DiablogeddonRock => MissileType::Fire,
        Missile::DiablogeddonTail => MissileType::Fire,
        Missile::DiablogeddonExplosion => MissileType::Fire,
        Missile::DiablogeddonFire => MissileType::Fire,
        Missile::MegademonInferno => MissileType::Fire,
        Missile::TrapFireBolt => MissileType::Fire,
        Missile::TrapPoisonJavCloud => MissileType::Poison,
        Missile::TrapNova => MissileType::Lightning,
        Missile::MephFrostNova => MissileType::Ice,
        Missile::MephLight => MissileType::Lightning,
        Missile::VampireMeteorFire => MissileType::Fire,
        Missile::StrafeArrow => MissileType::Physical,
        Missile::StrafeBolt => MissileType::Physical,
        Missile::RecklessAttacksmissile => MissileType::Physical,
        Missile::LeapAttack => MissileType::Physical,
        Missile::SigilLethargyMedium => MissileType::Magic,
        Missile::SigilRancorMedium => MissileType::Magic,
        Missile::SigilDeathMedium => MissileType::Magic,
        Missile::RingOfFire => MissileType::Fire,
        Missile::FlameWave => MissileType::Fire,
        Missile::MindWallMaker => MissileType::Magic,
        Missile::BladeWarpExplode => MissileType::Magic,
        Missile::MiasmaBoltDot => MissileType::Poison,
        Missile::AbyssalShatterShards => MissileType::Magic,
        Missile::FlameWaveFire => MissileType::Fire,
        Missile::MiasmaChainsCloud => MissileType::Poison,
        Missile::MiasmaBolt => MissileType::Poison,
        Missile::MiasmaPuff => MissileType::Poison,
        Missile::SigilLethargySmall => MissileType::Magic,
        Missile::SigilRancorSmall => MissileType::Magic,
        Missile::SigilDeathSmall => MissileType::Magic,
        Missile::SigilLethargyLarge => MissileType::Magic,
        Missile::SigilRancorLarge => MissileType::Magic,
        Missile::SigilDeathLarge => MissileType::Magic,
        Missile::MiasmaBoltCloud => MissileType::Poison,
        Missile::EchoingStrike => MissileType::Magic,
        Missile::BladeWarp => MissileType::Magic,
        Missile::FlameWaveUnveiling => MissileType::Fire,
        Missile::AbyssCenter => MissileType::Magic,
        Missile::Engorge => MissileType::Magic,
        Missile::MiasmaChainsMaker => MissileType::Poison,
        Missile::MiasmaChains => MissileType::Poison,
        Missile::MiasmaChainsCloudMaker => MissileType::Poison,
        Missile::Abyss => MissileType::Magic,
        Missile::AbyssExplode => MissileType::Magic,
        Missile::FlameWaveLingerFire => MissileType::Fire,
        Missile::HexPurgeExplode => MissileType::Magic,
        Missile::EngorgeCorpseEffect => MissileType::Magic,
        Missile::BloodBoilExplode => MissileType::Magic,
        Missile::EchoingStrikeDetonate => MissileType::Magic,
        Missile::EldritchBlastNova => MissileType::Magic,
        Missile::Apocalypse => MissileType::Fire,
        Missile::BindDemonChannelMaker => MissileType::Magic,
        Missile::BindDemonChannel => MissileType::Magic,
        Missile::ColdFissureCenter => MissileType::Ice,
        Missile::IceCrack1 => MissileType::Ice,
        Missile::IceCrack2 => MissileType::Ice,
        Missile::IceVapor1 => MissileType::Ice,
        Missile::IceVapor2 => MissileType::Ice,
        Missile::RingOfFireExplode => MissileType::Fire,
        Missile::ColossalThrowingAxe => MissileType::Physical,
        Missile::FireTwister => MissileType::Fire,
        Missile::TaintedFireBolt => MissileType::Fire,
        Missile::TaintedFireBall => MissileType::Fire,
        Missile::ColossalChargedBolt => MissileType::Lightning,
        Missile::Unknown => MissileType::FxTrigger,
        //_=> MissileType::Dummy,
    }
}
